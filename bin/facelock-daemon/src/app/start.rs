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
    dbus::{
        FaceLockDBusInterface, auth::FaceLockAuthDBusInterface,
        enroll::FaceLockEnrollDBusInterface, manager::FaceLockManagerDBusInterface,
    },
};
use facelock_common::error::FaceLockError;
use tokio::{signal, sync::Mutex};

impl App {
    /// Starts the application.
    ///
    /// - Idempotent: Calling this method on an already running [`App`] will return Ok(())
    /// - Returns [`FaceLockError::Startup`] if the application could not be started.
    pub async fn start(&mut self) -> Result<(), FaceLockError> {
        // Idempotent: Calling this method on an already running [`App`] will return Ok(())
        if self.state == AppState::Running {
            tracing::info!("FaceLock Application is already running");
            return Ok(());
        }

        self.state = AppState::Running;

        // instanciate DBus interface
        let face_lock_dbus_interface = FaceLockDBusInterface::new();
        let face_lock_auth_dbus_interface = FaceLockAuthDBusInterface::new();
        let face_lock_enroll_dbus_interface = FaceLockEnrollDBusInterface::new();
        let face_lock_manager_dbus_interface = FaceLockManagerDBusInterface::new();

        // 1. Establish DBus Session Connection
        // FIXME: Use system connection instead of session connection
        tracing::info!("Establishing D-Bus session connection...");
        let conn = zbus::connection::Builder::system()?
            .name("org.rde.FaceLock")?
            .serve_at("/org/rde/FaceLock", face_lock_dbus_interface)?
            .serve_at("/org/rde/FaceLock/Auth", face_lock_auth_dbus_interface)?
            .serve_at("/org/rde/FaceLock/Enroll", face_lock_enroll_dbus_interface)?
            .serve_at(
                "/org/rde/FaceLock/Manager",
                face_lock_manager_dbus_interface,
            )?
            .build()
            .await
            .map_err(FaceLockError::DBus)?;

        tracing::debug!("D-Bus interface registered at /org/rde/FaceLock");
        tracing::debug!("D-Bus interface registered at /org/rde/FaceLock/Auth");
        tracing::debug!("D-Bus interface registered at /org/rde/FaceLock/Enroll");
        tracing::debug!("D-Bus interface registered at /org/rde/FaceLock/Manager");

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
