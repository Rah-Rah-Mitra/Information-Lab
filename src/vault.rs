//! Vault writer — hierarchical, two-axis index.
//!
//! Layout:
//!
//! ```text
//! {vault}/
//!   Index.md                     type: index (root) → Sources + Topics
//!   Sources/
//!     {source}.md                type: index, one per ingested PDF source
//!   Topics/
//!     {tag}.md                   type: index, cross-source (cross-textbook)
//!   Generated/
//!     {source}/
//!       {slug}-{yyyymmdd-hhmmss}.md   type: content
//! ```
//!
//! Two-axis navigation: a reader/agent can browse by source (one textbook at
//! a time) *or* by topic (same concept across every textbook that mentions
//! it). The Topics axis is what makes cross-textbook discovery work — if
//! both GIS and Stats tag a note `kernel-density-estimation`, both end up
//! in `Topics/kernel-density-estimation.md`.
//!
//! Index files cap at `INDEX_ENTRY_CAP` entries (default 20). On overflow
//! the file is split into alphabetical buckets (`a-e`, `f-j`, `k-o`,
//! `p-t`, `u-z`, `other`) under a same-named sub-directory, and the
//! original file is replaced with a parent-of-buckets pointer index
//! (`split: true` in frontmatter). Subsequent `upsert_index_entry` calls
//! follow the pointer to the correct bucket. Buckets are stable — adding
//! an entry never renames an existing bucket file, so Obsidian wikilinks
//! stay valid. A bucket exceeding the cap is a hard error (deep-splitting
//! is not implemented; callers must surface the failure rather than silently
//! continue).

use std::path::{Path, PathBuf};

use chrono::Utc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tracing::{info, instrument, warn};

use crate::agents::bridge::BridgeNote;
use crate::agents::curator::{Formula, TopicSynthesis};
use crate::agents::derivation::DerivationChainNote;
use crate::agents::report::DailyReport;
use crate::agents::research::ResearchResult;
use crate::agents::theorem::TheoremNote;
use crate::agents::KgOutput;
use crate::error::{AppError, AppResult};

/// One writer per vault. The mutex serialises index updates so concurrent
/// writers can't clobber an in-progress append. The reasoner is already
/// serialised upstream, but this is cheap insurance.
pub struct VaultWriter {
    vault_dir: PathBuf,
    index_entry_cap: usize,
    index_lock: Mutex<()>,
}

impl VaultWriter {
    pub fn new(vault_dir: PathBuf, index_entry_cap: usize) -> Self {
        Self {
            vault_dir,
            index_entry_cap,
            index_lock: Mutex::new(()),
        }
    }

    /// Absolute path of the vault root. Used by the research drainer to
    /// resolve `rel_path` entries from the task queue.
    pub fn vault_dir(&self) -> &Path {
        &self.vault_dir
    }

