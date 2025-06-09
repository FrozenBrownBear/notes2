use iced::{widget::{Container, Column, Text}, Length, Element, Subscription, event, Event, touch};

const WIDTH: u16 = 220;
const SPEED: f32 = 0.1;

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    Toggle,
    SwipeLeft,
    SwipeRight,
}

#[derive(Default)]
pub struct Sidebar {
    open: bool,
    progress: f32,
}

impl Sidebar {
    pub fn new() -> Self {
        Self { open: false, progress: 0.0 }
    }

    pub fn update(&mut self, msg: SidebarMessage) -> iced::Task<SidebarMessage> {
        match msg {
            SidebarMessage::Toggle => {
                self.open = !self.open;
            }
            SidebarMessage::SwipeLeft => {
                self.open = false;
            }
            SidebarMessage::SwipeRight => {
                self.open = true;
            }
        }
        iced::Task::none()
    }

    pub fn tick(&mut self) {
        let target = if self.open { 1.0 } else { 0.0 };
        if (self.progress - target).abs() > f32::EPSILON {
            if self.progress < target {
                self.progress = (self.progress + SPEED).min(1.0);
            } else {
                self.progress = (self.progress - SPEED).max(0.0);
            }
        }
    }

    pub fn view(&self) -> Element<SidebarMessage> {
        let content = Column::new().push(Text::new("Sidebar"));
        Container::new(content)
            .width(Length::Fixed(WIDTH as f32 * self.progress))
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<SidebarMessage> {
        event::listen_with(|event, status, _| {
            use iced::Point;
            thread_local! {
                static LAST: std::cell::RefCell<Option<(u64, Point)>> = std::cell::RefCell::new(None);
            }

            if status == event::Status::Ignored {
                match event {
                    Event::Touch(touch::Event::FingerPressed { id, position }) => {
                        LAST.with(|l| *l.borrow_mut() = Some((id.0, position)));
                    }
                    Event::Touch(touch::Event::FingerMoved { id, position }) => {
                        return LAST.with(|l| {
                            if let Some((fid, start)) = *l.borrow() {
                                if fid == id.0 {
                                    let delta_x = position.x - start.x;
                                    if delta_x > 30.0 {
                                        *l.borrow_mut() = None;
                                        return Some(SidebarMessage::SwipeRight);
                                    } else if delta_x < -30.0 {
                                        *l.borrow_mut() = None;
                                        return Some(SidebarMessage::SwipeLeft);
                                    }
                                }
                            }
                            None
                        });
                    }
                    Event::Touch(touch::Event::FingerLifted { .. })
                    | Event::Touch(touch::Event::FingerLost { .. }) => {
                        LAST.with(|l| *l.borrow_mut() = None);
                    }
                    _ => {}
                }
            }
            None
        })
    }
}

