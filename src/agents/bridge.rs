//! Bridge-finder: a 3-iteration loop that searches for *real* cross-source
//! links between two Topics and emits a Bridge note only if the final
//! confidence clears `cfg.bridge_confidence_tau`.
//!
//! Iteration layout (hard-coded; Loop combinator drives the cadence):
//!
//!   1. **propose**  — `BridgeProposal` from two topic summaries.
//!   2. **search**   — [`LiteratureSearchAgent`] grounds the hypothesis in
//!      real published research (one Tavily call max; if the monthly
//!      budget is spent the iter degrades to a pure LLM refine on the
//!      current proposal).
//!   3. **critique** — final refinement; emits iff
//!      `updated_confidence ≥ τ`.
//!
//! Each LLM leg goes through [`Limiter::admit(Role::Bridge)`].

use std::sync::Arc;

use autoagents::llm::{
    backends::google::Google,
    builder::LLMBuilder,
    chat::{ChatMessage, ChatProvider, StructuredOutputFormat},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, warn};

use crate::{
    config::Config,
    error::{AppError, AppResult},
    limiter::{Limiter, Role},
};

use super::{
    curator::Formula,
    search::{LiteratureSearchAgent, SearchHit},
    truncate,
};

pub const BRIDGE_FINDER_SKILL: &str = include_str!("../../skills/bridge_finder.md");
pub const BRIDGE_SEARCH_REFINE_SKILL: &str =
    include_str!("../../skills/bridge_search_refine.md");

// ----------------------------------------------------------------------------
// Output schemas
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCitation {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeProposal {
    pub topic_a: String,
    pub topic_b: String,
    pub hypothesis: String,
    pub confidence: f32,
    pub rationale: String,
    #[serde(default)]
    pub shared_formulas: Vec<Formula>,
    #[serde(default)]
    pub external_citations: Vec<ExternalCitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeCritique {
    pub keep: bool,
    pub weakness: String,
    pub suggested_refinement: String,
    pub updated_confidence: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct BridgeNote {
    pub proposal: BridgeProposal,
    pub iterations: u8,
    pub final_markdown: String,
}

/// Input handed to the bridge runner for a single pair.
#[derive(Debug, Clone)]
pub struct TopicPair {
    pub topic_a: String,
    pub topic_b: String,
    pub source_a: String,
    pub source_b: String,
    pub summary_a: String,
    pub summary_b: String,
}

fn proposal_schema() -> Value {
    json!({
        "type": "object",
        "required": ["topic_a","topic_b","hypothesis","confidence","rationale"],
        "properties": {
            "topic_a":    { "type": "string" },
            "topic_b":    { "type": "string" },
            "hypothesis": { "type": "string" },
            "confidence": { "type": "number" },
            "rationale":  { "type": "string" },
            "shared_formulas": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["latex","symbols","context_caption","note_rel_path"],
                    "properties": {
                        "latex":           { "type": "string" },
                        "symbols":         { "type": "array", "items": { "type": "string" } },
                        "context_caption": { "type": "string" },
                        "note_rel_path":   { "type": "string" },
                        "derived":         { "type": "boolean" }
                    }
                }
            },
            "external_citations": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["title","url"],
                    "properties": {
                        "title": { "type": "string" },
                        "url":   { "type": "string" }
                    }
                }
            }
        }
    })
}

fn critique_schema() -> Value {
    json!({
        "type": "object",
        "required": ["keep","weakness","suggested_refinement","updated_confidence"],
        "properties": {
            "keep":                 { "type": "boolean" },
            "weakness":             { "type": "string" },
            "suggested_refinement": { "type": "string" },
            "updated_confidence":   { "type": "number" }
        }
    })
}

// ----------------------------------------------------------------------------
// Agent
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct BridgeFinderAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    search: LiteratureSearchAgent,
    max_iters: u8,
    tau: f32,
}

impl BridgeFinderAgent {
    pub fn new(
        cfg: &Config,
        limiter: Arc<Limiter>,
        search: LiteratureSearchAgent,
    ) -> AppResult<Self> {
        let model = cfg.model_for_role(&cfg.bridge_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model)
            .temperature(0.3)
            .timeout_seconds(120)
            .build()
            .map_err(|e| AppError::other(format!("build bridge llm: {e}")))?;
        Ok(Self {
            llm,
            limiter,
            search,
            max_iters: cfg.bridge_max_iters,
            tau: cfg.bridge_confidence_tau,
        })
    }

