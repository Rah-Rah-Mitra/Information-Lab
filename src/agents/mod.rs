//! Agent layer.
//!
//! Decomposed into one file per role:
//!
//! * [`extractor`] — PDF chunks → KG JSON (legacy single agent, now renamed).
//!
//! The other roles (curator, bridge, harvester, search) arrive in
//! follow-up commits. [`AgentCtx`] is the shared handle they all take:
//! DB, vault, limiter, and the single LLM client.

use std::{sync::Arc, time::Instant};

use autoagents::llm::backends::google::Google;
use opentelemetry::trace::TraceContextExt;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    db::{Db, UsageKind},
    error::AppResult,
    limiter::{Limiter, Role},
    vault::VaultWriter,
};

pub mod bridge;
pub mod curator;
pub mod derivation;
pub mod extractor;
pub mod formula_extractor;
pub mod harvester;
pub mod report;
pub mod retrier;
pub mod search;
pub mod theorem;

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

/// Cheap chars/4 token estimate. Gemini doesn't expose usage through the
/// autoagents 0.3 Google backend, so this gives the status page a useful
/// signal proportional to real usage rather than the hardcoded 0 the old
/// code was logging.
#[allow(dead_code)]
pub(crate) fn estimate_tokens(s: &str) -> i64 {
    // ceil_div(chars, 4) — counts chars, not bytes, so multi-byte scripts
    // don't over-count.
    let n = s.chars().count() as i64;
    (n + 3) / 4
}

/// Byte length summary preview used by the event log.
pub(crate) const EVENT_SUMMARY_MAX: usize = 600;

/// Strip NUL bytes, replacement characters, and chat-template tokens that
/// Gemma 4 occasionally leaks into structured output (`$ w = \0 P \0 X $`
/// was the first confirmed case). Safe to apply to any LLM-emitted text
/// before it reaches the vault or DB.
pub(crate) fn scrub_llm_text(s: &str) -> String {
    const TOKENS: &[&str] = &[
        "<start_of_turn>",
        "<end_of_turn>",
        "<|turn>",
        "<|turn|>",
        "<|channel|>",
        "<|channel>",
        "<|think|>",
        "<|think>",
        "<|tool|>",
        "<|tool>",
        "<|\"|>",
        "<|user|>",
        "<|assistant|>",
        "<|system|>",
    ];
    let mut cleaned = s.to_string();
    for tok in TOKENS {
        if cleaned.contains(tok) {
            cleaned = cleaned.replace(tok, "");
        }
    }
    let no_controls: String = cleaned
        .chars()
        .filter(|c| *c != '\u{0000}' && *c != '\u{FFFD}')
        .collect();
    restore_latex_escapes(&no_controls)
}

/// Reverse the damage when a model emits JSON with *single*-backslash
/// LaTeX commands. JSON decodes `"\frac"` to `FF+rac`, `"\nabla"` to
/// `LF+abla`, `"\bar"` to `BS+ar`, etc. — we see `rac{...}`, broken
/// lines, and stray control chars in the vault.
///
/// Control chars that never have a legitimate use in content (BEL, BS,
/// VT, FF) are restored unconditionally. Ambiguous ones (LF, CR, HT)
/// are only restored inside `$...$` / `$$...$$` math spans, where they
/// cannot legitimately appear.
pub(crate) fn restore_latex_escapes(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len() + 16);
    let mut in_math = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '$' {
            out.push('$');
            if i + 1 < chars.len() && chars[i + 1] == '$' {
                out.push('$');
                in_math = !in_math;
                i += 2;
                continue;
            }
            in_math = !in_math;
            i += 1;
            continue;
        }
        match c {
            '\u{0007}' => out.push_str("\\a"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000B}' => out.push_str("\\v"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' if in_math => out.push_str("\\n"),
            '\r' if in_math => out.push_str("\\r"),
            '\t' if in_math => out.push_str("\\t"),
            other => out.push(other),
        }
        i += 1;
    }
    out
}

