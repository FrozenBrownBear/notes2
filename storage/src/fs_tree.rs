use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc::{channel, Receiver},
};

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

/// Represents a node in the file tree.
#[derive(Debug, Clone)]
pub struct Node {
    path: PathBuf,
    metadata: fs::Metadata,
    children: Option<Vec<Node>>, // Lazily loaded
    is_symlink: bool,
}

impl Node {
    fn new(path: PathBuf, metadata: fs::Metadata) -> Self {
        let is_symlink = metadata.file_type().is_symlink();
        Self {
            path,
            metadata,
            children: None,
            is_symlink,
        }
    }

    /// Path of this node.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Metadata of this node.
    pub fn metadata(&self) -> &fs::Metadata {
        &self.metadata
    }

    /// Whether this node is a directory.
    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    /// Whether this node is a symbolic link.
    pub fn is_symlink(&self) -> bool {
        self.is_symlink
    }

    /// Get the lazily loaded children of this node.
    pub fn children(&self) -> Option<&[Node]> {
        self.children.as_deref()
    }
}

/// A lazy loading file tree that watches the filesystem for updates.
pub struct FileTree {
    root: Node,
    _watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>, // receives filesystem events
}

impl FileTree {
    /// Create a new `FileTree` rooted at `path` and start watching for changes.
    pub fn new(path: PathBuf) -> notify::Result<Self> {
        let md = fs::symlink_metadata(&path)?;
        let root = Node::new(path.clone(), md);

        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            // Ignore send errors if receiver was dropped
            let _ = tx.send(res);
        })?;
        watcher.watch(&path, RecursiveMode::Recursive)?;

        Ok(Self {
            root,
            _watcher: watcher,
            rx,
        })
    }

    /// Returns the root node.
    pub fn root(&self) -> &Node {
        &self.root
    }

    /// Returns mutable reference to root node.
    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }

    /// Expand the given node if it is a directory. Does nothing if already loaded.
    pub fn expand(node: &mut Node) -> std::io::Result<()> {
        if node.is_dir() && node.children.is_none() {
            let mut children = Vec::new();
            for entry in fs::read_dir(&node.path)? {
                let entry = entry?;
                let p = entry.path();
                let md = fs::symlink_metadata(&p)?;
                children.push(Node::new(p, md));
            }
            node.children = Some(children);
        }
        Ok(())
    }

    /// Poll the watcher for filesystem events and update cached nodes.
    pub fn poll_events(&mut self) {
        while let Ok(res) = self.rx.try_recv() {
            if let Ok(event) = res {
                self.process_event(event);
            }
        }
    }

    fn process_event(&mut self, event: Event) {
        for path in event.paths {
            self.invalidate(&path);
        }
    }

    fn invalidate(&mut self, path: &Path) {
        Self::invalidate_node(&mut self.root, path);
    }

    fn invalidate_node(node: &mut Node, path: &Path) -> bool {
        if path == node.path {
            node.children = None;
            return true;
        }
        if let Some(children) = node.children.as_mut() {
            for child in children {
                if path.starts_with(&child.path) {
                    if Self::invalidate_node(child, path) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