    /// Write a Markdown note under `Generated/{source}/` and update the
    /// source index plus every topic index referenced by the note's tags.
    /// Returns the absolute path of the content file.
    #[instrument(
        level = "info",
        skip(self, output),
        fields(title = %output.title, source = %source_name)
    )]
    pub async fn write_note(
        &self,
        source_name: &str,
        output: &KgOutput,
    ) -> AppResult<PathBuf> {
        let source_slug = path_component(source_name);
        let gen_dir = self.vault_dir.join("Generated").join(&source_slug);
        fs::create_dir_all(&gen_dir).await?;

        let title_slug = slugify(&output.title);
        let ts = Utc::now().format("%Y%m%d-%H%M%S");
        let filename = format!("{title_slug}-{ts}.md");
        let content_path = gen_dir.join(&filename);

        let rel_content = format!("Generated/{source_slug}/{filename}");
        let source_index_rel = format!("Sources/{source_slug}.md");

        // --- Content file ---------------------------------------------------
        let frontmatter = build_content_frontmatter(
            output,
            source_name,
            &source_index_rel,
        );
        let heading = format!(
            "# {}\n\n*{}*\n",
            output.title,
            collapse_ws(&output.summary)
        );
        let body = format!(
            "{frontmatter}\n\n{heading}\n{}\n",
            output.markdown_snippet
        );

        let mut f = fs::File::create(&content_path).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        // --- Index updates (best-effort, log & continue on failure) --------
        let _guard = self.index_lock.lock().await;

        let source_index_path = self.vault_dir.join(&source_index_rel);
        if let Err(e) = upsert_index_entry(
            &source_index_path,
            &IndexFile {
                kind: IndexKind::Source {
                    source: source_name.to_string(),
                },
                cap: self.index_entry_cap,
            },
            &IndexEntry {
                title: output.title.clone(),
                summary: output.summary.clone(),
                rel_path: rel_content.clone(),
                tags: output.tags.clone(),
            },
        )
        .await
        {
            warn!(error = %e, "source index update failed");
        }

        for tag in &output.tags {
            let tag_slug = slugify(tag);
            if tag_slug.is_empty() {
                continue;
            }
            let topic_rel = format!("Topics/{tag_slug}.md");
            let topic_path = self.vault_dir.join(&topic_rel);
            if let Err(e) = upsert_index_entry(
                &topic_path,
                &IndexFile {
                    kind: IndexKind::Topic {
                        tag: tag.clone(),
                    },
                    cap: self.index_entry_cap,
                },
                &IndexEntry {
                    title: output.title.clone(),
                    summary: output.summary.clone(),
                    rel_path: rel_content.clone(),
                    tags: output.tags.clone(),
                },
            )
            .await
            {
                warn!(error = %e, tag = %tag, "topic index update failed");
            }

            if let Err(e) = register_root_link(
                &self.vault_dir,
                RootSection::Topics,
                &topic_rel,
                tag,
            )
            .await
            {
                warn!(error = %e, "root Topics registration failed");
            }
        }

        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Sources,
            &source_index_rel,
            source_name,
        )
        .await
        {
            warn!(error = %e, "root Sources registration failed");
        }

        info!(path = %content_path.display(), "note written");
        Ok(content_path)
    }

    /// Write a cross-textbook synthesis note under `Generated/_Syntheses/`.
    /// Registers the note in each of its topic indexes and in the root
    /// `## Syntheses` section.
    #[instrument(level = "info", skip(self, synthesis),
        fields(topic = %synthesis.topic, formulas = synthesis.formulas.len()))]
    pub async fn write_synthesis(
        &self,
        synthesis: &TopicSynthesis,
    ) -> AppResult<PathBuf> {
        let dir = self.vault_dir.join("Generated").join("_Syntheses");
        fs::create_dir_all(&dir).await?;
        let ts = Utc::now().format("%Y%m%d-%H%M%S");
        let slug = slugify(&synthesis.topic);
        let filename = format!("{slug}-{ts}.md");
        let rel = format!("Generated/_Syntheses/{filename}");
        let abs = dir.join(&filename);

        let frontmatter = build_synthesis_frontmatter(synthesis);
        let heading = format!(
            "# Synthesis: {}\n\n*{}*\n",
            synthesis.topic,
            collapse_ws(&synthesis.summary)
        );
        let formulas_block = render_formulas_block(&synthesis.formulas);
        let citations_block = render_citations_block(&synthesis.citations);
        let body = format!(
            "{frontmatter}\n\n{heading}\n{body}\n\n{formulas_block}{citations_block}",
            body = synthesis.markdown_body,
        );
        let mut f = fs::File::create(&abs).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        let _guard = self.index_lock.lock().await;
        for concept in synthesis.key_concepts.iter().chain(std::iter::once(&synthesis.topic)) {
            let tag_slug = slugify(concept);
            if tag_slug.is_empty() { continue; }
            let topic_rel = format!("Topics/{tag_slug}.md");
            let topic_path = self.vault_dir.join(&topic_rel);
            if let Err(e) = upsert_index_entry(
                &topic_path,
                &IndexFile {
                    kind: IndexKind::Topic { tag: concept.clone() },
                    cap: self.index_entry_cap,
                },
                &IndexEntry {
                    title: format!("Synthesis: {}", synthesis.topic),
                    summary: synthesis.summary.clone(),
                    rel_path: rel.clone(),
                    tags: vec!["synthesis".to_string()],
                },
            ).await {
                warn!(error = %e, topic = %concept, "synthesis topic upsert failed");
            }
        }
        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Syntheses,
            &rel,
            &format!("Synthesis: {}", synthesis.topic),
        ).await {
            warn!(error = %e, "root Syntheses registration failed");
        }
        info!(path = %abs.display(), "synthesis written");
        Ok(abs)
    }

    /// Write a Bridge note under `Generated/_Bridges/`. Registers in both
    /// topic indexes and in the root `## Bridges` section.
    #[instrument(level = "info", skip(self, bridge),
        fields(a = %bridge.proposal.topic_a, b = %bridge.proposal.topic_b,
               conf = bridge.proposal.confidence, iters = bridge.iterations))]
    pub async fn write_bridge(&self, bridge: &BridgeNote) -> AppResult<PathBuf> {
        let dir = self.vault_dir.join("Generated").join("_Bridges");
        fs::create_dir_all(&dir).await?;
        let ts = Utc::now().format("%Y%m%d-%H%M%S");
        let a_slug = slugify(&bridge.proposal.topic_a);
        let b_slug = slugify(&bridge.proposal.topic_b);
        let filename = format!("{a_slug}__{b_slug}-{ts}.md");
        let rel = format!("Generated/_Bridges/{filename}");
        let abs = dir.join(&filename);

        let frontmatter = build_bridge_frontmatter(bridge);
        let body = format!("{frontmatter}\n\n{}\n", bridge.final_markdown);
        let mut f = fs::File::create(&abs).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        let _guard = self.index_lock.lock().await;
        for topic in [&bridge.proposal.topic_a, &bridge.proposal.topic_b] {
            let tag_slug = slugify(topic);
            if tag_slug.is_empty() { continue; }
            let topic_rel = format!("Topics/{tag_slug}.md");
            let topic_path = self.vault_dir.join(&topic_rel);
            if let Err(e) = upsert_index_entry(
                &topic_path,
                &IndexFile {
                    kind: IndexKind::Topic { tag: topic.clone() },
                    cap: self.index_entry_cap,
                },
                &IndexEntry {
                    title: format!("Bridge: {} ↔ {}",
                        bridge.proposal.topic_a, bridge.proposal.topic_b),
                    summary: trim_summary(&bridge.proposal.hypothesis),
                    rel_path: rel.clone(),
                    tags: vec!["bridge".to_string()],
                },
            ).await {
                warn!(error = %e, topic = %topic, "bridge topic upsert failed");
            }
        }
        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Bridges,
            &rel,
            &format!("Bridge: {} ↔ {}",
                bridge.proposal.topic_a, bridge.proposal.topic_b),
        ).await {
            warn!(error = %e, "root Bridges registration failed");
        }
        info!(path = %abs.display(), "bridge written");
        Ok(abs)
    }

    /// Rewrite `Formulas.md` at the vault root as a single pure table of
    /// every known formula. Idempotent; safe to call on every harvest tick.
    #[instrument(level = "info", skip(self, formulas), fields(n = formulas.len()))]
    pub async fn write_formulas_index(
        &self,
        formulas: &[Formula],
    ) -> AppResult<PathBuf> {
        let rel = "Formulas.md".to_string();
        let abs = self.vault_dir.join(&rel);
        let mut body = String::new();
        body.push_str("---\ntype: index\nindex_kind: formulas\n");
        body.push_str(&format!("generated: {}\n---\n\n", Utc::now().to_rfc3339()));
        body.push_str("# Formulas\n\n");
        body.push_str("| Formula | Symbols | Context | Source note |\n");
        body.push_str("|---|---|---|---|\n");
        for f in formulas {
            let symbols = f.symbols.join(", ");
            let caption = f.context_caption.replace('|', "\\|").replace('\n', " ");
            let latex = f.latex.replace('|', "\\|").replace('\n', " ");
            body.push_str(&format!(
                "| $${}$$ | {} | {} | [[{}]] |\n",
                latex, symbols, caption, f.note_rel_path
            ));
        }
        let mut fh = fs::File::create(&abs).await?;
        fh.write_all(body.as_bytes()).await?;
        fh.sync_all().await?;

        let _guard = self.index_lock.lock().await;
        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Formulas,
            &rel,
            "Formulas",
        ).await {
            warn!(error = %e, "root Formulas registration failed");
        }
        Ok(abs)
    }

    /// Write a Theorem note under `Generated/_Theorems/`. Registers under
    /// both topic indexes and under the root `## Theorems` section.
    #[instrument(level = "info", skip(self, theorem),
        fields(a = %theorem.topic_a, b = %theorem.topic_b))]
    pub async fn write_theorem(&self, theorem: &TheoremNote) -> AppResult<PathBuf> {
        let dir = self.vault_dir.join("Generated").join("_Theorems");
        fs::create_dir_all(&dir).await?;
        let ts = Utc::now().format("%Y%m%d-%H%M%S");
        let a_slug = slugify(&theorem.topic_a);
        let b_slug = slugify(&theorem.topic_b);
        let filename = format!("{a_slug}__{b_slug}-{ts}.md");
        let rel = format!("Generated/_Theorems/{filename}");
        let abs = dir.join(&filename);

        let mut fm = String::new();
        fm.push_str("---\n");
        fm.push_str("type: theorem\n");
        fm.push_str(&format!("topic_a: {}\n", yaml_scalar(&theorem.topic_a)));
        fm.push_str(&format!("topic_b: {}\n", yaml_scalar(&theorem.topic_b)));
        fm.push_str(&format!("bridge_rel_path: {}\n",
            yaml_scalar(&theorem.bridge_rel_path)));
        fm.push_str(&format!("created: {}\n", Utc::now().to_rfc3339()));
        fm.push_str("---");
        let body = format!("{fm}\n\n# {}\n\n{}\n", theorem.title, theorem.markdown_body);
        let mut f = fs::File::create(&abs).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        let _guard = self.index_lock.lock().await;
        for topic in [&theorem.topic_a, &theorem.topic_b] {
            let tag_slug = slugify(topic);
            if tag_slug.is_empty() { continue; }
            let topic_path = self.vault_dir.join(format!("Topics/{tag_slug}.md"));
            if let Err(e) = upsert_index_entry(
                &topic_path,
                &IndexFile {
                    kind: IndexKind::Topic { tag: topic.clone() },
                    cap: self.index_entry_cap,
                },
                &IndexEntry {
                    title: format!("Theorem: {}", theorem.title),
                    summary: trim_summary(&theorem.claim),
                    rel_path: rel.clone(),
                    tags: vec!["theorem".to_string()],
                },
            ).await {
                warn!(error = %e, topic = %topic, "theorem topic upsert failed");
            }
        }
        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Theorems,
            &rel,
            &format!("Theorem: {}", theorem.title),
        ).await {
            warn!(error = %e, "root Theorems registration failed");
        }
        info!(path = %abs.display(), "theorem written");
        Ok(abs)
    }

    /// Write a Derivation chain note under `Generated/_Derivations/`.
    #[instrument(level = "info", skip(self, chain), fields(title = %chain.title))]
    pub async fn write_derivation(
        &self,
        chain: &DerivationChainNote,
    ) -> AppResult<PathBuf> {
        let dir = self.vault_dir.join("Generated").join("_Derivations");
        fs::create_dir_all(&dir).await?;
        let ts = Utc::now().format("%Y%m%d-%H%M%S");
        let slug = slugify(&chain.title);
        let filename = format!("{slug}-{ts}.md");
        let rel = format!("Generated/_Derivations/{filename}");
        let abs = dir.join(&filename);

        let mut fm = String::new();
        fm.push_str("---\n");
        fm.push_str("type: derivation\n");
        fm.push_str(&format!("title: {}\n", yaml_scalar(&chain.title)));
        fm.push_str(&format!("entry_symbol: {}\n", yaml_scalar(&chain.entry_symbol)));
        fm.push_str(&format!("exit_symbol: {}\n", yaml_scalar(&chain.exit_symbol)));
        fm.push_str(&format!("steps: {}\n", chain.steps.len()));
        fm.push_str(&format!("created: {}\n", Utc::now().to_rfc3339()));
        fm.push_str("---");
        let body = format!("{fm}\n\n# {}\n\n{}\n", chain.title, chain.markdown_body);
        let mut f = fs::File::create(&abs).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        let _guard = self.index_lock.lock().await;
        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Derivations,
            &rel,
            &format!("Derivation: {}", chain.title),
        ).await {
            warn!(error = %e, "root Derivations registration failed");
        }
        info!(path = %abs.display(), "derivation written");
        Ok(abs)
    }

    /// Write a daily Report under `Generated/_Reports/{YYYY-MM-DD}.md`.
    /// Idempotent per day — overwrites if the file already exists.
    #[instrument(level = "info", skip(self, report), fields(date = %report.date))]
    pub async fn write_report(&self, report: &DailyReport) -> AppResult<PathBuf> {
        let dir = self.vault_dir.join("Generated").join("_Reports");
        fs::create_dir_all(&dir).await?;
        let filename = format!("{}.md", report.date);
        let rel = format!("Generated/_Reports/{filename}");
        let abs = dir.join(&filename);

        let mut fm = String::new();
        fm.push_str("---\n");
        fm.push_str("type: report\n");
        fm.push_str(&format!("date: {}\n", yaml_scalar(&report.date)));
        fm.push_str(&format!("headline: {}\n", yaml_scalar(&report.headline)));
        fm.push_str(&format!("sections: {}\n", report.sections.len()));
        fm.push_str(&format!("created: {}\n", Utc::now().to_rfc3339()));
        fm.push_str("---");
        let body = format!(
            "{fm}\n\n# Daily report — {}\n\n*{}*\n\n{}\n",
            report.date, report.headline, report.markdown_body
        );
        let mut f = fs::File::create(&abs).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        let _guard = self.index_lock.lock().await;
        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Reports,
            &rel,
            &format!("Report {}", report.date),
        ).await {
            warn!(error = %e, "root Reports registration failed");
        }
        info!(path = %abs.display(), "report written");
        Ok(abs)
    }

    /// Write an ad-hoc research response under `Generated/_Research/`.
    #[instrument(level = "info", skip(self, out), fields(title = %out.title))]
    pub async fn write_research(&self, out: &ResearchResult) -> AppResult<PathBuf> {
        let dir = self.vault_dir.join("Generated").join("_Research");
        fs::create_dir_all(&dir).await?;
        let ts = Utc::now().format("%Y%m%d-%H%M%S");
        let filename = format!("{}-{ts}.md", slugify(&out.title));
        let rel = format!("Generated/_Research/{filename}");
        let abs = dir.join(&filename);

        let mut fm = String::new();
        fm.push_str("---\n");
        fm.push_str("type: research\n");
        fm.push_str(&format!("title: {}\n", yaml_scalar(&out.title)));
        fm.push_str(&format!("created: {}\n", Utc::now().to_rfc3339()));
        fm.push_str("---");

        let refs = if out.references.is_empty() {
            String::new()
        } else {
            let mut s = String::from("\n## References\n");
            for r in &out.references {
                s.push_str(&format!("- {r}\n"));
            }
            s
        };
        let oq = if out.open_questions.is_empty() {
            String::new()
        } else {
            let mut s = String::from("\n## Open Questions\n");
            for q in &out.open_questions {
                s.push_str(&format!("- {q}\n"));
            }
            s
        };

        let body = format!(
            "{fm}\n\n# {}\n\n*{}*\n\n{}\n{}{}\n",
            out.title, out.summary, out.markdown_body, refs, oq
        );
        let mut f = fs::File::create(&abs).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        let _guard = self.index_lock.lock().await;
        if let Err(e) = register_root_link(
            &self.vault_dir,
            RootSection::Reports,
            &rel,
            &format!("Research: {}", out.title),
        )
        .await
        {
            warn!(error = %e, "root research registration failed");
        }
        info!(path = %abs.display(), "research written");
        Ok(abs)
    }
}

