//! Telegram Bot API integration for ad-hoc research requests and lightweight
//! vault navigation.

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use reqwest::StatusCode;
use serde::Serialize;
use teloxide::{prelude::*, requests::Requester};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::{config::Config, db::Db};

#[derive(Clone)]
struct TelegramState {
    auth_code: Arc<str>,
    authenticated_chats: Arc<Mutex<HashSet<i64>>>,
    vault_dir: PathBuf,
    research_request_endpoint: String,
    http: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct ResearchRequestBody {
    problem: String,
    max_iterations: Option<u8>,
    skills_scope: Vec<String>,
    telegram_chat_id: Option<String>,
    telegram_message_id: Option<i64>,
}

pub fn spawn(cfg: &Config, _db: Db) {
    let Some(token) = cfg.telegram_bot_token.clone() else {
        info!("telegram bot disabled (TELEGRAM_BOT_TOKEN not set)");
        return;
    };

    let auth_seed = uuid::Uuid::new_v4().as_simple().to_string();
    let auth_code = auth_seed[..8].to_ascii_uppercase();
    info!(
        auth_code = %auth_code,
        endpoint = %cfg.research_request_endpoint,
        "telegram bot enabled; use /auth <CODE> in Telegram to link a chat"
    );

    let telegram_api_base = cfg.telegram_api_base.clone();

    let state = TelegramState {
        auth_code: Arc::<str>::from(auth_code),
        authenticated_chats: Arc::new(Mutex::new(HashSet::new())),
        vault_dir: cfg.vault_dir.clone(),
        research_request_endpoint: cfg.research_request_endpoint.clone(),
        http: reqwest::Client::new(),
    };

    tokio::spawn(async move {
        let bot = if telegram_api_base.trim() != "https://api.telegram.org" {
            match reqwest::Url::parse(&telegram_api_base) {
                Ok(url) => Bot::new(token).set_api_url(url),
                Err(e) => {
                    warn!(error = %e, base = %telegram_api_base, "invalid TELEGRAM_API_BASE; using default");
                    Bot::new(token)
                }
            }
        } else {
            Bot::new(token)
        };
        teloxide::repl(bot, move |bot: Bot, msg: Message| {
            let state = state.clone();
            async move {
                handle_message(bot, msg, state).await;
                respond(())
            }
        })
        .await;
    });
}

async fn handle_message(bot: Bot, msg: Message, state: TelegramState) {
    let Some(text) = msg.text() else {
        return;
    };

    let chat_id = msg.chat.id;
    let cmd = text.trim();

    if cmd.starts_with("/start") {
        let body = "Information Lab bot ready.\n\nAuthenticate with /auth <CODE>.\nAfter auth: /library [path], /view <path>, /research <question>.";
        let _ = bot.send_message(chat_id, body).await;
        return;
    }

    if let Some(code) = cmd.strip_prefix("/auth ") {
        if code.trim().eq_ignore_ascii_case(&state.auth_code) {
            {
                let mut guard = state.authenticated_chats.lock().await;
                guard.insert(chat_id.0);
            }
            let _ = bot
                .send_message(
                    chat_id,
                    "Authenticated. You can now use /library, /view, and /research.",
                )
                .await;
        } else {
            let _ = bot.send_message(chat_id, "Invalid one-time code.").await;
        }
        return;
    }

    if !is_authenticated(chat_id.0, &state).await {
        let _ = bot
            .send_message(
                chat_id,
                "Not authenticated. Use /auth <CODE> from terminal output.",
            )
            .await;
        return;
    }

    if cmd.starts_with("/library") {
        let suffix = cmd.strip_prefix("/library").unwrap_or_default().trim();
        match list_library(&state.vault_dir, suffix).await {
            Ok(out) => {
                let _ = bot.send_message(chat_id, out).await;
            }
            Err(e) => {
                let _ = bot
                    .send_message(chat_id, format!("library error: {e}"))
                    .await;
            }
        }
        return;
    }

    if let Some(path) = cmd.strip_prefix("/view ") {
        match view_file(&state.vault_dir, path.trim()).await {
            Ok(out) => {
                let _ = bot.send_message(chat_id, out).await;
            }
            Err(e) => {
                let _ = bot.send_message(chat_id, format!("view error: {e}")).await;
            }
        }
        return;
    }

    if let Some(problem) = cmd.strip_prefix("/research ") {
        match enqueue_research(problem.trim(), &msg, &state).await {
            Ok(task_id) => {
                let _ = bot
                    .send_message(chat_id, format!("Queued research task #{task_id}."))
                    .await;
            }
            Err(e) => {
                error!(error = %e, "telegram research enqueue failed");
                let _ = bot
                    .send_message(chat_id, "Failed to enqueue request; check service logs.")
                    .await;
            }
        }
        return;
    }

    let _ = bot
        .send_message(
            chat_id,
            "Unknown command. Use /library [path], /view <path>, or /research <question>.",
        )
        .await;
}

async fn is_authenticated(chat_id: i64, state: &TelegramState) -> bool {
    state.authenticated_chats.lock().await.contains(&chat_id)
}

async fn enqueue_research(
    problem: &str,
    msg: &Message,
    state: &TelegramState,
) -> anyhow::Result<i64> {
    if problem.is_empty() {
        anyhow::bail!("problem must be non-empty")
    }
    let body = ResearchRequestBody {
        problem: problem.to_string(),
        max_iterations: Some(2),
        skills_scope: Vec::new(),
        telegram_chat_id: Some(msg.chat.id.0.to_string()),
        telegram_message_id: Some(i64::from(msg.id.0)),
    };

    let resp = state
        .http
        .post(&state.research_request_endpoint)
        .json(&body)
        .send()
        .await?;

    if resp.status() != StatusCode::ACCEPTED {
        let status = resp.status();
        let txt = resp.text().await.unwrap_or_else(|_| String::new());
        anyhow::bail!("research endpoint returned {status}: {txt}")
    }

    let payload: serde_json::Value = resp.json().await?;
    let task_id = payload
        .get("task_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("missing task_id in response"))?;
    Ok(task_id)
}

async fn list_library(vault_dir: &Path, suffix: &str) -> anyhow::Result<String> {
    let target = resolve_path(vault_dir, suffix)?;
    let mut entries = tokio::fs::read_dir(&target).await?;
    let mut dirs = Vec::new();
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let kind = entry.file_type().await?;
        let name = entry.file_name().to_string_lossy().to_string();
        if kind.is_dir() {
            dirs.push(format!("📁 {name}"));
        } else {
            files.push(format!("📄 {name}"));
        }
        if dirs.len() + files.len() >= 40 {
            break;
        }
    }

    dirs.sort();
    files.sort();
    let display = target.strip_prefix(vault_dir).unwrap_or(&target).display();
    let mut lines = vec![format!("Library path: /{}", display)];
    lines.extend(dirs);
    lines.extend(files);
    if lines.len() == 1 {
        lines.push("(empty directory)".to_string());
    }
    Ok(lines.join("\n"))
}

async fn view_file(vault_dir: &Path, rel: &str) -> anyhow::Result<String> {
    let target = resolve_path(vault_dir, rel)?;
    if !target.is_file() {
        anyhow::bail!("path is not a file")
    }
    let content = tokio::fs::read_to_string(&target).await?;
    let mut out = String::new();
    for (idx, line) in content.lines().take(60).enumerate() {
        out.push_str(&format!("{:>2}: {}\n", idx + 1, line));
    }
    if out.is_empty() {
        out = "(empty file)".to_string();
    }
    if content.lines().count() > 60 {
        out.push_str("... (truncated)");
    }
    Ok(out)
}

fn resolve_path(root: &Path, user_input: &str) -> anyhow::Result<PathBuf> {
    let trimmed = user_input.trim().trim_start_matches('/');
    let candidate = root.join(trimmed);
    let canonical_root = root.canonicalize()?;
    let canonical_target = candidate.canonicalize()?;

    if !canonical_target.starts_with(&canonical_root) {
        anyhow::bail!("path escapes vault root")
    }
    Ok(canonical_target)
}
