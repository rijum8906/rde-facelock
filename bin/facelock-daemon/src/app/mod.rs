//! FaceLock Daemon Application
//!
//! Top-level application container that owns and manages the daemon's
//! lifecycle state (e.g., `Running`, `Stopped`, `Crashed`).
//!
//! This module is responsible for:
//! - Holding the current application state
//! - Driving state transitions (start, stop, restart)
//! - Exposing lifecycle hooks to the rest of the system
//!
//! [`start`] - handles the starting behavior of the application and the logic.
//! [`stop`] - handles the stopping behavior of the application.

pub mod start;
pub mod stop;

/// Lifecycle position of the [`App`]. Answers "where is it in start/stop?"
///
/// Transitions:
///   Stopped  --start()-->  Starting  --ok-->  Running
///   Starting --err-->      Stopped
///   Running  --stop()-->   Stopping  --ok-->  Stopped
///
/// Health (alive? responsive?) is tracked separately — see [`AppHealth`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    /// Not running. Safe to call `start`.
    Stopped,
    /// `start()` in progress, not yet `Running`.
    Starting,
    /// Fully initialized and serving. `stop()` is valid.
    Running,
    /// `stop()` in progress; in-flight work is draining.
    Stopping,
}

/// Runtime health of a running [`App`]. A *measurement*, not a lifecycle state.
///
/// Reported by periodic checks (heartbeat, self-probe, IPC ping).
/// Does not drive lifecycle changes directly — a supervisor observes this
/// and may call `stop`/`restart` in response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppHealth {
    /// Not yet assessed (e.g., just entered `Running`, first probe pending).
    Unknown,
    /// All checks passing.
    Healthy,
    /// Running but degraded — some checks failing, still serving.
    Degraded,
    /// Failing checks and not serving correctly; restart is warranted.
    Unhealthy,
}

/// Represents the top-level application container.
pub struct App {
    state: AppState,
    health: AppHealth,
    started_at: std::time::Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Starting,
            health: AppHealth::Unknown,
            started_at: std::time::Instant::now(),
        }
    }

    /// Returns a reference to the current application state.
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Returns the duration since the application started.
    pub fn uptime(&self) -> std::time::Duration {
        self.started_at.elapsed()
    }

    /// Returns the current health state of the application.
    pub fn health(&self) -> AppHealth {
        self.health
    }
}
