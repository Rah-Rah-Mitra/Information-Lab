//! Debounced filesystem watcher. Emits path events to a channel; the ingest
//! actor is responsible for handling them. Debounce is critical because
//! Syncthing writes files in many small chunks during a sync.

use std::{path::PathBuf, time::Duration};

use notify::{RecursiveMode, Watcher as _};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::error::AppResult;

pub fn spawn(
    watch_dir: PathBuf,
    debounce: Duration,
) -> AppResult<mpsc::Receiver<PathBuf>> {
    let (tx, rx) = mpsc::channel::<PathBuf>(256);

    // Bridge the sync notify thread → async mpsc.
    let (sync_tx, sync_rx) = std::sync::mpsc::channel::<
        Result<Vec<DebouncedEvent>, Vec<notify::Error>>,
    >();

    let mut debouncer = new_debouncer(debounce, None, move |res| {
        let _ = sync_tx.send(res);
    })?;

    debouncer
        .watcher()
        .watch(&watch_dir, RecursiveMode::Recursive)?;
    // Keep cache aligned with the watched tree.
    debouncer.cache().add_root(&watch_dir, RecursiveMode::Recursive);

    info!(dir = %watch_dir.display(), "watching");

    // Drain the sync channel on a blocking thread; forward PDF paths async.
    std::thread::Builder::new()
        .name("fs-bridge".into())
        .spawn(move || {
            // Keep debouncer alive inside the thread.
            let _debouncer = debouncer;
            while let Ok(res) = sync_rx.recv() {
                match res {
                    Ok(events) => {
                        for ev in events {
                            for path in ev.event.paths {
                                if is_pdf(&path) {
                                    debug!(path = %path.display(), "fs event");
                                    if tx.blocking_send(path).is_err() {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    Err(errs) => {
                        for e in errs {
                            error!(error = %e, "watcher error");
                        }
                    }
                }
            }
        })
        .map_err(std::io::Error::from)?;

    Ok(rx)
}

fn is_pdf(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}
