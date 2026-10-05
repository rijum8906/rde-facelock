//! Unified error handling types for the `facelock` ecosystem.
//!
//! This module provides [`FaceLockError`], which encapsulates domain errors,
//! IPC failures, device errors, and hardware security module faults using `thiserror`.

/// The unified error type for facelock.
///
/// This error type is used throughout the facelock crates to provide a consistent error handling mechanism.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum FaceLockError {
    /// An internal error occurred.
    #[error("Internal error: {0}")]
    Internal(String),

    /// An I/O operation failed.
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    /// A startup error occurred.
    ///
    /// This error is returned when the application fails to start up properly.
    #[error("Startup error: {0}")]
    Startup(String),

    /// When DBus error occured
    ///
    /// Used when there is issue with DBus connection
    #[error("DBus error: {0}")]
    DBus(#[from] zbus::Error),
}

/// Convenience alias for `Result<T, FaceLockError>`.
pub type Result<T, E = FaceLockError> = std::result::Result<T, E>;
