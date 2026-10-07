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

use std::sync::Arc;

use crate::{
    app::{App, AppState},
    dbus::FaceLockDBusInterface,
};
use facelock_common::error::FaceLockError;
use tokio::{signal, sync::Mutex};

impl App {
    /// Starts the application.
    ///
    /// - Idempotent: Calling this method on an already running [`App`] will return Ok(())
    /// - Returns [`FaceLockError::Startup`] if the application could not be started.
    pub async fn start(&mut self) -> Result<(), FaceLockError> {
        self.state = AppState::Running;

        // instanciate DBus interface
        let interface = FaceLockDBusInterface::new();

        // 1. Establish DBus Session Connection
        // FIXME: Use system connection instead of session connection
        tracing::info!("Establishing D-Bus session connection...");
        let conn = zbus::connection::Builder::session()?
            .name("org.rde.FaceLock")?
            .serve_at("/org/rde/FaceLock", interface)?
            .build()
            .await
            .map_err(FaceLockError::DBus)?;

        tracing::debug!("D-Bus interface registered at /org/rde/FaceLock");

        conn.request_name("org.rde.FaceLock").await?;
        tracing::info!("FaceLock D-Bus service started successfully");

        // Store the connection in the application state
        self.dbus_conn = Arc::new(Mutex::new(Some(conn)));

        // Wait for Ctrl+C to exit
        tracing::info!("Waiting for Ctrl+C signal to shutdown...");
        signal::ctrl_c().await?;

        tracing::info!("Ctrl+C signal received. Shutting down FaceLock Application...");
        self.stop().await;

        Ok(())
    }
}