/// Map the [`Role`] the limiter knows about to the [`UsageKind`] column
/// `api_usage` tracks. They're parallel enums because `Vision` / `Reasoner`
/// predate the multi-agent layer.
fn role_to_usage_kind(role: Role) -> UsageKind {
    match role {
        Role::Extractor => UsageKind::Reasoner,
        Role::Curator => UsageKind::Curator,
        Role::Bridge => UsageKind::Bridge,
        Role::Harvester => UsageKind::Harvester,
        Role::Theorem => UsageKind::Theorem,
        Role::Derivation => UsageKind::Derivation,
        Role::Report => UsageKind::Report,
        Role::FormulaExtractor => UsageKind::FormulaExtract,
    }
}

/// Pull (trace_id, span_id, parent_span_id) strings out of the active
/// tracing span, if an OTel subscriber has been installed. All three are
/// `None` when telemetry is running in stdout-only mode.
fn current_trace_ids() -> (Option<String>, Option<String>, Option<String>) {
    let span = Span::current();
    let ctx = span.context();
    let sc = ctx.span().span_context().clone();
    if !sc.is_valid() {
        return (None, None, None);
    }
    let trace_id = format!("{:032x}", u128::from_be_bytes(sc.trace_id().to_bytes()));
    let span_id = format!("{:016x}", u64::from_be_bytes(sc.span_id().to_bytes()));
    (Some(trace_id), Some(span_id), None)
}

/// Structured record of one agent LLM call. Written to both the OTel
/// span (as an event) and the `agent_events` SQLite table, so the status
/// page and Jaeger both see the same discrete events.
#[allow(dead_code)]
pub(crate) struct AgentCall<'a> {
    pub role: Role,
    pub input: &'a str,
    pub output: &'a str,
    pub thinking: Option<&'a str>,
    pub payload_json: Option<&'a str>,
    pub started: Instant,
}

/// Record an `agent_events` row plus a matching OTel span event, and
/// increment today's `api_usage` counter for the role. Token counts are
/// estimated from char length — see [`estimate_tokens`] for why.
#[allow(dead_code)]
pub(crate) async fn record_agent_call(db: &Db, call: AgentCall<'_>) -> AppResult<(i64, i64)> {
    let tokens_sent = estimate_tokens(call.input);
    let tokens_received = estimate_tokens(call.output);
    let duration_ms = call.started.elapsed().as_millis() as i64;

    let input_summary = truncate(call.input, EVENT_SUMMARY_MAX);
    let output_summary = truncate(call.output, EVENT_SUMMARY_MAX);
    let role_name = call.role.as_str();

    // Mirror onto the active span so Jaeger sees the same event.
    tracing::info!(
        agent.role = role_name,
        agent.event = "call",
        tokens.sent = tokens_sent,
        tokens.received = tokens_received,
        duration_ms,
        "agent call"
    );

    let (trace_id, span_id, parent_span_id) = current_trace_ids();

    db.insert_agent_event(
        trace_id.as_deref(),
        span_id.as_deref(),
        parent_span_id.as_deref(),
        role_name,
        "call",
        Some(&input_summary),
        Some(&output_summary),
        call.thinking,
        call.payload_json,
        tokens_sent,
        tokens_received,
        Some(duration_ms),
    )
    .await?;

    db.increment_usage(role_to_usage_kind(call.role), tokens_sent, tokens_received)
        .await?;

    Ok((tokens_sent, tokens_received))
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tempfile::tempdir;

    use super::role_to_usage_kind;
    use crate::{db::Db, limiter::Role};

    #[tokio::test]
    async fn role_usage_mapping_increments_expected_counter() {
        let tmp = tempdir().expect("tempdir");
        let db_path = tmp.path().join("usage-mapping.sqlite3");
        let db = Db::open(&db_path).await.expect("open db");
        let today = Utc::now().date_naive();

        for role in Role::all() {
            db.increment_usage(role_to_usage_kind(*role), 0, 0)
                .await
                .expect("increment usage");
        }

        let usage = db.usage_for(today).await.expect("load usage row");
        assert_eq!(usage.reasoner_calls, 1);
        assert_eq!(usage.curator_calls, 1);
        assert_eq!(usage.bridge_calls, 1);
        assert_eq!(usage.harvester_calls, 1);
        assert_eq!(usage.theorem_calls, 1);
        assert_eq!(usage.derivation_calls, 1);
        assert_eq!(usage.report_calls, 1);
        assert_eq!(usage.formula_extract_calls, 1);
    }
}