    #[tracing::instrument(level = "info", skip(self, pair),
        fields(a = %pair.topic_a, b = %pair.topic_b))]
    pub async fn run(&self, pair: TopicPair) -> AppResult<Option<BridgeNote>> {
        let mut proposal = self.propose(&pair).await?;
        let mut iters: u8 = 1;

        // Iter 2: literature-search refine.
        if self.max_iters >= 2 && proposal.confidence < self.tau {
            iters = 2;
            let query = format!(
                "{} {} relationship",
                pair.topic_a.trim(),
                pair.topic_b.trim()
            );
            let hits = self.search.search(&query).await?;
            if !hits.hits.is_empty() {
                proposal = self.refine_with_search(&proposal, &hits.hits).await?;
            } else {
                debug!("bridge iter2: no tavily hits, skipping refine");
            }
        }

        // Iter 3: final critique.
        if self.max_iters >= 3 {
            iters = self.max_iters;
            let critique = self.critique(&proposal).await?;
            proposal.confidence = critique.updated_confidence;
            if !critique.keep {
                warn!(updated = critique.updated_confidence, "critique rejected");
                return Ok(None);
            }
        }

        if proposal.confidence < self.tau {
            debug!(conf = proposal.confidence, tau = self.tau, "below threshold");
            return Ok(None);
        }

        let final_markdown = render_bridge_markdown(&proposal, iters);
        Ok(Some(BridgeNote {
            proposal,
            iterations: iters,
            final_markdown,
        }))
    }

    #[tracing::instrument(level = "debug", skip(self, pair))]
    async fn propose(&self, pair: &TopicPair) -> AppResult<BridgeProposal> {
        let _permit = self.limiter.admit(Role::Bridge).await?;
        let prompt = format!(
            "{BRIDGE_FINDER_SKILL}\n\n\
             # Role\nPROPOSE\n\n\
             # Topic A\n**{}** (source: {})\n{}\n\n\
             # Topic B\n**{}** (source: {})\n{}\n",
            pair.topic_a, pair.source_a, pair.summary_a,
            pair.topic_b, pair.source_b, pair.summary_b,
        );
        self.call_for_proposal(&prompt, "bridge_proposal").await
    }

    #[tracing::instrument(level = "debug", skip(self, proposal, hits),
        fields(hits = hits.len()))]
    async fn refine_with_search(
        &self,
        proposal: &BridgeProposal,
        hits: &[SearchHit],
    ) -> AppResult<BridgeProposal> {
        let _permit = self.limiter.admit(Role::Bridge).await?;
        let hits_block = hits
            .iter()
            .map(|h| {
                format!(
                    "- **{}** ({})\n  url: {}\n  snippet: {}",
                    h.title, h.source_domain, h.url, h.snippet
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let current = serde_json::to_string_pretty(proposal).unwrap_or_default();
        let prompt = format!(
            "{BRIDGE_SEARCH_REFINE_SKILL}\n\n# Current proposal\n{current}\n\n# Search hits\n{hits_block}\n"
        );
        self.call_for_proposal(&prompt, "bridge_proposal_refined").await
    }

    #[tracing::instrument(level = "debug", skip(self, proposal))]
    async fn critique(&self, proposal: &BridgeProposal) -> AppResult<BridgeCritique> {
        let _permit = self.limiter.admit(Role::Bridge).await?;
        let current = serde_json::to_string_pretty(proposal).unwrap_or_default();
        let prompt = format!(
            "{BRIDGE_FINDER_SKILL}\n\n# Role\nCRITIQUE\n\n# Current proposal\n{current}\n"
        );
        let messages = vec![ChatMessage::user().content(prompt).build()];
        let schema = StructuredOutputFormat {
            name: "bridge_critique".into(),
            description: Some("Bridge critique".into()),
            schema: Some(critique_schema()),
            strict: Some(true),
        };
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("bridge critique: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty critique".into()))?;
        serde_json::from_str(&text)
            .map_err(|e| AppError::Schema(format!("parse critique: {e} :: {}", truncate(&text, 400))))
    }

    async fn call_for_proposal(
        &self,
        prompt: &str,
        schema_name: &str,
    ) -> AppResult<BridgeProposal> {
        let messages = vec![ChatMessage::user().content(prompt.to_string()).build()];
        let schema = StructuredOutputFormat {
            name: schema_name.into(),
            description: Some("Bridge proposal".into()),
            schema: Some(proposal_schema()),
            strict: Some(true),
        };
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("bridge chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty bridge response".into()))?;
        serde_json::from_str(&text)
            .map_err(|e| AppError::Schema(format!("parse proposal: {e} :: {}", truncate(&text, 400))))
    }
}

fn render_bridge_markdown(p: &BridgeProposal, iters: u8) -> String {
    let mut s = String::new();
    s.push_str(&format!("# Bridge: {} ↔ {}\n\n", p.topic_a, p.topic_b));
    s.push_str(&format!("*confidence: {:.2} · iterations: {}*\n\n", p.confidence, iters));
    s.push_str("## Hypothesis\n");
    s.push_str(&p.hypothesis);
    s.push_str("\n\n## Rationale\n");
    s.push_str(&p.rationale);
    s.push('\n');
    if !p.shared_formulas.is_empty() {
        s.push_str("\n## Shared formulas\n");
        for f in &p.shared_formulas {
            s.push_str(&format!("- $${}$$ — {}\n", f.latex, f.context_caption));
        }
    }
    if !p.external_citations.is_empty() {
        s.push_str("\n## External citations\n");
        for c in &p.external_citations {
            s.push_str(&format!("- [{}]({})\n", c.title, c.url));
        }
    }
    s
}