// ---------------------------------------------------------------------------
// Synthesis / bridge frontmatter + helpers
// ---------------------------------------------------------------------------

fn build_synthesis_frontmatter(s: &TopicSynthesis) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str("type: synthesis\n");
    out.push_str(&format!("topic: {}\n", yaml_scalar(&s.topic)));
    out.push_str(&format!("created: {}\n", Utc::now().to_rfc3339()));
    out.push_str(&format!("summary: {}\n", yaml_scalar(&s.summary)));
    out.push_str("sources:\n");
    for src in &s.sources {
        out.push_str(&format!("  - {}\n", yaml_scalar(src)));
    }
    out.push_str("key_concepts:\n");
    for k in &s.key_concepts {
        out.push_str(&format!("  - {}\n", yaml_scalar(k)));
    }
    out.push_str("---");
    out
}

fn build_bridge_frontmatter(b: &BridgeNote) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str("type: bridge\n");
    out.push_str(&format!("topic_a: {}\n", yaml_scalar(&b.proposal.topic_a)));
    out.push_str(&format!("topic_b: {}\n", yaml_scalar(&b.proposal.topic_b)));
    out.push_str(&format!("bridge_confidence: {:.3}\n", b.proposal.confidence));
    out.push_str(&format!("iterations: {}\n", b.iterations));
    out.push_str(&format!("created: {}\n", Utc::now().to_rfc3339()));
    out.push_str("---");
    out
}

