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
//! stay valid. A single bucket exceeding the cap is logged as a warning
//! (deep splitting is not implemented).

use std::path::{Path, PathBuf};

use chrono::Utc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tracing::{info, instrument, warn};

use crate::agents::KgOutput;
use crate::error::AppResult;

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
            warn!(
                index = %target.display(),
                entries = count,
                cap = file.cap,
                "bucket index over cap — deep-split not implemented"
            );
        } else {
            split_index(&target, file).await?;
        }
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
    };
    let line = match section {
        RootSection::Sources => {
            format!("- [[{display}]] ({rel_path})")
        }
        RootSection::Topics => {
            format!("- [[{display}]] — #{display} ({rel_path})")
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
         ## Topics\n\n",
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
