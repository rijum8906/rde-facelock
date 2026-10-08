//! Camera device abstraction for the Facelock project.

use std::time::Duration;

/// Stable-ish identifier for a camera on the system.
///
/// Prefer `by_id` over `index` when persisting a choice, since `/dev/videoN`
/// numbering is not stable across reboots or hotplug.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceId {
    /// `/dev/videoN`
    Index(u32),
    /// `/dev/v4l/by-id/usb-...-video-index0`
    ById(String),
    /// `/dev/v4l/by-path/...`
    ByPath(String),
}

impl DeviceId {
    /// Convert the device ID to a path string.
    pub fn to_path(&self) -> String {
        match self {
            DeviceId::Index(n) => format!("/dev/video{}", n),
            DeviceId::ById(id) => format!("/dev/v4l/by-id/{}", id),
            DeviceId::ByPath(path) => format!("/dev/v4l/by-path/{}", path),
        }
    }
}

/// Information about a camera device, including its supported formats.
#[derive(Debug, Clone)]
pub struct CameraInfo {
    /// Stable identifier for the camera device.
    pub id: DeviceId,
    /// Human-readable name of the camera device.
    pub name: String,
    /// Vendor string, if available from VIDIOC_QUERYCAP.
    pub vendor: Option<String>,
    /// Supported pixel formats for the camera device.
    pub formats: Vec<PixelFormat>,
}

/// Pixel format of a camera device. This is a subset of the formats supported by v4l2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Yuyv,  // YUYV 4:2:2
    Mjpeg, // Motion JPEG
    Nv12,
    Rgb24,
    Gray,
    // TODO: store is as raw [u8, 4] and implement Display to show the FOURCC string
    Other(u32), // raw FOURCC
}

impl From<v4l::FourCC> for PixelFormat {
    fn from(fourcc: v4l::FourCC) -> Self {
        match fourcc.str().unwrap_or_default() {
            "YUYV" => PixelFormat::Yuyv,
            "MJPG" => PixelFormat::Mjpeg,
            "NV12" => PixelFormat::Nv12,
            "RGB3" => PixelFormat::Rgb24,
            "GREY" => PixelFormat::Gray,
            _ => Self::Other(fourcc.into()),
        }
    }
}

/// Convert a v4l2 format description to our PixelFormat enum.
impl From<v4l::format::Description> for PixelFormat {
    fn from(desc: v4l::format::Description) -> Self {
        match desc.to_string().as_str() {
            "YUYV" => PixelFormat::Yuyv,
            "MJPG" => PixelFormat::Mjpeg,
            "NV12" => PixelFormat::Nv12,
            "RGB3" => PixelFormat::Rgb24,
            "GREY" => PixelFormat::Gray,
            other => Self::Other(str::parse(other).unwrap_or(0)), // fallback to raw FOURCC
        }
    }
}

impl std::fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yuyv => f.write_str("YUYV"),
            Self::Mjpeg => f.write_str("MJPG"),
            Self::Nv12 => f.write_str("NV12"),
            Self::Rgb24 => f.write_str("RGB3"),
            Self::Gray => f.write_str("GREY"),
            // FIXME: This will print the raw FOURCC as a number, which is not very user-friendly. Consider converting it back to a string if possible.
            Self::Other(_) => f.write_str("Other"),
        }
    }
}

/// Resolution of a camera frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// One captured frame. Buffer is owned so it can outlive the borrow of the camera.
#[derive(Debug, Clone)]
pub struct Frame {
    pub data: Vec<u8>,
    pub format: PixelFormat,
    pub resolution: Resolution,
    pub sequence: u64,
    pub timestamp: Duration, // since stream start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id_to_path() {
        let id_index = DeviceId::Index(0);
        assert_eq!(id_index.to_path(), "/dev/video0");
    }
}
