//! Markdown → text chunks.
//!
//! Strategy: split on H1/H2/H3 headings, fall back to ~1200-char windows for
//! sections without headings. Each chunk keeps the heading path as `section`
//! so search results can name where the snippet came from.

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use super::ChunkRecord;

const MAX_CHARS: usize = 1500;

pub fn chunk_markdown(note_id: &str, markdown: &str) -> Vec<ChunkRecord> {
    let mut chunks = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut buf = String::new();
    let mut in_heading = false;
    let mut byte_cursor = 0i64;
    let mut ord: i64 = 0;

    let parser = Parser::new(markdown);
    for ev in parser {
        match ev {
            Event::Start(Tag::Heading { level, .. }) => {
                flush(&mut chunks, note_id, &mut ord, &current_heading, &mut buf, &mut byte_cursor);
                if matches!(level, HeadingLevel::H1 | HeadingLevel::H2 | HeadingLevel::H3) {
                    in_heading = true;
                    current_heading = Some(String::new());
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
            }
            Event::Text(t) | Event::Code(t) => {
                if in_heading {
                    if let Some(h) = current_heading.as_mut() {
                        h.push_str(&t);
                    }
                } else {
                    buf.push_str(&t);
                    buf.push(' ');
                    if buf.len() > MAX_CHARS {
                        flush(
                            &mut chunks,
                            note_id,
                            &mut ord,
                            &current_heading,
                            &mut buf,
                            &mut byte_cursor,
                        );
                    }
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if !in_heading {
                    buf.push('\n');
                }
            }
            _ => {}
        }
    }
    flush(&mut chunks, note_id, &mut ord, &current_heading, &mut buf, &mut byte_cursor);
    if chunks.is_empty() && !markdown.trim().is_empty() {
        // Non-markdown / no headings: dump the whole thing as one chunk.
        chunks.push(ChunkRecord {
            id: format!("{note_id}:0"),
            note_id: note_id.to_string(),
            ord: 0,
            section: None,
            byte_start: 0,
            byte_end: markdown.len() as i64,
            text: markdown.trim().to_string(),
        });
    }
    chunks
}

fn flush(
    out: &mut Vec<ChunkRecord>,
    note_id: &str,
    ord: &mut i64,
    section: &Option<String>,
    buf: &mut String,
    byte_cursor: &mut i64,
) {
    let text = buf.trim().to_string();
    if text.is_empty() {
        buf.clear();
        return;
    }
    let start = *byte_cursor;
    let end = start + text.len() as i64;
    out.push(ChunkRecord {
        id: format!("{note_id}:{ord}"),
        note_id: note_id.to_string(),
        ord: *ord,
        section: section.clone(),
        byte_start: start,
        byte_end: end,
        text,
    });
    *ord += 1;
    *byte_cursor = end;
    buf.clear();
}

/// Extract inline `#tag` tokens (ASCII only) from markdown text.
pub fn extract_tags(markdown: &str) -> Vec<String> {
    let mut out = Vec::new();
    for tok in markdown.split_whitespace() {
        if let Some(rest) = tok.strip_prefix('#') {
            if !rest.is_empty()
                && rest
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '/')
            {
                out.push(rest.to_lowercase());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Extract the first H1 as a title, else first non-empty line.
pub fn extract_title(markdown: &str) -> Option<String> {
    for line in markdown.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("# ") {
            return Some(rest.trim().to_string());
        }
    }
    markdown
        .lines()
        .map(|s| s.trim())
        .find(|s| !s.is_empty())
        .map(|s| s.to_string())
}

/// Detect inline TODO lines and return (line, title).
pub fn extract_todos(markdown: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (i, line) in markdown.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("- [ ]") || t.starts_with("* [ ]") || t.to_ascii_uppercase().starts_with("TODO:") {
            let title = t
                .trim_start_matches(|c: char| c == '-' || c == '*' || c.is_whitespace())
                .trim_start_matches("[ ]")
                .trim_start_matches("TODO:")
                .trim_start_matches("todo:")
                .trim()
                .to_string();
            if !title.is_empty() {
                out.push((i + 1, title));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_by_heading() {
        let md = "# A\nfirst body\n## B\nsecond body\n";
        let chunks = chunk_markdown("n1", md);
        assert!(chunks.len() >= 2);
        assert!(chunks[0].text.contains("first body"));
    }

    #[test]
    fn tags_and_todos() {
        let md = "# note\nSome #work text\n- [ ] finish report\nTODO: call Alex";
        assert_eq!(extract_tags(md), vec!["work".to_string()]);
        let todos = extract_todos(md);
        assert_eq!(todos.len(), 2);
    }
}