fn render_formulas_block(fs: &[Formula]) -> String {
    if fs.is_empty() { return String::new(); }
    let mut s = String::from("\n## Formulas\n\n");
    for f in fs {
        s.push_str(&format!("- $${}$$ — {}\n", f.latex, f.context_caption));
    }
    s
}

fn render_citations_block(cs: &[crate::agents::curator::Citation]) -> String {
    if cs.is_empty() { return String::new(); }
    let mut s = String::from("\n## Citations\n\n");
    for c in cs {
        s.push_str(&format!("- [[{}]] — {}\n", c.note_rel_path, c.source));
    }
    s
}

// ---------------------------------------------------------------------------
// Index file management
// ---------------------------------------------------------------------------

enum IndexKind {
    Source { source: String },
    Topic { tag: String },
}

struct IndexFile {
    kind: IndexKind,
    cap: usize,
}

struct IndexEntry {
    title: String,
    summary: String,
    rel_path: String,
    tags: Vec<String>,
}

/// Append (or no-op if already present) a one-line entry to an index file.
/// Creates the file with a typed frontmatter header on first use. If the
/// target is already a split parent, follows the pointer to the right
/// bucket. If the append pushes a leaf over `cap`, triggers a bucket
/// split (unless the leaf is itself a bucket — we don't deep-split).
async fn upsert_index_entry(
    index_path: &Path,
    file: &IndexFile,
    entry: &IndexEntry,
) -> AppResult<()> {
    // Follow split-parent pointers to a leaf.
    let mut target = index_path.to_path_buf();
    loop {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).await?;
        }
        let existing = fs::read_to_string(&target).await.unwrap_or_default();
        if !is_split_parent(&existing) {
            break;
        }
        let stem = target
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("index")
            .to_string();
        let parent_dir = target
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        let bucket = bucket_for(&entry.title);
        target = parent_dir.join(&stem).join(format!("{bucket}.md"));
    }

    let existing = fs::read_to_string(&target).await.unwrap_or_default();
    let marker = format!("({})", entry.rel_path);
    let line = render_entry_line(entry);

    let mut body = if existing.is_empty() {
        build_index_seed(file, Some(&target))
    } else if existing.contains(&marker) {
        return Ok(()); // already indexed — idempotent
    } else {
        existing
    };

    if !body.ends_with('\n') {
        body.push('\n');
    }
    body.push_str(&line);
    body.push('\n');
    while body.ends_with("\n\n\n") {
        body.pop();
    }

    let count = body
        .lines()
        .filter(|l| l.starts_with("- [["))
        .count();

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).await?;
    }
    let mut f = fs::File::create(&target).await?;
    f.write_all(body.as_bytes()).await?;
    f.sync_all().await?;

    if count > file.cap {
        if is_bucket_leaf(&target) {
            return Err(AppError::other(format!(
                "index bucket overflow: {} has {} entries (cap {}); \
                 deep-split not implemented",
                target.display(),
                count,
                file.cap
            )));
        }
        split_index(&target, file).await?;
    }
    Ok(())
}

