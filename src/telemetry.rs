//! Tracing setup and lightweight metrics counters.
//!
//! We keep metrics deliberately small — atomic counters + a histogram-free
//! latency accumulator — so there is zero dependency on Prometheus/OTEL in
//! local-first deployments. A `/healthz` endpoint surfaces the values.

use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init(log_level: &str) -> anyhow::Result<()> {
    let filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = fmt::layer().with_target(true).with_level(true);
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init();
    Ok(())
}

#[derive(Default)]
pub struct Metrics {
    pub ingestion_jobs: AtomicU64,
    pub ingestion_failures: AtomicU64,
    pub tool_invocations: AtomicU64,
    pub tool_failures: AtomicU64,
    pub search_latency_ms_total: AtomicU64,
    pub search_calls: AtomicU64,
    pub recall_latency_ms_total: AtomicU64,
    pub recall_calls: AtomicU64,
    pub outbound_delivery_failures: AtomicU64,
}

impl Metrics {
    pub fn incr(&self, c: &AtomicU64) {
        c.fetch_add(1, Ordering::Relaxed);
    }
    pub fn add(&self, c: &AtomicU64, n: u64) {
        c.fetch_add(n, Ordering::Relaxed);
    }
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            ingestion_jobs: self.ingestion_jobs.load(Ordering::Relaxed),
            ingestion_failures: self.ingestion_failures.load(Ordering::Relaxed),
            tool_invocations: self.tool_invocations.load(Ordering::Relaxed),
            tool_failures: self.tool_failures.load(Ordering::Relaxed),
            search_latency_ms_total: self.search_latency_ms_total.load(Ordering::Relaxed),
            search_calls: self.search_calls.load(Ordering::Relaxed),
            recall_latency_ms_total: self.recall_latency_ms_total.load(Ordering::Relaxed),
            recall_calls: self.recall_calls.load(Ordering::Relaxed),
            outbound_delivery_failures: self.outbound_delivery_failures.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MetricsSnapshot {
    pub ingestion_jobs: u64,
    pub ingestion_failures: u64,
    pub tool_invocations: u64,
    pub tool_failures: u64,
    pub search_latency_ms_total: u64,
    pub search_calls: u64,
    pub recall_latency_ms_total: u64,
    pub recall_calls: u64,
    pub outbound_delivery_failures: u64,
}
