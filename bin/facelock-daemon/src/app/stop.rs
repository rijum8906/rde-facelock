//! Application shutdown.
//!
//! Transition the application state to [`AppState::Stopping`] -> [`AppState::Stopped`] and stop the application.

use crate::app::{App, AppState};

impl App {
    pub async fn stop(&mut self) {
        self.state = AppState::Stopping;

        // TODO: Logic to stop the application

        self.state = AppState::Stopped;
    }
}