/// Split an overflowing leaf index into alphabetical bucket children.
/// The original file is rewritten as a split-parent pointing at the
/// buckets it now aggregates. Idempotent-safe: re-entering on an
/// already-split file is a no-op (is_split_parent short-circuits).
async fn split_index(index_path: &Path, file: &IndexFile) -> AppResult<()> {
    let existing = fs::read_to_string(index_path).await.unwrap_or_default();
    if is_split_parent(&existing) {
        return Ok(());
    }

    let entries: Vec<ParsedEntry> = existing
        .lines()
        .filter_map(parse_entry_line)
        .collect();
    if entries.is_empty() {
        return Ok(());
    }

    let stem = index_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("index")
        .to_string();
    let parent_dir = index_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let bucket_dir = parent_dir.join(&stem);
    fs::create_dir_all(&bucket_dir).await?;

    let mut buckets: std::collections::BTreeMap<&'static str, Vec<String>> =
        std::collections::BTreeMap::new();
    for e in &entries {
        buckets
            .entry(bucket_for(&e.title))
            .or_default()
            .push(e.raw.clone());
    }

    for (bucket, lines) in &buckets {
        let bucket_path = bucket_dir.join(format!("{bucket}.md"));
        let mut body = build_index_seed(file, Some(&bucket_path));
        if !body.ends_with('\n') {
            body.push('\n');
        }
        for l in lines {
            body.push_str(l);
            body.push('\n');
        }
        let mut f = fs::File::create(&bucket_path).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;
    }

    // Replace original file with a split-parent pointer index.
    let pointer = build_split_parent(file, &stem, buckets.keys().copied().collect());
    let mut f = fs::File::create(index_path).await?;
    f.write_all(pointer.as_bytes()).await?;
    f.sync_all().await?;

    info!(
        index = %index_path.display(),
        buckets = buckets.len(),
        entries = entries.len(),
        "index split into alphabetical buckets"
    );
    Ok(())
}

