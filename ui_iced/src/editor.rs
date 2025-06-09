use iced::{widget::{Column, Text, Button, Checkbox}, Element};

#[derive(Debug, Clone)]
pub enum InteractiveTag {
    Link(String),
    Checkbox(bool),
    Text(String),
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    Toggle(usize),
}

#[derive(Default)]
pub struct Editor {
    tags: Vec<InteractiveTag>,
}

impl Editor {
    pub fn new() -> Self {
        Self { tags: render_viewport() }
    }

    pub fn update(&mut self, message: EditorMessage) -> iced::Task<EditorMessage> {
        match message {
            EditorMessage::Toggle(i) => {
                if let Some(InteractiveTag::Checkbox(value)) = self.tags.get_mut(i) {
                    *value = !*value;
                }
            }
        }
        iced::Task::none()
    }

    pub fn view(&self) -> Element<EditorMessage> {
        let mut col = Column::new();
        for (i, tag) in self.tags.iter().enumerate() {
            match tag {
                InteractiveTag::Link(url) => {
                    col = col.push(Button::new(Text::new(url)).on_press(EditorMessage::Toggle(i)));
                }
                InteractiveTag::Checkbox(value) => {
                    col = col.push(Checkbox::new("", *value).on_toggle(move |_| EditorMessage::Toggle(i)));
                }
                InteractiveTag::Text(t) => {
                    col = col.push(Text::new(t));
                }
            }
        }
        col.into()
    }
}

fn render_viewport() -> Vec<InteractiveTag> {
    vec![
        InteractiveTag::Text("Example note".into()),
        InteractiveTag::Link("https://example.com".into()),
        InteractiveTag::Checkbox(false),
    ]
}
