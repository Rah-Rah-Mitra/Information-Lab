//! Formula harvester: scans `Generated/**.md` for `$$…$$` and `\(…\)` LaTeX
//! blocks, normalises by symbol set, and writes/updates `Formulas.md` at the
//! vault root.
//!
//! **Regex-first.** The LLM fallback is reserved for ambiguous blocks — we
//! don't pay a Gemma call just to parse a well-formed equation. For the
//! current revision this agent runs entirely offline; the fallback path is
//! stubbed behind [`HarvestResult::llm_calls`] so the orchestrator can see
//! we didn't consume quota.

use std::{collections::HashSet, path::{Path, PathBuf}};

use regex::Regex;
use tokio::fs;
use tracing::debug;
use walkdir::WalkDir;

use crate::{
    db::Db,
    error::{AppError, AppResult},
};

use super::curator::Formula;

#[derive(Debug, Clone, Default)]
pub struct HarvestResult {
    pub new_formulas: Vec<Formula>,
    pub notes_scanned: usize,
    #[allow(dead_code)]
    pub llm_calls: u32,
}

#[derive(Clone)]
pub struct FormulaHarvesterAgent {
    vault_dir: PathBuf,
    db: Db,
    display_block: Regex,
    inline_paren: Regex,
    symbol: Regex,
}

impl FormulaHarvesterAgent {
    pub fn new(vault_dir: PathBuf, db: Db) -> AppResult<Self> {
        let display_block = Regex::new(r"(?s)\$\$(.+?)\$\$")
            .map_err(|e| AppError::other(format!("regex display: {e}")))?;
        let inline_paren = Regex::new(r"(?s)\\\((.+?)\\\)")
            .map_err(|e| AppError::other(format!("regex inline: {e}")))?;
        let symbol = Regex::new(r"\\[a-zA-Z]+|[A-Za-z]")
            .map_err(|e| AppError::other(format!("regex symbol: {e}")))?;
        Ok(Self {
            vault_dir,
            db,
            display_block,
            inline_paren,
            symbol,
        })
    }

    #[tracing::instrument(level = "info", skip(self), fields(max_notes = max_notes))]
    pub async fn harvest(&self, max_notes: usize) -> AppResult<HarvestResult> {
        let root = self.vault_dir.join("Generated");
        if !root.exists() {
            return Ok(HarvestResult::default());
        }
        let paths: Vec<PathBuf> = WalkDir::new(&root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
            .take(max_notes)
            .collect();

        let mut result = HarvestResult {
            notes_scanned: paths.len(),
            ..HarvestResult::default()
        };

        for path in paths {
            let content = match fs::read_to_string(&path).await {
                Ok(c) => c,
                Err(e) => {
                    debug!(path = %path.display(), error = %e, "skip unreadable note");
                    continue;
                }
            };
            let rel = rel_path(&self.vault_dir, &path);
            for f in self.extract_formulas(&content, &rel) {
                let norm = normalise(&f.latex, &self.symbol);
                let symbols_csv = f.symbols.join(",");
                let inserted = self
                    .db
                    .upsert_formula(
                        &norm,
                        &f.latex,
                        &symbols_csv,
                        &f.context_caption,
                        &rel,
                    )
                    .await?;
                if inserted {
                    result.new_formulas.push(f);
                }
            }
        }
        debug!(new = result.new_formulas.len(), "harvest ok");
        Ok(result)
    }

    fn extract_formulas(&self, content: &str, rel_path: &str) -> Vec<Formula> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        for caps in self.display_block.captures_iter(content) {
            let latex = caps.get(1).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            if latex.is_empty() || !seen.insert(latex.clone()) { continue; }
            out.push(build_formula(latex, rel_path, &self.symbol, content, caps.get(0).unwrap().start()));
        }
        for caps in self.inline_paren.captures_iter(content) {
            let latex = caps.get(1).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            if latex.is_empty() || !seen.insert(latex.clone()) { continue; }
            out.push(build_formula(latex, rel_path, &self.symbol, content, caps.get(0).unwrap().start()));
        }
        out
    }
}

fn build_formula(latex: String, rel_path: &str, symbol_re: &Regex, content: &str, at: usize) -> Formula {
    let symbols: Vec<String> = symbol_re
        .find_iter(&latex)
        .map(|m| m.as_str().to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let context = surrounding_line(content, at);
    Formula {
        latex,
        symbols,
        context_caption: context,
        note_rel_path: rel_path.to_string(),
        derived: false,
    }
}

fn surrounding_line(content: &str, at: usize) -> String {
    let start = content[..at].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end = content[at..].find('\n').map(|i| at + i).unwrap_or(content.len());
    let line = content[start..end].trim();
    if line.len() > 120 {
        let mut cut = 120;
        while !line.is_char_boundary(cut) && cut > 0 { cut -= 1; }
        format!("{}…", &line[..cut])
    } else {
        line.to_string()
    }
}

fn normalise(latex: &str, symbol_re: &Regex) -> String {
    let mut syms: Vec<String> = symbol_re
        .find_iter(latex)
        .map(|m| m.as_str().to_string())
        .collect();
    syms.sort();
    syms.dedup();
    syms.join("|")
}

fn rel_path(root: &Path, p: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .replace('\\', "/")
}
