//! Interactive markup parsing for widgets.

/// Simple widget types produced by the parser.
#[derive(Debug, PartialEq, Eq)]
pub enum Widget {
    Toggle { key: String },
}

/// Parse markers of the form `/option_<key>_toggle/`.
pub fn parse_tag(input: &str) -> Option<Widget> {
    let body = input.strip_prefix("/option_")?.strip_suffix('/')?;
    let (key, rest) = body.split_once('_')?;
    if rest != "toggle" || key.is_empty() {
        return None;
    }
    Some(Widget::Toggle { key: key.to_string() })
}
