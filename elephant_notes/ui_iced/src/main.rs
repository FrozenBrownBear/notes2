use iced::{Application, executor, Command, Element, Settings, Theme, widget::Text};

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
    App::run(Settings::default())
}
