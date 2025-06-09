use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "gui")]
    mode: Mode,
}

#[derive(Copy, Clone, ValueEnum)]
enum Mode {
    Gui,
    Tui,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.mode {
        Mode::Gui => ui_iced::run().map_err(|e| e.into()),
        Mode::Tui => tui_editor::run(),
    }
}
