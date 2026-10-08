//! Unified Logger System
//!
//! This module provides a unified logging system for the FaceLock service,
//! supporting both console and file logging with configurable log levels.
//! It leverages the `tracing` ecosystem to provide structured logging, colored output, and runtime log level changes.

use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

use tracing::Level;
use tracing_subscriber::{
    EnvFilter, Registry, fmt, layer::SubscriberExt, reload, util::SubscriberInitExt,
};

use crate::error::{FaceLockError, Result};

/// Specifies the minimum log level emitted by the logger.
#[derive(Clone, Copy, Debug, Default)]
pub enum LogLevel {
    /// Enables debug messages and all higher-severity messages.
    Debug,

    /// Enables informational messages and all higher-severity messages.
    #[default]
    Info,

    /// Enables warning messages and errors.
    Warn,

    /// Enables only error messages.
    Error,
}

impl LogLevel {
    /// Returns the level as a string understood by `tracing_subscriber`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }

    /// Converts this level to a `tracing` level.
    pub const fn as_tracing_level(self) -> Level {
        match self {
            Self::Debug => Level::DEBUG,
            Self::Info => Level::INFO,
            Self::Warn => Level::WARN,
            Self::Error => Level::ERROR,
        }
    }
}

/// Configures and manages logging for a FaceLock service.
///
/// The logger provides:
///
/// - Colored console output.
/// - Daily rotating log files.
/// - `RUST_LOG` environment filtering.
/// - Runtime log-level changes.
/// - Non-blocking file logging.
#[derive(Debug)]
pub struct Logger {
    /// Directory where log files are stored.
    log_dir: PathBuf,

    /// Minimum log level used when `RUST_LOG` is not set.
    level: LogLevel,

    /// Name of the service used for the log file.
    service_name: String,
}

type ReloadHandle = reload::Handle<EnvFilter, Registry>;

struct LoggerState {
    reload_handle: ReloadHandle,

    // Must remain alive for the lifetime of the application.
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

static LOGGER_STATE: OnceLock<LoggerState> = OnceLock::new();

impl Logger {
    /// Creates a new logger configuration.
    pub fn new(
        level: LogLevel,
        log_dir: impl Into<PathBuf>,
        service_name: impl Into<String>,
    ) -> Self {
        Self {
            log_dir: log_dir.into(),
            level,
            service_name: service_name.into(),
        }
    }

    /// Initializes the global tracing subscriber.
    ///
    /// This method may only successfully initialize the logger once.
    ///
    /// If `RUST_LOG` is set, it takes precedence over the configured
    /// [`LogLevel`].
    pub fn init(&self) -> Result<()> {
        std::fs::create_dir_all(&self.log_dir).map_err(FaceLockError::Io)?;

        if LOGGER_STATE.get().is_some() {
            return Err(FaceLockError::Internal(
                "Logger has already been initialized".into(),
            ));
        }

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(self.level.as_str()));

        let (reload_layer, reload_handle) = reload::Layer::new(filter);

        let console_layer = fmt::layer()
            .with_writer(std::io::stdout)
            .with_ansi(true)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(false)
            .with_thread_names(false);

        let file_appender = tracing_appender::rolling::daily(&self.log_dir, &self.service_name);

        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .with_ansi(false)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(false)
            .with_thread_names(false);

        let subscriber = Registry::default()
            .with(reload_layer)
            .with(console_layer)
            .with(file_layer);

        subscriber.try_init().map_err(|error| {
            FaceLockError::Internal(format!("Failed to initialize logger: {error}"))
        })?;

        LOGGER_STATE
            .set(LoggerState {
                reload_handle,
                _guard: guard,
            })
            .map_err(|_| {
                FaceLockError::Internal("Logger state has already been initialized".into())
            })?;

        Ok(())
    }

    /// Changes the global log level at runtime.
    pub fn change_level(level: LogLevel) -> Result<()> {
        let state = LOGGER_STATE
            .get()
            .ok_or_else(|| FaceLockError::Internal("Logger has not been initialized".into()))?;

        let filter = EnvFilter::new(level.as_str());

        state.reload_handle.reload(filter).map_err(|error| {
            FaceLockError::Internal(format!("Failed to change log level: {error}"))
        })?;

        Ok(())
    }

    /// Returns the configured log directory.
    pub fn log_dir(&self) -> &Path {
        &self.log_dir
    }

    /// Returns the configured service name.
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    /// Returns the configured default log level.
    pub const fn level(&self) -> LogLevel {
        self.level
    }
}
