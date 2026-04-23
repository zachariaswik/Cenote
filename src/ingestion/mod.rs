//! Privacy-first ingestion.
//!
//! Public entrypoint: `pipeline::ingest`. Everything else (parsers, extractor,
//! sidecar writer) is wired internally.
//!
//! Privacy contract: callers outside this module — and tool responses — must
//! never see raw file bytes. Only the cloud-safe outputs in `IngestResult`
//! leave this module.

pub mod parsers;
pub mod pipeline;
pub mod sidecar;
