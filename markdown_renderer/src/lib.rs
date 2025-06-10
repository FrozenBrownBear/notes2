use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use pulldown_cmark::{Parser, Options, Event, Tag, CodeBlockKind};

#[derive(Debug, Clone)]
pub enum Widget {
    Header(u32, String),
    Bold(String),
    Italic(String),
    CodeBlock { lang: Option<String>, code: String },
    Latex(String),
    Image { alt: String, url: String },
    Table(Vec<Widget>),
    Mermaid(String),
    Details(Vec<Widget>),
    Text(String),
    NoteLink(String),
    IncludeNote(String),
    CustomImage(String),
}

fn parse_macro<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let trimmed = text.trim();
    let rest = trimmed.strip_prefix("{%")?.trim_start();
    let rest = rest.strip_prefix(name)?.trim_start();
    let arg_end = rest.find("%}")?;
    let arg = rest[..arg_end].trim();
    if !arg.is_empty() {
        Some(arg)
    } else {
        None
    }
}

fn parse_latex_inline(text: &str) -> Option<String> {
    if text.starts_with('$') && text.ends_with('$') && text.len() >= 2 {
        let start = text.chars().take_while(|&c| c == '$').count();
        let end = text.chars().rev().take_while(|&c| c == '$').count();
        if start == end && (start == 1 || start == 2) {
            return Some(text[start..text.len() - end].to_string());
        }
    }
    None
}

pub fn render_markdown(input: &str) -> Vec<Widget> {
    let parser = Parser::new_ext(input, Options::all());
    let mut iter = parser.peekable();
    parse_events(&mut iter)
}

fn parse_events<'a, I>(events: &mut std::iter::Peekable<I>) -> Vec<Widget>
where
    I: Iterator<Item = Event<'a>>,
{
    
    let mut widgets = Vec::new();
    while let Some(event) = events.next() {
        match event {
            Event::Start(Tag::Heading(level, ..)) => {
                let text = collect_text(events, Tag::Heading(level, None, Vec::new()));
                widgets.push(Widget::Header(level as u32, text));
            }
            Event::Start(Tag::Strong) => {
                let text = collect_text(events, Tag::Strong);
                widgets.push(Widget::Bold(text));
            }
            Event::Start(Tag::Emphasis) => {
                let text = collect_text(events, Tag::Emphasis);
                widgets.push(Widget::Italic(text));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match &kind {
                    CodeBlockKind::Fenced(lang) => Some(lang.to_string()),
                    _ => None,
                };
                let text = collect_text(events, Tag::CodeBlock(kind.clone()));
                if lang.as_deref() == Some("mermaid") {
                    widgets.push(Widget::Mermaid(text));
                } else if lang.as_deref() == Some("math") {
                    widgets.push(Widget::Latex(text));
                } else {
                    widgets.push(Widget::CodeBlock { lang, code: text });
                }
            }
            Event::Start(Tag::Table(_aligns)) => {
                let inner = parse_until(events, |e| matches!(e, Event::End(Tag::Table(_))));
                widgets.push(Widget::Table(inner));
            }
            Event::Start(Tag::Image(_link, url, title)) => {
                widgets.push(Widget::Image {
                    alt: title.to_string(),
                    url: url.to_string(),
                });
            }
            Event::Start(Tag::Link(_link, url, _title)) => {
                if url.starts_with("note://") {
                    // consume inner text
                    let _ = parse_until(events, |e| matches!(e, Event::End(Tag::Link(_, _, _))));
                    widgets.push(Widget::NoteLink(url.to_string()));
                }
            }
            Event::Html(html) => {
                let trimmed = html.trim();
                if trimmed.starts_with("<details") {
                    let inner = parse_until(events, |e| match e {
                        Event::Html(h) => h.trim().starts_with("</details"),
                        _ => false,
                    });
                    widgets.push(Widget::Details(inner));
                }
            }
            Event::Text(text) => {
                let t = text.trim();
                if let Some(arg) = parse_macro(t, "include-note") {
                    widgets.push(Widget::IncludeNote(arg.to_string()));
                } else if let Some(arg) = parse_macro(t, "image") {
                    widgets.push(Widget::CustomImage(arg.to_string()));
                } else if let Some(latex) = parse_latex_inline(t) {
                    widgets.push(Widget::Latex(latex));
                } else {
                    widgets.push(Widget::Text(text.to_string()));
                }
            }
            _ => {}
        }
    }
    widgets
}

fn collect_text<'a, I>(events: &mut std::iter::Peekable<I>, until: Tag<'a>) -> String
where
    I: Iterator<Item = Event<'a>>,
{
    let mut text = String::new();
    while let Some(event) = events.next() {
        match &event {
            Event::End(tag) if tag == &until => break,
            Event::Text(t) => text.push_str(t),
            _ => {}
        }
    }
    text
}

fn parse_until<'a, I, F>(events: &mut std::iter::Peekable<I>, end: F) -> Vec<Widget>
where
    I: Iterator<Item = Event<'a>>,
    F: Fn(&Event<'a>) -> bool,
{
    let mut collected = Vec::new();
    while let Some(event) = events.next() {
        if end(&event) {
            break;
        } else {
            let mut single = vec![event];
            let mut it = single.into_iter().peekable();
            collected.extend(parse_events(&mut it));
        }
    }
    collected
}

pub fn render_viewport<P: AsRef<Path>>(path: P, start: u64, end: u64) -> std::io::Result<Vec<Widget>> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(start))?;
    let mut buf = vec![0u8; (end - start) as usize];
    file.read_exact(&mut buf)?;
    let content = String::from_utf8_lossy(&buf);
    Ok(render_markdown(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport() {
        use std::io::Write;
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path();
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "# Title\ntext").unwrap();
        let res = render_viewport(path, 0, 9).unwrap();
        assert!(!res.is_empty());
    }
}
pub fn placeholder() {}
