//! Configuration handling using YAML front matter.

use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use core::{Message};

/// Available UI themes.
#[derive(Clone, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

/// Settings loaded from disk.
#[derive(Clone, Deserialize)]
pub struct Settings {
    pub auto_fold_ms: u64,
    pub theme: Theme,
}

/// Load settings from a file with YAML front matter.
pub fn load(path: &Path) -> Result<Settings> {
    let content = fs::read_to_string(path)?;
    let doc = YamlFrontMatter::parse::<Settings>(&content)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(doc.metadata)
}

/// Watch the given file and send [`Message::ReloadSettings`] when it changes.
pub fn watch(path: &Path, tx: Sender<Message>) {
    let path = path.to_path_buf();
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |_| {
        let _ = tx.send(Message::ReloadSettings);
    }).expect("watcher");
    watcher.watch(&path, RecursiveMode::NonRecursive).ok();
    std::mem::forget(watcher);
}
