use std::io;
use std::path::PathBuf;
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}, event::{self, Event, KeyCode}};
use clap::Parser;
use ratatui::{Terminal, prelude::*, widgets::{Block, Borders}};

#[derive(Parser)]
struct Args {
    /// Directory where notes are stored
    #[arg(long)]
    origin: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let origin = args
        .origin
        .or_else(|| std::env::var_os("ELEPHANT_NOTES_ORIGIN").map(PathBuf::from))
        .expect("--origin or ELEPHANT_NOTES_ORIGIN required");
    std::fs::create_dir_all(&origin)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    println!("Using notes directory: {}", origin.display());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut focus_editor = true;
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(9), Constraint::Min(1)].as_ref())
                .split(size);
            f.render_widget(Block::default().borders(Borders::ALL).title("SB"), areas[0]);
            let title = if focus_editor {"Edit"} else {"Preview"};
            f.render_widget(Block::default().borders(Borders::ALL).title(title), areas[1]);
        })?;
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => break,
                Event::Key(k) if k.code == KeyCode::Tab => focus_editor = !focus_editor,
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
