//! Path Module
//!
//! A module for handling paths in the `facelock` ecosystem.
//! This module provides utilities for working with file system paths, including validation and manipulation of paths related to the application.
//! e.g.
//!     - `/dev/v4l/by-id/` and `by-path/` entries for camera devices.
//!     - '/var/lib/facelock' for storing application data.

// NOTE: all the directory paths must have a suffix of `_DIR` to indicate that they are directories, not files.
// all no directory path should end with a trailing slash `/` to avoid confusion with directory paths.

// =================================================================================
// Application Related Paths
// =================================================================================
/// The directory where FaceLock stores its data files.
pub const FACLOCK_DATA_DIR: &str = "/var/lib/facelock";
/// The directory where FaceLock stores its log files.
///
/// log files will be stored datewise in a particular format
pub const FACELOCK_LOG_DIR: &str = "/var/log/facelock";

// =================================================================================
// Camera Related Paths for Linux
// =================================================================================
/// The directory where video devices are located on Linux systems.
pub const V4L_DIR: &str = "/dev/v4l";
/// The directory where video devices are listed by their unique identifiers on Linux systems.
pub const V4L_BY_ID_DIR: &str = "/dev/v4l/by-id";
/// The directory where video devices are listed by their physical connection paths on Linux systems.
pub const V4L_BY_PATH_DIR: &str = "/dev/v4l/by-path";
