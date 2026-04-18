//! Agent layer.
//!
//! Decomposed into one file per role:
//!
//! * [`extractor`] — PDF chunks → KG JSON (legacy single agent, now renamed).
//!
//! The other roles (curator, bridge, harvester, search) arrive in
//! follow-up commits. [`AgentCtx`] is the shared handle they all take:
//! DB, vault, limiter, and the single LLM client.

use std::sync::Arc;

use autoagents::llm::backends::google::Google;

use crate::{db::Db, limiter::Limiter, vault::VaultWriter};

pub mod bridge;
pub mod curator;
pub mod extractor;
pub mod harvester;
pub mod search;

pub use extractor::{KgOutput, KnowledgeGraphAgent};
#[allow(unused_imports)]
pub use extractor::Relationship;

// ----------------------------------------------------------------------------
// Skill prompts — compiled into the binary; see `skills/*.md`.
// ----------------------------------------------------------------------------

pub const KG_EXTRACTOR_SKILL: &str = include_str!("../../skills/kg_extractor.md");
pub const OBSIDIAN_WRITER_SKILL: &str = include_str!("../../skills/obsidian_writer.md");

/// Shared handle threaded through every agent and workflow combinator.
///
/// `llm` is a single `Arc<Google>` built once in `main.rs` — the autoagents
/// Google backend wraps a stateless `reqwest` client, so `Arc` sharing (no
/// `Mutex`) is safe across tasks. All LLM calls must gate through
/// [`Limiter::admit`] keyed by [`crate::limiter::Role`].
#[allow(dead_code)]
#[derive(Clone)]
pub struct AgentCtx {
    pub db: Db,
    pub vault: Arc<VaultWriter>,
    pub limiter: Arc<Limiter>,
    pub llm: Arc<Google>,
}

/// Truncate a string to at most `max` bytes, respecting UTF-8 boundaries.
/// Used for embedding large LLM responses in error messages without
/// shredding a multi-byte codepoint.
#[allow(dead_code)]
pub(crate) fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let mut cut = max;
        while !s.is_char_boundary(cut) && cut > 0 {
            cut -= 1;
        }
        format!("{}…", &s[..cut])
    }
}
