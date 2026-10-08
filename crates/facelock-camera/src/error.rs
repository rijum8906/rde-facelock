#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CameraError {
    #[error("device path has no videoN index: {0}")]
    InvalidDevicePath(std::path::PathBuf),

    #[error("device is not a video capture device")]
    NotACaptureDevice,

    #[error("no cameras found on the system")]
    NotFound,

    #[error("camera is busy (already in use)")]
    Busy,

    #[error("permission denied opening camera (is the user in the `video` group?)")]
    PermissionDenied,

    #[error("requested format or resolution not supported")]
    UnsupportedFormat,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
