use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

/// Message emitted by [`FileExplorer`] when user interacts with the tree.
#[derive(Debug, Clone)]
pub enum ExplorerMessage {
    /// Request to open the file at the given path.
    Open(PathBuf),
    /// Request to pin the file at the given path.
    Pin(PathBuf),
}

/// A node in the [`FileTree`].
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
    pub expanded: bool,
}

impl FileNode {
    pub fn new_file(name: impl Into<String>, path: PathBuf) -> Self {
        Self { name: name.into(), path, is_dir: false, children: Vec::new(), expanded: false }
    }

    pub fn new_dir(name: impl Into<String>, path: PathBuf) -> Self {
        Self { name: name.into(), path, is_dir: true, children: Vec::new(), expanded: false }
    }
}

/// Tree of files shown in the explorer.
#[derive(Debug, Clone)]
pub struct FileTree {
    pub root: FileNode,
}

impl FileTree {
    pub fn new(root: FileNode) -> Self {
        Self { root }
    }
}

#[derive(Clone, Debug)]
struct VisibleNode {
    path: Vec<usize>,
    name: String,
    is_dir: bool,
    expanded: bool,
    indent: usize,
    full_path: PathBuf,
}

/// Interactive file explorer widget.
#[derive(Debug)]
pub struct FileExplorer {
    tree: FileTree,
    state: ListState,
    visible: Vec<VisibleNode>,
    last_click: Option<(Instant, usize)>,
    press_start: Option<(Instant, usize)>,
}

impl FileExplorer {
    pub fn new(tree: FileTree) -> Self {
        Self {
            tree,
            state: ListState::default(),
            visible: Vec::new(),
            last_click: None,
            press_start: None,
        }
    }

    fn collect(nodes: &[FileNode], prefix: &mut Vec<usize>, indent: usize, out: &mut Vec<VisibleNode>) {
        for (i, node) in nodes.iter().enumerate() {
            prefix.push(i);
            out.push(VisibleNode {
                path: prefix.clone(),
                name: node.name.clone(),
                is_dir: node.is_dir,
                expanded: node.expanded,
                indent,
                full_path: node.path.clone(),
            });
            if node.is_dir && node.expanded {
                Self::collect(&node.children, prefix, indent + 1, out);
            }
            prefix.pop();
        }
    }

    fn refresh_visible(&mut self) {
        self.visible.clear();
        let mut prefix = Vec::new();
        Self::collect(std::slice::from_ref(&self.tree.root), &mut prefix, 0, &mut self.visible);
    }

    fn get_mut(&mut self, path: &[usize]) -> Option<&mut FileNode> {
        let mut node = &mut self.tree.root;
        for &idx in path.iter().skip(1) {
            node = node.children.get_mut(idx)?;
        }
        Some(node)
    }

    /// Draw the explorer in the given area.
    pub fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.refresh_visible();
        let items: Vec<ListItem> = self
            .visible
            .iter()
            .map(|v| {
                let indent = "  ".repeat(v.indent);
                let icon = if v.is_dir {
                    if v.expanded { "▾" } else { "▸" }
                } else {
                    " "
                };
                ListItem::new(format!("{}{} {}", indent, icon, v.name))
            })
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Files"));
        f.render_stateful_widget(list, area, &mut self.state);
    }

    /// Handle an input event. Returns a message if one was produced.
    pub fn handle_event(&mut self, ev: &Event) -> Option<ExplorerMessage> {
        match ev {
            Event::Mouse(me) => match me.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    let idx = self.state.selected().unwrap_or(0);
                    self.last_click = match self.last_click.take() {
                        Some((inst, last_idx)) if last_idx == idx && inst.elapsed() < Duration::from_millis(500) => {
                            if let Some(node) = self.visible.get(idx) {
                                if node.is_dir {
                                    let path = node.path.clone();
                                    if let Some(n) = self.get_mut(&path) { n.expanded = !n.expanded; }
                                } else {
                                    return Some(ExplorerMessage::Open(node.full_path.clone()));
                                }
                            }
                            None
                        }
                        _ => Some((Instant::now(), idx)),
                    };
                    self.press_start = Some((Instant::now(), idx));
                }
                MouseEventKind::Up(MouseButton::Left) => {
                    if let Some((start, idx)) = self.press_start.take() {
                        if start.elapsed() >= Duration::from_millis(800) {
                            if let Some(node) = self.visible.get(idx) {
                                if !node.is_dir {
                                    return Some(ExplorerMessage::Pin(node.full_path.clone()));
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::Key(key) => match key.code {
                KeyCode::Up => {
                    let i = self.state.selected().unwrap_or(0);
                    self.state.select(Some(i.saturating_sub(1)));
                }
                KeyCode::Down => {
                    let i = self.state.selected().unwrap_or(0);
                    let max = self.visible.len().saturating_sub(1);
                    let next = (i + 1).min(max);
                    self.state.select(Some(next));
                }
                KeyCode::Right => {
                    if let Some(idx) = self.state.selected() {
                        if let Some(node) = self.visible.get(idx) {
                            if node.is_dir && !node.expanded {
                                let path = node.path.clone();
                                if let Some(n) = self.get_mut(&path) { n.expanded = true; }
                            }
                        }
                    }
                }
                KeyCode::Left => {
                    if let Some(idx) = self.state.selected() {
                        if let Some(node) = self.visible.get(idx) {
                            if node.is_dir && node.expanded {
                                let path = node.path.clone();
                                if let Some(n) = self.get_mut(&path) { n.expanded = false; }
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        None
    }
}

