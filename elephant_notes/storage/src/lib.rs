//! Simple lazy file tree with filesystem watcher.

use std::fs;
use std::path::{Path, PathBuf};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

/// Node in the file tree.
#[derive(Debug, Clone)]
pub struct Node {
    path: PathBuf,
    children: Option<Vec<Node>>,
}

impl Node {
    fn new(path: PathBuf) -> Self {
        Self { path, children: None }
    }

    /// Lazily load children if this node is a directory.
    pub fn expand(&mut self) -> std::io::Result<()> {
        if self.children.is_none() && self.path.is_dir() {
            let mut vec = Vec::new();
            for entry in fs::read_dir(&self.path)? {
                let p = entry?.path();
                vec.push(Node::new(p));
            }
            self.children = Some(vec);
        }
        Ok(())
    }

    /// Borrow child nodes if loaded.
    pub fn children(&self) -> Option<&[Node]> { self.children.as_deref() }
}

/// File tree with watcher.
pub struct FileTree {
    root: Node,
    _watcher: RecommendedWatcher,
}

impl FileTree {
    /// Create new tree watching `root`.
    pub fn new(path: PathBuf) -> notify::Result<Self> {
        let mut watcher = notify::recommended_watcher(|_| {}).unwrap();
        watcher.watch(&path, RecursiveMode::Recursive)?;
        Ok(Self { root: Node::new(path), _watcher: watcher })
    }

    /// Root node.
    pub fn root(&self) -> &Node { &self.root }
}
