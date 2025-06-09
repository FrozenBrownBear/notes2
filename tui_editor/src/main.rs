use std::fs;
use std::io::{self};
use std::path::Path;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Span, Line};
use tui_textarea::{Input, Key, TextArea};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { theme: "light".into() }
    }
}

fn load_config() -> Config {
    if let Ok(data) = fs::read_to_string("config.yaml") {
        serde_yaml::from_str(&data).unwrap_or_default()
    } else {
        Config::default()
    }
}

fn save_config(cfg: &Config) {
    if let Ok(data) = serde_yaml::to_string(cfg) {
        let _ = fs::write("config.yaml", data);
    }
}

fn transform_heading(line: &str) -> (String, Option<Style>) {
    let trimmed = line.trim_start();
    let mut level = 0;
    for ch in trimmed.chars() {
        if ch == '#' {
            level += 1;
        } else {
            break;
        }
    }
    if level > 0 && trimmed.get(level..level + 1) == Some(" ") {
        let text = trimmed[level + 1..].trim_start();
        let style = Style::default().add_modifier(Modifier::BOLD);
        (format!("H{}: {}", level, text), Some(style))
    } else {
        (line.to_string(), None)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut editor = TextArea::default();
    editor.set_block(Block::default().borders(Borders::ALL).title("Edit"));
    let mut cfg = load_config();
    let mut show_palette = false;
    let mut palette = TextArea::default();
    palette.set_block(Block::default().borders(Borders::ALL).title("Settings"));

    if Path::new("config.yaml").exists() {
        if let Ok(content) = fs::read_to_string("config.yaml") {
            palette.insert_str(&content);
        }
    } else {
        palette.insert_str(&serde_yaml::to_string(&cfg).unwrap());
    }

    loop {
        terminal.draw(|f| {
            let size = f.size();
            if show_palette {
                let area = size;
                let widget = palette.widget();
                f.render_widget(widget, area);
            } else {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(size);
                f.render_widget(editor.widget(), chunks[0]);

                let lines: Vec<Line> = editor
                    .lines()
                    .iter()
                    .map(|l| {
                        let (text, style) = transform_heading(l);
                        match style {
                            Some(s) => Line::from(Span::styled(text, s)),
                            None => Line::from(text),
                        }
                    })
                    .collect();
                let preview = Paragraph::new(lines)
                    .block(Block::default().borders(Borders::ALL).title("Preview"));
                f.render_widget(preview, chunks[1]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    if show_palette {
                        match key.code {
                            KeyCode::Esc => {
                                if let Ok(new_cfg) = serde_yaml::from_str::<Config>(&palette.lines().iter().cloned().collect::<Vec<_>>().join("\n")) {
                                    cfg = new_cfg;
                                    save_config(&cfg);
                                }
                                show_palette = false;
                            }
                            _ => { let _ = palette.input(to_input(key)); },
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                            KeyCode::Char('p') if key.modifiers.is_empty() => {
                                palette = TextArea::default();
                                palette.set_block(Block::default().borders(Borders::ALL).title("Settings"));
                                palette.insert_str(&serde_yaml::to_string(&cfg).unwrap());
                                show_palette = true;
                            }
                            _ => { let _ = editor.input(to_input(key)); },
                        }
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn to_input(ev: crossterm::event::KeyEvent) -> Input {
    match ev.code {
        KeyCode::Backspace => Input { key: Key::Backspace, ctrl: false, alt: false },
        KeyCode::Enter => Input { key: Key::Enter, ctrl: false, alt: false },
        KeyCode::Left => Input { key: Key::Left, ctrl: false, alt: false },
        KeyCode::Right => Input { key: Key::Right, ctrl: false, alt: false },
        KeyCode::Up => Input { key: Key::Up, ctrl: false, alt: false },
        KeyCode::Down => Input { key: Key::Down, ctrl: false, alt: false },
        KeyCode::Home => Input { key: Key::Home, ctrl: false, alt: false },
        KeyCode::End => Input { key: Key::End, ctrl: false, alt: false },
        KeyCode::Delete => Input { key: Key::Delete, ctrl: false, alt: false },
        KeyCode::Char(c) => Input { key: Key::Char(c), ctrl: ev.modifiers.contains(KeyModifiers::CONTROL), alt: ev.modifiers.contains(KeyModifiers::ALT) },
        _ => Input { key: Key::Null, ctrl: false, alt: false },
    }
}
