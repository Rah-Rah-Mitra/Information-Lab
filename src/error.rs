//! Central error type. All fallible layers return `AppResult<T>`.

use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("missing required env var: {0}")]
    MissingEnv(String),

    #[error("bad env var {0}: {1}")]
    BadEnv(String, String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("db: {0}")]
    Db(#[from] sqlx::Error),

    #[error("migrate: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("pdf: {0}")]
    Pdf(String),

    #[error("watcher: {0}")]
    Watcher(#[from] notify::Error),

    #[allow(dead_code)] // reserved for surfacing upstream status codes
    #[error("api returned {status}: {body}")]
    Api { status: u16, body: String },

    #[error("schema validation: {0}")]
    Schema(String),

    #[allow(dead_code)] // reserved for actor-framework errors
    #[error("actor: {0}")]
    Actor(String),

    #[error("other: {0}")]
    Other(String),
}

impl AppError {
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}
