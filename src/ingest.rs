//! PDF → hash → text chunks → SQLite. Zero-API-cost local step.
//!
//! We group pages into ~CHUNK_TARGET_TOKENS-sized chunks so batches downstream
//! can be assembled efficiently. `pdf_oxide` does the heavy lifting.

use std::path::Path;

use sha2::{Digest, Sha256};
use tokio::fs;
use tracing::{debug, info, warn};

use crate::db::Db;
use crate::error::{AppError, AppResult};

/// Target tokens per DB row. Small enough to give the batcher flexibility,
/// large enough to avoid INSERT-per-paragraph overhead.
const CHUNK_TARGET_TOKENS: usize = 1500;

/// Rough heuristic: ~4 chars per token. Good enough for budgeting; the real
/// count comes back from the API.
fn approx_tokens(s: &str) -> usize {
    (s.len() / 4).max(1)
}

/// Hash a file by streaming (avoid loading whole PDF into RAM on the Pi).
pub async fn hash_file(path: &Path) -> AppResult<(String, u64)> {
    use tokio::io::AsyncReadExt;
    let mut f = fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024];
    let mut total: u64 = 0;
    loop {
        let n = f.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        total += n as u64;
    }
    Ok((hex::encode(hasher.finalize()), total))
}

/// Pull per-page markdown via pdf_oxide, regroup into token-budgeted chunks,
/// and persist to SQLite. Runs the CPU-bound extraction on a blocking thread
/// so we don't stall the tokio runtime on the Pi's limited cores.
pub async fn ingest_pdf(db: &Db, watch_dir: &Path, path: &Path) -> AppResult<IngestOutcome> {
    let (hash, size) = hash_file(path).await?;
    let source_name = derive_source_name(watch_dir, path);
    let inserted = db
        .insert_document(
            &hash,
            &path.display().to_string(),
            size as i64,
            &source_name,
        )
        .await?;
    if !inserted {
        debug!(hash = %hash, path = %path.display(), "duplicate — skipping");
        return Ok(IngestOutcome::Duplicate);
    }

    info!(hash = %hash, path = %path.display(), bytes = size, "extracting");

    let pages = extract_pages_blocking(path).await?;
    if pages.is_empty() {
        warn!(path = %path.display(), "no pages extracted");
        db.mark_extracted(&hash).await?;
        return Ok(IngestOutcome::Empty);
    }

    // Regroup pages into token-budgeted chunks with page-range bookkeeping.
    let mut chunk_buf = String::new();
    let mut chunk_start: i64 = 1;
    let mut chunk_tokens: usize = 0;
    let mut chunks_written = 0_u32;

    for (idx, page_md) in pages.iter().enumerate() {
        let page_no = (idx as i64) + 1;
        let t = approx_tokens(page_md);
        if !chunk_buf.is_empty() && chunk_tokens + t > CHUNK_TARGET_TOKENS {
            let end_page = page_no - 1;
            db.insert_chunk(
                &hash,
                chunk_start,
                end_page,
                &chunk_buf,
                chunk_tokens as i64,
            )
            .await?;
            chunks_written += 1;
            chunk_buf.clear();
            chunk_tokens = 0;
            chunk_start = page_no;
        }
        if !chunk_buf.is_empty() {
            chunk_buf.push_str("\n\n");
        }
        chunk_buf.push_str(page_md);
        chunk_tokens += t;
    }

    if !chunk_buf.is_empty() {
        let end_page = pages.len() as i64;
        db.insert_chunk(&hash, chunk_start, end_page, &chunk_buf, chunk_tokens as i64)
            .await?;
        chunks_written += 1;
    }

    db.mark_extracted(&hash).await?;
    info!(hash = %hash, chunks = chunks_written, "ingest complete");
    Ok(IngestOutcome::Ingested {
        hash,
        chunks: chunks_written,
    })
}

/// Run pdf_oxide on a blocking thread. `to_markdown(page)` gives us clean,
/// reading-order text suitable for the reasoner.
async fn extract_pages_blocking(path: &Path) -> AppResult<Vec<String>> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || -> AppResult<Vec<String>> {
        let mut doc = pdf_oxide::document::PdfDocument::open(&path)
            .map_err(|e| AppError::Pdf(format!("open: {e}")))?;
        let n = doc
            .page_count()
            .map_err(|e| AppError::Pdf(format!("page_count: {e}")))?;
        let opts = pdf_oxide::converters::ConversionOptions::default();
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            match doc.to_markdown(i, &opts) {
                Ok(md) => {
                    let trimmed = md.trim();
                    if !trimmed.is_empty() {
                        out.push(trimmed.to_string());
                    }
                }
                Err(e) => {
                    warn!(page = i, error = %e, "page extract failed — skipping");
                }
            }
        }
        Ok(out)
    })
    .await
    .map_err(|e| AppError::other(format!("join: {e}")))?
}

/// Pick a human-readable source name for this PDF. Used as the vault
/// folder and source-index key — stable per PDF, deterministic, no hash
/// exposure.
///
/// Rule: first path component under `watch_dir` if the PDF sits in a
/// subdir (e.g. `public/GIS/foo.pdf` → `GIS`); otherwise the filename
/// stem (e.g. `public/foo.pdf` → `foo`).
fn derive_source_name(watch_dir: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(watch_dir).unwrap_or(path);
    let mut comps = rel.components();
    if let Some(first) = comps.next() {
        if comps.next().is_some() {
            // At least one more component remains — `first` is a dir.
            if let Some(s) = first.as_os_str().to_str() {
                if !s.is_empty() {
                    return s.to_string();
                }
            }
        }
    }
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

#[derive(Debug)]
pub enum IngestOutcome {
    Ingested { hash: String, chunks: u32 },
    Duplicate,
    Empty,
}
