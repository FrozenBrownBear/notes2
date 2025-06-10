#[derive(Debug, PartialEq, Eq)]
pub enum InteractiveTag {
    Option { key: String, typ: String },
}

/// Parse an interactive marker of the form `/option_<key>_<type>/`.
/// Returns `Some(InteractiveTag)` if the input matches the expected
/// format, otherwise `None`.
pub fn parse_tag(tag: &str) -> Option<InteractiveTag> {
    let input = tag.strip_prefix("/option_")?.strip_suffix('/')?;
    let mut parts = input.splitn(2, '_');
    let key = parts.next()?;
    let typ = parts.next()?;
    if key.is_empty()
        || typ.is_empty()
        || key.contains('/')
        || key.contains('_')
        || typ.contains('/')
        || typ.contains('_')
    {
        return None;
    }
    Some(InteractiveTag::Option {
        key: key.to_string(),
        typ: typ.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_option() {
        let tag = "/option_volume_int/";
        let parsed = parse_tag(tag);
        assert_eq!(
            parsed,
            Some(InteractiveTag::Option {
                key: "volume".to_string(),
                typ: "int".to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_slashes() {
        assert_eq!(parse_tag("option_volume_int"), None);
    }

    #[test]
    fn rejects_unexpected_format() {
        assert_eq!(parse_tag("/opt_volume_int/"), None);
    }

    #[test]
    fn rejects_empty_parts() {
        assert_eq!(parse_tag("/option__int/"), None);
        assert_eq!(parse_tag("/option_volume_/"), None);
    }
}