struct ParsedEntry {
    title: String,
    raw: String,
}

fn parse_entry_line(line: &str) -> Option<ParsedEntry> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("- [[") {
        return None;
    }
    let after = &trimmed[4..];
    let end = after.find("]]")?;
    let title = after[..end].to_string();
    Some(ParsedEntry {
        title,
        raw: line.to_string(),
    })
}

fn is_split_parent(content: &str) -> bool {
    let mut in_fm = false;
    for line in content.lines() {
        if line.trim() == "---" {
            if !in_fm {
                in_fm = true;
                continue;
            } else {
                break;
            }
        }
        if in_fm && line.trim() == "split: true" {
            return true;
        }
    }
    false
}

const BUCKETS: &[&str] = &["a-e", "f-j", "k-o", "p-t", "u-z", "other"];

fn bucket_for(title: &str) -> &'static str {
    let c = title
        .chars()
        .find(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .unwrap_or('_');
    match c {
        'a'..='e' => "a-e",
        'f'..='j' => "f-j",
        'k'..='o' => "k-o",
        'p'..='t' => "p-t",
        'u'..='z' => "u-z",
        _ => "other",
    }
}

fn is_bucket_leaf(path: &Path) -> bool {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| BUCKETS.contains(&s))
        .unwrap_or(false)
}

