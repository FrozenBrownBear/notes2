use iced::{application, widget::Column, Subscription, time, Task};
use crossbeam_channel::{unbounded, Receiver, Sender};

mod sidebar;
mod editor;

use sidebar::{Sidebar, SidebarMessage};
use editor::{Editor, EditorMessage};

#[derive(Debug, Clone)]
enum CoreEvent {
    Dummy,
}

struct EventBus {
    sender: Sender<CoreEvent>,
}

impl EventBus {
    fn new() -> (Self, Receiver<CoreEvent>) {
        let (tx, rx) = unbounded();
        (Self { sender: tx }, rx)
    }
}

struct App {
    sidebar: Sidebar,
    editor: Editor,
    event_bus: EventBus,
    event_rx: Receiver<CoreEvent>,
}

#[derive(Debug, Clone)]
enum Message {
    Sidebar(SidebarMessage),
    Editor(EditorMessage),
    Tick,
}

fn init() -> (App, Task<Message>) {
    let (bus, rx) = EventBus::new();
    (
        App {
            sidebar: Sidebar::new(),
            editor: Editor::new(),
            event_bus: bus,
            event_rx: rx,
        },
        Task::none(),
    )
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Sidebar(msg) => app.sidebar.update(msg).map(Message::Sidebar),
        Message::Editor(msg) => app.editor.update(msg).map(Message::Editor),
        Message::Tick => {
            app.sidebar.tick();
            while let Ok(event) = app.event_rx.try_recv() {
                match event {
                    CoreEvent::Dummy => {}
                }
            }
            Task::none()
        }
    }
}

fn view(app: &App) -> Column<Message> {
    Column::new()
        .push(app.sidebar.view().map(Message::Sidebar))
        .push(app.editor.view().map(Message::Editor))
}

fn subscription(app: &App) -> Subscription<Message> {
    let tick = time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick);
    Subscription::batch(vec![tick, app.sidebar.subscription().map(Message::Sidebar)])
}

fn main() -> iced::Result {
    application("notes2", update, view)
        .subscription(subscription)
        .run_with(init)
}
