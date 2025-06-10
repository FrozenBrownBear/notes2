use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode};
use ratatui::prelude::*;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, List, ListItem};

use core_notes::events::{EventSink, Message, SidebarAction};

/// Pixel art icons used in the sidebar.
pub const ICON_HOME_ART: &[&str] = &[" /\\ ", "/__\\", "|  |", "|__|"];

pub const ICON_SEARCH_ART: &[&str] = &[" /\\ ", "(  )", " \\/ ", " /  "];

pub const ICON_ADD_ART: &[&str] = &["  +  ", " +++ ", "  +  "];

pub const ICON_SETTINGS_ART: &[&str] = &["[===]", "| o |", "[===]"];

pub const ICON_BACK_ART: &[&str] = &["<--"];

/// Simple trait for a file explorer to allow the sidebar to trigger actions.
pub trait FileExplorer {
    fn go_home(&mut self);
    fn new_note(&mut self);
    fn search(&mut self);
    fn go_back(&mut self);
    fn settings(&mut self);
}

/// Sidebar component state.
pub struct Sidebar {
    pub open: bool,
    last_interaction: Instant,
    /// Milliseconds of inactivity before the sidebar automatically folds.
    pub auto_fold_ms: Option<u64>,
}

impl Sidebar {
    pub fn new(auto_fold_ms: Option<u64>) -> Self {
        Self {
            open: false,
            last_interaction: Instant::now(),
            auto_fold_ms,
        }
    }

    /// Handle a key or other event. Emits [`Message::SidebarAction`] using the
    /// provided [`EventSink`] whenever an action is triggered.
    pub fn handle_event<E: EventSink, F: FileExplorer>(
        &mut self,
        ev: &Event,
        sink: &E,
        explorer: &mut F,
    ) {
        match ev {
            Event::Key(k) => match k.code {
                KeyCode::Char('h') => {
                    explorer.go_home();
                    sink.send(Message::SidebarAction(SidebarAction::Home));
                }
                KeyCode::Char('f') => {
                    explorer.search();
                    sink.send(Message::SidebarAction(SidebarAction::Search));
                }
                KeyCode::Char('n') => {
                    explorer.new_note();
                    sink.send(Message::SidebarAction(SidebarAction::Add));
                }
                KeyCode::Char('b') => {
                    explorer.go_back();
                    sink.send(Message::SidebarAction(SidebarAction::Back));
                }
                KeyCode::Char('s') => {
                    explorer.settings();
                    sink.send(Message::SidebarAction(SidebarAction::Settings));
                }
                KeyCode::Tab => {
                    self.open = !self.open;
                    sink.send(Message::SidebarAction(SidebarAction::Toggle(self.open)));
                }
                _ => {}
            },
            _ => {}
        }
        self.last_interaction = Instant::now();
    }

    /// Should be called periodically to update automatic folding behaviour.
    pub fn tick(&mut self) {
        if self.open {
            if let Some(ms) = self.auto_fold_ms {
                if self.last_interaction.elapsed() >= Duration::from_millis(ms) {
                    self.open = false;
                }
            }
        }
    }

    /// Render the sidebar widget.
    pub fn view(&self) -> List<'static> {
        fn item(lines: &[&str], label: &str) -> ListItem<'static> {
            let mut text: Vec<Line<'static>> =
                lines.iter().map(|&l| Line::from(l.to_string())).collect();
            text.push(Line::from(label.to_string()));
            ListItem::new(Text::from(text))
        }

        let items = vec![
            item(ICON_HOME_ART, "home"),
            item(ICON_SEARCH_ART, "search"),
            item(ICON_ADD_ART, "new"),
            item(ICON_SETTINGS_ART, "settings"),
            item(ICON_BACK_ART, "back"),
        ];
        List::new(items).block(Block::default().borders(Borders::ALL).title("Sidebar"))
    }
}
