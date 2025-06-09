/// High level events emitted by the application core.
#[derive(Debug, Clone)]
pub enum Message {
    /// Reload application settings.
    ReloadSettings,
    /// An action from the sidebar component.
    SidebarAction(SidebarAction),
}

/// Actions that originate from the sidebar UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarAction {
    Home,
    Search,
    Add,
    Back,
    Settings,
    Toggle(bool),
}

/// Trait for sinks that accept [`Message`]s emitted by the application.
pub trait EventSink {
    fn send(&self, msg: Message);
}
