#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CameraError {
    #[error("Device is not a capture device")]
    NotACaptureDevice,

    #[error("Failed to enumerate devices: {0}")]
    EnumerateDevices(#[from] std::io::Error),

    #[error("Invalid device path: {0}")]
    InvalidDevicePath(std::path::PathBuf),

    #[error("Failed to open device: {0}")]
    OpenDevice(std::io::Error),

    #[error("Failed to query capabilities: {0}")]
    QueryCapabilities(std::io::Error),

    #[error("Failed to enumerate formats: {0}")]
    EnumerateFormats(std::io::Error),

    #[error("Failed to set format: {0}")]
    SetFormat(std::io::Error),

    #[error("Failed to start streaming: {0}")]
    StartStreaming(std::io::Error),

    #[error("Failed to read frame: {0}")]
    ReadFrame(std::io::Error),
}
