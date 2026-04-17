//! Writes reasoner output as a `.md` file into the Obsidian vault, and
//! maintains a root `INDEX.md` plus per-folder `_folder_index.md` so that
//! downstream readers (both humans and small models) can locate notes by
//! one-line summary without reading every file.

use std::path::{Path, PathBuf};

use chrono::Utc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tracing::{info, instrument};

use crate::agents::KgOutput;
use crate::error::AppResult;

/// Single writer per vault. The `Mutex` serializes index updates so
/// concurrent writers don't clobber each other. In practice the reasoner
/// is already serialized, but this is cheap insurance.
pub struct VaultWriter {
    vault_dir: PathBuf,
    index_lock: Mutex<()>,
}

impl VaultWriter {
    pub fn new(vault_dir: PathBuf) -> Self {
        Self {
            vault_dir,
            index_lock: Mutex::new(()),
        }
    }

    /// Write a Markdown note. `source_hint` is typically the source PDF
    /// filename (without extension); used to namespace generated notes.
    /// Returns the path written.
    #[instrument(level = "info", skip(self, output), fields(title = %output.title, source = %source_hint))]
    pub async fn write_note(
        &self,
        source_hint: &str,
        output: &KgOutput,
    ) -> AppResult<PathBuf> {
        let folder_slug = sanitize(source_hint);
        let dir = self.vault_dir.join("Generated").join(&folder_slug);
        fs::create_dir_all(&dir).await?;

        let title_slug = slugify(&output.title);
        // Namespace collisions with a short timestamp suffix.
        let ts = Utc::now().format("%Y%m%d-%H%M%S");
        let filename = format!("{title_slug}-{ts}.md");
        let path = dir.join(&filename);

        let frontmatter = build_frontmatter(output);
        let heading = format!("# {}\n\n*{}*\n", output.title, trim_summary(&output.summary));
        let body = format!("{frontmatter}\n\n{heading}\n{}\n", output.markdown_snippet);

        let mut f = fs::File::create(&path).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        // Update indexes. Failures here should not lose the note — log & continue.
        let rel_from_root = Path::new("Generated").join(&folder_slug).join(&filename);
        let rel_from_folder = PathBuf::from(&filename);
        if let Err(e) = self
            .append_index(
                &self.vault_dir.join("INDEX.md"),
                &IndexEntry {
                    title: &output.title,
                    summary: &output.summary,
                    rel_path: rel_from_root.to_string_lossy().replace('\\', "/"),
                    tags: &output.tags,
                },
                /*header=*/ "# Knowledge-Graph Index\n\n\
                             One-line summaries of every generated note. \
                             Read this first; open individual notes only when you need detail.\n",
            )
            .await
        {
            tracing::warn!(error = %e, "root index update failed");
        }
        if let Err(e) = self
            .append_index(
                &dir.join("_folder_index.md"),
                &IndexEntry {
                    title: &output.title,
                    summary: &output.summary,
                    rel_path: rel_from_folder.to_string_lossy().replace('\\', "/"),
                    tags: &output.tags,
                },
                /*header=*/ &format!("# Folder Index · {folder_slug}\n\n\
                                       Notes generated from this source document.\n"),
            )
            .await
        {
            tracing::warn!(error = %e, "folder index update failed");
        }

        info!(path = %path.display(), "note written");
        Ok(path)
    }

    /// Append a `- [[note]] — summary` line to an index file, creating the
    /// file (with header) on first use. Deduplicates on exact rel_path so
    /// replays don't double-list.
    async fn append_index(
        &self,
        index_path: &Path,
        entry: &IndexEntry<'_>,
        header: &str,
    ) -> AppResult<()> {
        let _guard = self.index_lock.lock().await;

        let existing = fs::read_to_string(index_path).await.unwrap_or_default();
        let marker = format!("({})", entry.rel_path);
        if existing.contains(&marker) {
            return Ok(()); // already indexed
        }

        let line = format_index_line(entry);
        let mut to_write = if existing.is_empty() {
            format!("{header}\n{line}\n")
        } else {
            // Ensure a trailing newline before appending.
            let mut s = existing;
            if !s.ends_with('\n') {
                s.push('\n');
            }
            s.push_str(&line);
            s.push('\n');
            s
        };
        // Normalize: collapse any accidental double-blank at EOF.
        while to_write.ends_with("\n\n\n") {
            to_write.pop();
        }

        let mut f = fs::File::create(index_path).await?;
        f.write_all(to_write.as_bytes()).await?;
        f.sync_all().await?;
        Ok(())
    }
}

struct IndexEntry<'a> {
    title: &'a str,
    summary: &'a str,
    rel_path: String,
    tags: &'a [String],
}

fn format_index_line(e: &IndexEntry<'_>) -> String {
    let tag_suffix = if e.tags.is_empty() {
        String::new()
    } else {
        let top: Vec<String> = e
            .tags
            .iter()
            .take(3)
            .map(|t| format!("#{}", t.trim_start_matches('#')))
            .collect();
        format!(" · {}", top.join(" "))
    };
    // Markdown link form keeps us portable across Obsidian and plain editors.
    format!(
        "- [{title}]({path}) — {summary}{tags}",
        title = e.title,
        path = e.rel_path,
        summary = trim_summary(e.summary),
        tags = tag_suffix
    )
}

/// Ensure the summary fits on one line — collapse whitespace and cap length.
fn trim_summary(s: &str) -> String {
    let collapsed: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    const MAX: usize = 200;
    if collapsed.chars().count() <= MAX {
        collapsed
    } else {
        let mut out: String = collapsed.chars().take(MAX - 1).collect();
        out.push('…');
        out
    }
}

fn build_frontmatter(o: &KgOutput) -> String {
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
    s.push_str(&format!("title: {}\n", yaml_scalar(&o.title)));
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

fn yaml_scalar(s: &str) -> String {
    // Quote defensively; always safe.
    format!("\"{}\"", escape_yaml(s))
}

fn escape_yaml(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

/// Produce a filesystem-safe, lowercase-kebab slug from a free-form title.
/// Keeps ASCII letters/digits/hyphens; collapses everything else to `-`.
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
    // Cap length: long filenames break on some filesystems (e.g. eCryptfs).
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
