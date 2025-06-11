use clap::{Parser, ValueEnum};
use crossterm::{event, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, prelude::*, widgets::{Block, Borders, List, ListItem}};
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    mode: Option<Mode>,
}

#[derive(Copy, Clone, ValueEnum)]
enum Mode {
    Gui,
    Tui,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mode = match args.mode {
        Some(m) => m,
        None => prompt_mode()?,
    };
    match mode {
        Mode::Gui => ui_iced::run().map_err(|e| e.into()),
        Mode::Tui => tui_editor::run(),
    }
}

fn prompt_mode() -> Result<Mode, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut index: usize = 0;
    let options = ["GUI", "TUI"];
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let items: Vec<_> = options
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    let style = if i == index {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    };
                    ListItem::new(*text).style(style)
                })
                .collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Select interface"));
            f.render_widget(list, size);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Up => {
                    if index > 0 { index -= 1; }
                }
                event::KeyCode::Down => {
                    if index + 1 < options.len() { index += 1; }
                }
                event::KeyCode::Enter => break,
                event::KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    return Err("aborted".into());
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(match index {
        0 => Mode::Gui,
        _ => Mode::Tui,
    })
}
