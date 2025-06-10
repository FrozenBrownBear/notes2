use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use notify::{RecursiveMode, Watcher};

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
    let last = Arc::new(Mutex::new(Instant::now() - Duration::from_millis(500)));
    let last_ev = last.clone();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(_e) => {
            let mut last = last_ev.lock().unwrap();
            if last.elapsed() >= Duration::from_millis(500) {
                bus.publish(Message::ReloadSettings);
                *last = Instant::now();
            }
        }
        Err(e) => eprintln!("watch error: {e}"),
    })?;

    watcher.watch(&path, RecursiveMode::NonRecursive)?;
    std::mem::forget(watcher);
    Ok(())
}
