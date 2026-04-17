//! Writes reasoner output as a `.md` file into the Obsidian vault.

use std::path::PathBuf;

use chrono::Utc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::agents::KgOutput;
use crate::error::AppResult;

pub struct VaultWriter {
    vault_dir: PathBuf,
}

impl VaultWriter {
    pub fn new(vault_dir: PathBuf) -> Self {
        Self { vault_dir }
    }

    /// Write a Markdown note. `source_hint` is typically the source PDF filename
    /// (without extension); used to namespace generated notes.
    pub async fn write_note(
        &self,
        source_hint: &str,
        output: &KgOutput,
    ) -> AppResult<PathBuf> {
        let dir = self.vault_dir.join("Generated").join(sanitize(source_hint));
        fs::create_dir_all(&dir).await?;

        let ts = Utc::now().format("%Y%m%d-%H%M%S%.3f");
        let filename = format!("{ts}.md");
        let path = dir.join(filename);

        let frontmatter = build_frontmatter(output);
        let body = format!("{frontmatter}\n\n{}\n", output.markdown_snippet);

        let mut f = fs::File::create(&path).await?;
        f.write_all(body.as_bytes()).await?;
        f.sync_all().await?;

        info!(path = %path.display(), "note written");
        Ok(path)
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