fn build_split_parent(file: &IndexFile, stem: &str, buckets: Vec<&'static str>) -> String {
    let (kind_label, title) = match &file.kind {
        IndexKind::Source { source } => ("source", source.clone()),
        IndexKind::Topic { tag } => ("topic", tag.clone()),
    };
    let created = Utc::now().to_rfc3339();
    let mut s = format!(
        "---\n\
         type: index\n\
         index_kind: {kind_label}\n\
         split: true\n\
         title: {yaml_title}\n\
         created: {created}\n\
         ---\n\n\
         # {title}\n\n\
         This index grew past its entry cap and was split into alphabetical \
         buckets. Open the bucket that covers the title you're looking for.\n\n\
         ## Buckets\n\n",
        kind_label = kind_label,
        yaml_title = yaml_scalar(&title),
        created = created,
        title = title,
    );
    for b in buckets {
        s.push_str(&format!(
            "- [[{stem}/{b}]] ({stem}/{b}.md)\n",
            stem = stem,
            b = b
        ));
    }
    s
}

fn build_index_seed(file: &IndexFile, path: Option<&Path>) -> String {
    let bucket_note = path
        .and_then(|p| p.file_stem().and_then(|s| s.to_str()))
        .filter(|s| BUCKETS.contains(s))
        .map(|s| format!(" (bucket `{s}`)"));
    let (kind_label, title, description) = match &file.kind {
        IndexKind::Source { source } => (
            "source",
            source.clone(),
            format!(
                "Notes generated from **{}**. Scan summaries; open a note \
                 when you need its full derivation.",
                source
            ),
        ),
        IndexKind::Topic { tag } => (
            "topic",
            tag.clone(),
            format!(
                "Notes tagged **#{tag}**, aggregated across every source in \
                 this library. This is the cross-textbook view of the topic."
            ),
        ),
    };
    let created = Utc::now().to_rfc3339();
    let header_suffix = bucket_note.as_deref().unwrap_or("");
    format!(
        "---\n\
         type: index\n\
         index_kind: {kind_label}\n\
         title: {yaml_title}\n\
         created: {created}\n\
         ---\n\n\
         # {title}{header_suffix}\n\n\
         {description}\n\n",
        yaml_title = yaml_scalar(&title),
        created = created,
        title = title,
        header_suffix = header_suffix,
        description = description,
    )
}

fn render_entry_line(entry: &IndexEntry) -> String {
    // Wikilink primary form; trailing (rel/path.md) is the dedup marker the
    // upsert relies on. Do NOT remove the parenthesised path.
    let tag_suffix = if entry.tags.is_empty() {
        String::new()
    } else {
        let top: Vec<String> = entry
            .tags
            .iter()
            .take(3)
            .map(|t| format!("#{}", t.trim_start_matches('#')))
            .collect();
        format!(" · {}", top.join(" "))
    };
    format!(
        "- [[{title}]] — {summary}{tags} ({path})",
        title = entry.title,
        summary = trim_summary(&entry.summary),
        tags = tag_suffix,
        path = entry.rel_path,
    )
}

// ---------------------------------------------------------------------------
// Root Index.md — Sources and Topics sections
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum RootSection {
    Sources,
    Topics,
    Syntheses,
    Bridges,
    Formulas,
    Theorems,
    Derivations,
    Reports,
}

/// Add a link under the given root section if not already present. The root
/// index is kept deliberately small: one bullet per source and one per
/// topic, deduped by the `(rel_path)` marker.
async fn register_root_link(
    vault_dir: &Path,
    section: RootSection,
    rel_path: &str,
    display: &str,
) -> AppResult<()> {
    let index_path = vault_dir.join("Index.md");
    let existing = fs::read_to_string(&index_path).await.unwrap_or_default();
    let seeded = if existing.is_empty() {
        build_root_seed()
    } else {
        existing
    };

    let marker = format!("({rel_path})");
    if seeded.contains(&marker) {
        return Ok(());
    }

    let header = match section {
        RootSection::Sources => "## Sources",
        RootSection::Topics => "## Topics",
        RootSection::Syntheses => "## Syntheses",
        RootSection::Bridges => "## Bridges",
        RootSection::Formulas => "## Formulas",
        RootSection::Theorems => "## Theorems",
        RootSection::Derivations => "## Derivations",
        RootSection::Reports => "## Reports",
    };
    let line = match section {
        RootSection::Sources => {
            format!("- [[{display}]] ({rel_path})")
        }
        RootSection::Topics => {
            format!("- [[{display}]] — #{display} ({rel_path})")
        }
        RootSection::Syntheses
        | RootSection::Bridges
        | RootSection::Formulas
        | RootSection::Theorems
        | RootSection::Derivations
        | RootSection::Reports => {
            format!("- [[{display}]] ({rel_path})")
        }
    };

    let body = insert_under_header(&seeded, header, &line);

    let mut f = fs::File::create(&index_path).await?;
    f.write_all(body.as_bytes()).await?;
    f.sync_all().await?;
    Ok(())
}

