use iced::{Application, executor, Command, Element, Settings, Theme, widget::Text};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Directory where notes are stored
    #[arg(long)]
    origin: Option<PathBuf>,
}

struct App;

impl Application for App {
    type Executor = executor::Default;
    type Message = (); 
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) { (Self, Command::none()) }

    fn title(&self) -> String { String::from("Elephant Notes") }

    fn update(&mut self, _msg: Self::Message) -> Command<Self::Message> { Command::none() }

    fn view(&self) -> Element<'_, Self::Message> { Text::new("Elephant Notes").into() }
}

fn main() -> iced::Result {
    let args = Args::parse();
    let origin = args
        .origin
        .or_else(|| std::env::var_os("ELEPHANT_NOTES_ORIGIN").map(PathBuf::from))
        .expect("--origin or ELEPHANT_NOTES_ORIGIN required");
    std::fs::create_dir_all(&origin).expect("create origin");
    println!("Using notes directory: {}", origin.display());
    App::run(Settings::default())
}
