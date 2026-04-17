//! AutoAgents-based agent layer.
//!
//! Two agents:
//!
//! * [`KnowledgeGraphAgent`] — takes a large batched text blob and emits a
//!   strict KG JSON object. We call Gemma 4 directly via autoagents' Google
//!   backend with `StructuredOutputFormat`, so the output is schema-enforced.
//! * [`ResearchAgent`] — a ReAct agent with `web_search`, `web_fetch`, and
//!   `vault_search` tools. Invoked after KG extraction to deepen entities the
//!   reasoner surfaced.

use std::{num::NonZeroU32, sync::Arc, time::Duration};

use autoagents::{
    core::agent::{
        memory::SlidingWindowMemory, prebuilt::executor::ReActAgent, task::Task, AgentBuilder,
        DirectAgent,
    },
    llm::{
        backends::google::Google,
        builder::LLMBuilder,
        chat::{ChatMessage, ChatProvider, StructuredOutputFormat},
        LLMProvider,
    },
    prelude::{AgentOutputT, ToolT},
};
use autoagents_derive::{AgentHooks, AgentOutput};
use governor::{clock::DefaultClock, state::{InMemoryState, NotKeyed}, Quota, RateLimiter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, warn};

use crate::{
    config::Config,
    error::{AppError, AppResult},
    tools::{VaultSearchTool, WebFetchTool, WebSearchTool},
};

pub type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

// ----------------------------------------------------------------------------
// Output schema
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub source: String,
    pub target: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgOutput {
    pub summary: String,
    pub tags: Vec<String>,
    pub entities: Vec<String>,
    pub relationships: Vec<Relationship>,
    pub markdown_snippet: String,

    #[serde(skip)]
    pub tokens_sent: i64,
    #[serde(skip)]
    pub tokens_received: i64,
}

/// JSON schema handed to Gemma 4 via `response_schema`.
fn kg_schema() -> Value {
    json!({
        "type": "object",
        "required": ["summary", "tags", "entities", "relationships", "markdown_snippet"],
        "properties": {
            "summary":          { "type": "string" },
            "tags":             { "type": "array", "items": { "type": "string" } },
            "entities":         { "type": "array", "items": { "type": "string" } },
            "relationships": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["source", "target", "description"],
                    "properties": {
                        "source":      { "type": "string" },
                        "target":      { "type": "string" },
                        "description": { "type": "string" }
                    }
                }
            },
            "markdown_snippet": { "type": "string" }
        }
    })
}

// ----------------------------------------------------------------------------
// Knowledge graph extractor (schema-forced single-shot call)
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct KnowledgeGraphAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    model: String,
}

impl KnowledgeGraphAgent {
    pub fn new(cfg: &Config) -> AppResult<Self> {
        let rpm = NonZeroU32::new(cfg.rpm_limit.max(1))
            .ok_or_else(|| AppError::other("rpm must be >= 1"))?;
        let limiter = Arc::new(RateLimiter::direct(Quota::per_minute(rpm)));

        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(cfg.reasoner_model.clone())
            .temperature(0.2)
            .timeout_seconds(120)
            .build()
            .map_err(|e| AppError::other(format!("build Google llm: {e}")))?;

        Ok(Self {
            llm,
            limiter,
            model: cfg.reasoner_model.clone(),
        })
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    /// Single call: rate-limited, schema-forced, parsed.
    pub async fn extract(&self, batched_text: &str) -> AppResult<KgOutput> {
        // Governor: wait for a slot.
        self.limiter.until_ready().await;

        let system = "You are a knowledge-graph extractor. Read the provided \
                      documents and output ONLY JSON matching the schema. Use \
                      [[Concept]] wikilinks for each named entity inside \
                      markdown_snippet. Keep tags short and lowercase.";

        let messages = vec![
            ChatMessage::user()
                .content(format!("{system}\n\n---\n\n{batched_text}"))
                .build(),
        ];

        let schema = StructuredOutputFormat {
            name: "kg_output".to_string(),
            description: Some("Knowledge graph extraction result".into()),
            schema: Some(kg_schema()),
            strict: Some(true),
        };

        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("gemma chat: {e}")))?;

        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty chat response".into()))?;

        let mut parsed: KgOutput = serde_json::from_str(&text)
            .map_err(|e| AppError::Schema(format!("parse kg json: {e} :: {}", truncate(&text, 400))))?;

        // autoagents doesn't surface usage in a stable way yet — leave zero.
        parsed.tokens_sent = 0;
        parsed.tokens_received = 0;
        debug!(bytes = text.len(), "kg extraction ok");
        Ok(parsed)
    }
}

