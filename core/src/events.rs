pub enum Message {
    /// Reload application settings.
    ReloadSettings,
}

/// Trait for sinks that accept [`Message`]s emitted by the application.
pub trait EventSink {
    fn send(&self, msg: Message);
}
