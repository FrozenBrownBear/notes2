use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use std::io::SeekFrom;

use pulldown_cmark::{Parser, Options, Event, Tag, CodeBlockKind};
use regex::Regex;

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

pub fn render_markdown(input: &str) -> Vec<Widget> {
    let parser = Parser::new_ext(input, Options::all());
    let mut iter = parser.peekable();
    parse_events(&mut iter)
}

fn parse_events<'a, I>(events: &mut std::iter::Peekable<I>) -> Vec<Widget>
where
    I: Iterator<Item = Event<'a>>,
{
    let include_note = Regex::new(r"\{\%\s*include-note\s+([^\s%]+)\s*\%\}").unwrap();
    let image_macro = Regex::new(r"\{\%\s*image\s+([^\s%]+)\s*\%\}").unwrap();
    let latex_inline = Regex::new(r"^\${1,2}(.*)\${1,2}$").unwrap();

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
                if let Some(cap) = include_note.captures(t) {
                    widgets.push(Widget::IncludeNote(cap[1].to_string()));
                } else if let Some(cap) = image_macro.captures(t) {
                    widgets.push(Widget::CustomImage(cap[1].to_string()));
                } else if let Some(cap) = latex_inline.captures(t) {
                    widgets.push(Widget::Latex(cap[1].to_string()));
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

pub async fn render_viewport<P: AsRef<Path>>(path: P, start: u64, end: u64) -> std::io::Result<Vec<Widget>> {
    let mut file = File::open(path).await?;
    file.seek(SeekFrom::Start(start)).await?;
    let mut buf = vec![0u8; (end - start) as usize];
    file.read_exact(&mut buf).await?;
    let content = String::from_utf8_lossy(&buf);
    Ok(render_markdown(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_viewport() {
        use std::io::Write;
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path();
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "# Title\ntext").unwrap();
        let res = render_viewport(path, 0, 9).await.unwrap();
        assert!(!res.is_empty());
    }
}
pub fn placeholder() {}