fn truncate(s: &str, max: usize) -> String {
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

// ----------------------------------------------------------------------------
// Research agent (ReAct + tools). Uses Gemini 3.1 Flash-Lite.
// ----------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, AgentOutput, Clone)]
pub struct ResearchBrief {
    #[output(description = "A 2–4 paragraph synthesis of external context on the concept")]
    pub brief: String,
    #[output(description = "Source URLs consulted")]
    pub sources: Vec<String>,
}

impl From<autoagents::core::agent::prebuilt::executor::ReActAgentOutput> for ResearchBrief {
    fn from(o: autoagents::core::agent::prebuilt::executor::ReActAgentOutput) -> Self {
        // ReAct returns a string; best-effort parse, fall back to raw.
        if let Ok(parsed) = serde_json::from_str::<ResearchBrief>(&o.response) {
            return parsed;
        }
        ResearchBrief {
            brief: o.response,
            sources: Vec::new(),
        }
    }
}

/// Research agent definition. Holds tool deps so `tools()` can hand them out
/// every time the ReAct executor asks.
#[derive(Clone, AgentHooks)]
pub struct ResearchAgentDef {
    pub web_search: WebSearchTool,
    pub web_fetch: WebFetchTool,
    pub vault_search: VaultSearchTool,
}

// Hand-written AgentDeriveT impl (the `#[agent]` macro's tool-initializer
// syntax can't take stateful tool instances; we need our Arc/http clones).
impl autoagents::core::agent::AgentDeriveT for ResearchAgentDef {
    type Output = ResearchBrief;

    fn name(&self) -> &'static str {
        "research_agent"
    }

    fn description(&self) -> &'static str {
        "Deep-research assistant that enriches knowledge-graph concepts with \
         external context. Use web_search to find authoritative sources, \
         web_fetch to pull page text, and vault_search to avoid duplicating \
         existing notes. Always cite your sources."
    }

    fn output_schema(&self) -> Option<Value> {
        Some(ResearchBrief::structured_output_format())
    }

    fn tools(&self) -> Vec<Box<dyn ToolT>> {
        vec![
            Box::new(self.web_search.clone()),
            Box::new(self.web_fetch.clone()),
            Box::new(self.vault_search.clone()),
        ]
    }
}

impl std::fmt::Debug for ResearchAgentDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("research_agent")
    }
}

/// Bundle of research-agent dependencies. Construct once, call per concept.
pub struct ResearchStack {
    llm: Arc<dyn LLMProvider>,
    web_search: WebSearchTool,
    web_fetch: WebFetchTool,
    vault_search: VaultSearchTool,
}

impl ResearchStack {
    pub fn new(cfg: &Config, tavily_key: Option<String>) -> AppResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(AppError::Http)?;

        let llm_google: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(cfg.vision_model.clone()) // reuse as our fast model
            .temperature(0.3)
            .timeout_seconds(60)
            .build()
            .map_err(|e| AppError::other(format!("build Google llm: {e}")))?;

        Ok(Self {
            llm: llm_google,
            web_search: WebSearchTool {
                http: http.clone(),
                api_key: tavily_key,
            },
            web_fetch: WebFetchTool { http },
            vault_search: VaultSearchTool {
                vault_dir: Arc::new(cfg.vault_dir.clone()),
            },
        })
    }

    /// Run the agent against a single concept and return the brief.
    pub async fn research(&self, concept: &str, hint: &str) -> AppResult<ResearchBrief> {
        let agent_def = ResearchAgentDef {
            web_search: self.web_search.clone(),
            web_fetch: self.web_fetch.clone(),
            vault_search: self.vault_search.clone(),
        };
        let agent = ReActAgent::new(agent_def);
        let memory = Box::new(SlidingWindowMemory::new(8));

        let handle = AgentBuilder::<_, DirectAgent>::new(agent)
            .llm(self.llm.clone())
            .memory(memory)
            .build()
            .await
            .map_err(|e| AppError::Actor(format!("research build: {e}")))?;

        let prompt = format!(
            "Research the concept: \"{concept}\".\n\nContext from source document:\n{hint}\n\n\
             Produce a concise brief with citations. Output JSON: \
             {{\"brief\": string, \"sources\": [url, ...]}}."
        );
        let task = Task::new(prompt);

        let raw = handle
            .agent
            .run(task)
            .await
            .map_err(|e| AppError::Actor(format!("research run: {e}")))?;

        let brief: ResearchBrief = raw.into();
        if brief.sources.is_empty() {
            warn!(concept, "research produced no sources");
        }
        Ok(brief)
    }
}
