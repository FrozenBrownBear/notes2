use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::Error;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Settings {
    /// Automatically fold the sidebar after this many milliseconds of
    /// inactivity. A value of `None` disables auto folding.
    #[serde(default)]
    pub sidebar_auto_fold_ms: Option<u64>,
}

/// Load the YAML front matter from the file at `path` into a [`Settings`] struct.
///
/// This function expects the file to begin with a YAML front matter block delimited by
/// triple dashes (`---`). Everything between the first two such markers is parsed as YAML.
pub fn load_settings(path: &Path) -> Result<Settings, Error> {
    let content = fs::read_to_string(path)?;
    let yaml = if let Some(rest) = content.strip_prefix("---") {
        if let Some(end) = rest.find("---") {
            &rest[..end]
        } else {
            rest
        }
    } else {
        content.as_str()
    };
    let settings: Settings = serde_yaml::from_str(yaml)?;
    Ok(settings)
}
pub fn placeholder() {}
