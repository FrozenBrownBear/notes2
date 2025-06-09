use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum InteractiveTag {
    Option { key: String, typ: String },
}

/// Parse an interactive marker of the form `/option_<key>_<type>/`.
/// Returns `Some(InteractiveTag)` if the input matches the expected
/// format, otherwise `None`.
pub fn parse_tag(tag: &str) -> Option<InteractiveTag> {
    // anchors ensure the entire string is matched
    let re = Regex::new(r"^/option_([^/_]+)_([^/_]+)/$").unwrap();
    re.captures(tag).map(|caps| InteractiveTag::Option {
        key: caps[1].to_string(),
        typ: caps[2].to_string(),
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