fn build_root_seed() -> String {
    format!(
        "---\n\
         type: index\n\
         index_kind: root\n\
         title: \"Information Lab\"\n\
         created: {created}\n\
         ---\n\n\
         # Information Lab — Index\n\n\
         Two-axis navigation. **Sources** lists every textbook ingested; \
         **Topics** is the cross-source view — one file per tag, containing \
         every note that uses it regardless of which textbook it came from.\n\n\
         ## Sources\n\n\
         ## Topics\n\n\
         ## Syntheses\n\n\
         ## Bridges\n\n\
         ## Theorems\n\n\
         ## Derivations\n\n\
         ## Reports\n\n\
         ## Formulas\n\n",
        created = Utc::now().to_rfc3339(),
    )
}

/// Insert `line` directly after the matching `## Header` section header,
/// preserving anything that comes before it and any content already under
/// later sections.
fn insert_under_header(existing: &str, header: &str, line: &str) -> String {
    if !existing.contains(header) {
        // Header missing — append a fresh section at the end.
        let mut s = existing.to_string();
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s.push('\n');
        s.push_str(header);
        s.push_str("\n\n");
        s.push_str(line);
        s.push('\n');
        return s;
    }

    let mut out = String::with_capacity(existing.len() + line.len() + 2);
    let mut inserted = false;
    let mut lines = existing.lines().peekable();
    while let Some(l) = lines.next() {
        out.push_str(l);
        out.push('\n');
        if !inserted && l.trim() == header {
            // Skip any blank line directly after the header, then insert.
            if let Some(next) = lines.peek() {
                if next.trim().is_empty() {
                    out.push_str(next);
                    out.push('\n');
                    lines.next();
                }
            }
            out.push_str(line);
            out.push('\n');
            inserted = true;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Content frontmatter
// ---------------------------------------------------------------------------

fn build_content_frontmatter(
    o: &KgOutput,
    source_name: &str,
    source_index_rel: &str,
) -> String {
    let tags = o
        .tags
        .iter()
        .map(|t| format!("  - {}", t.trim_start_matches('#')))
        .collect::<Vec<_>>()
        .join("\n");
    let entities = o
        .entities
        .iter()
        .map(|e| format!("  - \"[[{}]]\"", escape_yaml(e)))
        .collect::<Vec<_>>()
        .join("\n");

    let mut s = String::new();
    s.push_str("---\n");
    s.push_str("type: content\n");
    s.push_str(&format!("title: {}\n", yaml_scalar(&o.title)));
    s.push_str(&format!("source: {}\n", yaml_scalar(source_name)));
    s.push_str(&format!(
        "index_parent: {}\n",
        yaml_scalar(source_index_rel)
    ));
    s.push_str(&format!("created: {}\n", Utc::now().to_rfc3339()));
    s.push_str(&format!("summary: {}\n", yaml_scalar(&o.summary)));
    s.push_str("tags:\n");
    s.push_str(&tags);
    s.push_str("\nentities:\n");
    s.push_str(&entities);
    s.push_str("\nrelationships:\n");
    for r in &o.relationships {
        s.push_str(&format!(
            "  - source: {}\n    target: {}\n    description: {}\n",
            yaml_scalar(&r.source),
            yaml_scalar(&r.target),
            yaml_scalar(&r.description)
        ));
    }
    s.push_str("---");
    s
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn yaml_scalar(s: &str) -> String {
    format!("\"{}\"", escape_yaml(s))
}

fn escape_yaml(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn collapse_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn trim_summary(s: &str) -> String {
    let collapsed = collapse_ws(s);
    const MAX: usize = 200;
    if collapsed.chars().count() <= MAX {
        collapsed
    } else {
        let mut out: String = collapsed.chars().take(MAX - 1).collect();
        out.push('…');
        out
    }
}

fn path_component(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_dash = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.chars().count() > 80 {
        out = out.chars().take(80).collect();
        while out.ends_with('-') {
            out.pop();
        }
    }
    if out.is_empty() {
        out.push_str("untitled");
    }
    out
}
