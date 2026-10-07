//! Application shutdown.
//!
//! Transition the application state to [`AppState::Stopping`] -> [`AppState::Stopped`] and stop the application.

use crate::app::{App, AppState};

impl App {
    pub async fn stop(&mut self) {
        self.state = AppState::Stopping;

        // stop DBus connection
        let mut guard = self.dbus_conn.lock().await;
        if let Some(conn) = guard.take() {
            match conn.close().await {
                Ok(_) => {
                    tracing::info!("D-Bus connection closed");
                }
                Err(e) => {
                    tracing::error!("Failed to close D-Bus connection: {}", e);
                    tracing::warn!("Trying graceful shutdown");
                    let Some(conn) = guard.take() else {
                        tracing::warn!("No D-Bus connection to shutdown");
                        return;
                    };
                    conn.graceful_shutdown().await;
                }
            }
        }

        self.state = AppState::Stopped;

        tracing::info!("FaceLock Application stopped");
    }
}
