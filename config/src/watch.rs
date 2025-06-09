use std::path::Path;
use std::time::Duration;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_mini::new_debouncer;

/// Message types emitted on the event bus.
#[derive(Debug, Clone)]
pub enum Message {
    ReloadSettings,
}

/// Simple trait for an event bus.
pub trait EventBus {
    fn publish(&self, msg: Message);
}

/// Watch the configuration file at `path` and publish [`Message::ReloadSettings`]
/// whenever it is modified. Uses a 500ms debounce to coalesce rapid changes.
pub fn watch_settings<B: EventBus + Send + 'static>(path: &Path, bus: B) -> notify::Result<()> {
    let path = path.to_path_buf();
    // Create a debouncer with 500ms delay.
    let mut debouncer = new_debouncer(Duration::from_millis(500), None, move |res| {
        match res {
            Ok(_events) => {
                bus.publish(Message::ReloadSettings);
            }
            Err(e) => eprintln!("watch error: {e}"),
        }
    })?;

    debouncer.watcher().watch(&path, RecursiveMode::NonRecursive)?;
    // Keep the debouncer running
    std::mem::forget(debouncer);
    Ok(())
}
