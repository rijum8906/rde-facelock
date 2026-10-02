//! Application startup.
//!
//! Transtition of the [`App`] state from [`AppState::Stopped`] to [`AppState::Running`].
//! Startup is synchronous and blocks until the application is fully started.
//!
//! # Invariants:
//! - Only one instance of [`App`] can be running at a time per process.
//! - Calling [`App::start`] on an already running [`App`] will returns Ok(())
//!
//! # Errors:
//! - [`FaceLockError::Startup`] if the application could not be started.

use crate::app::{App, AppState};
use facelock_common::error::FaceLockError;

impl App {
    /// Starts the application.
    ///
    /// - Idempotent: Calling this method on an already running [`App`] will return Ok(())
    /// - Returns [`FaceLockError::Startup`] if the application could not be started.
    pub async fn start(&mut self) -> Result<(), FaceLockError> {
        self.state = AppState::Running;
        // TODO: Logic to start the application
        Err(FaceLockError::Startup(
            "Application startup failed".to_string(),
        ))
    }
}
