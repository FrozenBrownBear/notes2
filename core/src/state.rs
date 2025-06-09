/// Different application states during runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    /// Application is starting up.
    Initializing,
    /// Application is actively running.
    Running,
    /// Application is shutting down.
    ShuttingDown,
}
