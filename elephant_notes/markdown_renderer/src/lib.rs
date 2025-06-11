//! Markdown rendering utilities.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use pulldown_cmark::{Parser, Options, html};

/// Render a slice of the file at `path` between byte offsets as HTML.
pub fn viewport<P: AsRef<Path>>(path: P, start: u64, end: u64) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(start))?;
    let mut buf = vec![0u8; (end - start) as usize];
    file.read_exact(&mut buf)?;
    let text = String::from_utf8_lossy(&buf);
    let parser = Parser::new_ext(&text, Options::all());
    let mut out = String::new();
    html::push_html(&mut out, parser);
    Ok(out)
}
