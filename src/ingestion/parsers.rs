//! Parsers produce plain-text strings. Supported: md, txt, pdf. Image OCR is
//! stubbed out (returns an explicit `Unsupported` so callers can tell the
//! user to pre-OCR).

use std::path::Path;

use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Markdown,
    Text,
    Pdf,
    Image,
    Unknown,
}

pub fn detect_kind(path: &Path) -> FileKind {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "md" | "markdown" => FileKind::Markdown,
        "txt" | "log" => FileKind::Text,
        "pdf" => FileKind::Pdf,
        "png" | "jpg" | "jpeg" | "heic" | "tiff" | "webp" => FileKind::Image,
        _ => FileKind::Unknown,
    }
}

pub fn extract_text(path: &Path) -> Result<(FileKind, String)> {
    let kind = detect_kind(path);
    let text = match kind {
        FileKind::Markdown | FileKind::Text => {
            std::fs::read_to_string(path).with_context(|| format!("reading {path:?}"))?
        }
        FileKind::Pdf => extract_pdf(path)?,
        FileKind::Image => {
            bail!("image ingestion requires OCR — not enabled in this build")
        }
        FileKind::Unknown => bail!("unsupported file type: {path:?}"),
    };
    Ok((kind, text))
}

fn extract_pdf(path: &Path) -> Result<String> {
    let doc = lopdf::Document::load(path).with_context(|| format!("opening pdf {path:?}"))?;
    let mut out = String::new();
    for (page_num, _page_id) in doc.get_pages() {
        match doc.extract_text(&[page_num]) {
            Ok(text) => {
                out.push_str(&text);
                out.push_str("\n\n");
            }
            Err(e) => {
                tracing::warn!(page = page_num, error=%e, "pdf page extract failed; continuing");
            }
        }
    }
    Ok(out)
}

pub fn kind_label(k: FileKind) -> &'static str {
    match k {
        FileKind::Markdown => "markdown",
        FileKind::Text => "text",
        FileKind::Pdf => "pdf",
        FileKind::Image => "image",
        FileKind::Unknown => "unknown",
    }
}
