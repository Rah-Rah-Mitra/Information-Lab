//! Telemetry setup: `tracing` + optional OpenTelemetry OTLP export.
//!
//! The fallback path (no OTLP endpoint configured) is identical to the
//! original `init_tracing` in `main.rs`: ANSI stdout layer + rolling JSON
//! file in `cfg.log_dir`. When `OTEL_EXPORTER_OTLP_ENDPOINT` is set, we
//! also install a `tracing-opentelemetry` layer that forwards spans to
//! the configured collector (OTLP/gRPC on :4317 or OTLP/HTTP on :4318).

use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{
    trace::{self as sdktrace, Sampler},
    Resource,
};
use opentelemetry_semantic_conventions::resource as semres;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::{config::Config, error::{AppError, AppResult}};

/// Guard that keeps the background log-writer alive and flushes the OTel
/// provider on drop. Must outlive the entire process — `main` leaks it.
pub struct TelemetryGuard {
    _file_guard: tracing_appender::non_blocking::WorkerGuard,
    otel_provider: Option<sdktrace::TracerProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(p) = self.otel_provider.take() {
            // Best-effort flush; ignore errors on shutdown.
            let _ = p.shutdown();
        }
    }
}

pub fn init(cfg: &Config) -> AppResult<TelemetryGuard> {
    std::fs::create_dir_all(&cfg.log_dir)?;
    let file_appender = tracing_appender::rolling::daily(&cfg.log_dir, "edge-kg-agent.log");
    let (nb_file, file_guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("edge_kg_agent=info,warn"));

    let stdout_layer = fmt::layer().with_target(true).with_ansi(true);
    let file_layer = fmt::layer().json().with_writer(nb_file);

    // Build the registry. We always install stdout + file. OTel is additive.
    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer);

    let otel_provider = if let Some(endpoint) = cfg.otlp_endpoint.as_deref() {
        let provider = build_otel_provider(endpoint, cfg)?;
        let tracer = provider.tracer(cfg.otel_service_name.clone());
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        registry.with(otel_layer).init();
        tracing::info!(endpoint, "opentelemetry enabled");
        Some(provider)
    } else {
        registry.init();
        tracing::info!("opentelemetry disabled (OTEL_EXPORTER_OTLP_ENDPOINT unset)");
        None
    };

    Ok(TelemetryGuard {
        _file_guard: file_guard,
        otel_provider,
    })
}

fn build_otel_provider(endpoint: &str, cfg: &Config) -> AppResult<sdktrace::TracerProvider> {
    // Pick gRPC for :4317, HTTP for :4318 — a simple heuristic the collector
    // docs all use. Users who need to override can set a full URL.
    let is_http = endpoint.contains(":4318") || endpoint.starts_with("http://127.0.0.1:4318")
        || endpoint.starts_with("https://") && endpoint.contains(":4318");

    let exporter = if is_http {
        opentelemetry_otlp::new_exporter()
            .http()
            .with_endpoint(endpoint)
            .with_protocol(Protocol::HttpBinary)
            .build_span_exporter()
            .map_err(|e| AppError::other(format!("otlp http exporter: {e}")))?
    } else {
        opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(endpoint)
            .build_span_exporter()
            .map_err(|e| AppError::other(format!("otlp grpc exporter: {e}")))?
    };

    let provider = sdktrace::TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_config(
            sdktrace::Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_resource(build_resource(cfg)),
        )
        .build();

    // Set as global so autoagents / any library that looks up a global
    // tracer provider sees ours.
    opentelemetry::global::set_tracer_provider(provider.clone());
    Ok(provider)
}

fn build_resource(cfg: &Config) -> Resource {
    let mut attrs = vec![
        KeyValue::new(semres::SERVICE_NAME, cfg.otel_service_name.clone()),
        KeyValue::new(
            semres::SERVICE_VERSION,
            env!("CARGO_PKG_VERSION").to_string(),
        ),
    ];

    if let Some(raw) = &cfg.otel_resource_attributes {
        for kv in raw.split(',') {
            let kv = kv.trim();
            if kv.is_empty() {
                continue;
            }
            if let Some((k, v)) = kv.split_once('=') {
                attrs.push(KeyValue::new(k.trim().to_string(), v.trim().to_string()));
            }
        }
    }

    Resource::new(attrs)
}
