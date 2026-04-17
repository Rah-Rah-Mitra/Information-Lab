//! Every N seconds, write SYSTEM_STATUS.md to the vault with queue depth
//! and today's API usage. Keeps the user informed via Obsidian itself.

use std::{path::PathBuf, time::Duration};

use chrono::Utc;
use tokio::{fs, io::AsyncWriteExt, time};
use tracing::{error, info};

use crate::db::Db;
use crate::error::AppResult;

pub fn spawn(db: Db, vault_dir: PathBuf, interval: Duration) {
    tokio::spawn(async move {
        let mut ticker = time::interval(interval);
        ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            if let Err(e) = write_status(&db, &vault_dir).await {
                error!(error = %e, "status write failed");
            }
        }
    });
}

async fn write_status(db: &Db, vault_dir: &PathBuf) -> AppResult<()> {
    let pending = db.pending_count().await?;
    let today = Utc::now().date_naive();
    let usage = db.usage_for(today).await?;

    let body = format!(
        "# System Status\n\n\
         _Updated: {now}_\n\n\
         ## Queue\n\n\
         - Pending chunks: **{pending}**\n\n\
         ## API Usage — {day}\n\n\
         | Metric | Value |\n\
         | --- | --- |\n\
         | Reasoner calls | {rc} |\n\
         | Vision calls | {vc} |\n\
         | Tokens sent | {ts} |\n\
         | Tokens received | {tr} |\n",
        now = Utc::now().to_rfc3339(),
        day = usage.day,
        pending = pending,
        rc = usage.reasoner_calls,
        vc = usage.vision_calls,
        ts = usage.tokens_sent,
        tr = usage.tokens_received,
    );

    fs::create_dir_all(vault_dir).await?;
    let path = vault_dir.join("SYSTEM_STATUS.md");
    let mut f = fs::File::create(&path).await?;
    f.write_all(body.as_bytes()).await?;
    f.sync_all().await?;
    info!(path = %path.display(), "status updated");
    Ok(())
}
