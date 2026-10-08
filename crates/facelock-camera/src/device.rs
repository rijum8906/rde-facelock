//! Device enumeration and probing for video capture devices on Linux (v4l2).

use v4l::{capability::Flags, context, video::Capture};

use crate::{CameraInfo, DeviceId, error::CameraError};

/// Enumerate all video capture devices on the system.
///
/// Walks `/dev/video*` and probes each node with `VIDIOC_QUERYCAP`, keeping
/// only those that advertise video capture. Devices that fail to open or
/// aren't cameras are logged and skipped, so a single broken node never
/// prevents discovery of the rest.
///
/// # Errors
///
/// Currently infallible — individual device failures are skipped rather than
/// propagated. The `Result` return is kept so callers don't have to change
/// when stricter behavior (e.g. failing on `EACCES`) is added.
pub fn enumerate() -> Result<Vec<CameraInfo>, CameraError> {
    let mut out = Vec::new();

    // `enum_devices` yields every `/dev/videoN`, including non-camera nodes
    // (metadata companions, codec devices, stale entries), so each must be
    // probed before it can be trusted.
    for dev in context::enum_devices() {
        let path = dev.path();
        match probe(path) {
            Ok(info) => out.push(info),
            Err(CameraError::NotACaptureDevice) => {
                tracing::trace!(?path, "not a capture device");
            }
            Err(e) => {
                tracing::debug!(?path, ?e, "failed to probe device");
            }
        }
    }

    Ok(out)
}

/// Probe a `/dev/video*` node and build a `CameraInfo` if it's a capture device.
fn probe(path: &std::path::Path) -> Result<CameraInfo, CameraError> {
    let dev = v4l::Device::with_path(path)?;
    let caps = dev.query_caps()?;

    // Some `/dev/video*` nodes aren't cameras: UVC devices expose metadata
    // companions (META_CAPTURE) alongside the real stream.
    if !caps.capabilities.contains(Flags::VIDEO_CAPTURE)
        || caps.capabilities.contains(Flags::META_CAPTURE)
    {
        return Err(CameraError::NotACaptureDevice);
    }

    let formats = dev.enum_formats()?.into_iter().map(Into::into).collect();

    Ok(CameraInfo {
        id: DeviceId::Index(index_from_path(path)?),
        name: caps.card,
        vendor: None, // NOTE: v4l doesn't expose vendor; use udev if needed
        formats,
    })
}

/// Extract the index `N` from a `/dev/videoN` path.
///
/// Only valid for paths from `context::enum_devices()`. `/dev/v4l/by-id/` and
/// `by-path/` entries have different names and are handled separately.
fn index_from_path(path: &std::path::Path) -> Result<u32, CameraError> {
    path.file_name()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix("video"))
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| CameraError::InvalidDevicePath(path.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_from_path() {
        assert_eq!(index_from_path("/dev/video0".as_ref()).unwrap(), 0);
        assert_eq!(index_from_path("/dev/video42".as_ref()).unwrap(), 42);
        assert!(index_from_path("/dev/video".as_ref()).is_err());
        assert!(index_from_path("/dev/videoX".as_ref()).is_err());
        assert!(index_from_path("/dev/v4l/by-id/usb-1234-video-index0".as_ref()).is_err());
    }

    #[test]
    fn test_probe_non_camera() {
        // NOTE: This test assumes that /dev/video0 is a valid camera device.
        // If it's not, the test will fail. Adjust the path as needed for your system.
        let path = "/dev/video0".as_ref();
        match probe(path) {
            Ok(info) => {
                println!("Found camera: {:?}", info);
            }
            Err(CameraError::NotACaptureDevice) => {
                println!("Device is not a capture device: {:?}", path);
            }
            Err(e) => {
                println!("Failed to probe device {:?}: {:?}", path, e);
            }
        }
    }

    // NOTE: The following test will enumerate all video devices on the system and print their information.
    // and will be confirm by reading `/dev/v4l/by-id/` path
    #[test]
    fn test_enumerate() {
        let sys_cameras = std::fs::read_dir("/dev/v4l/by-id/").unwrap().count();
        match enumerate() {
            Ok(cameras) => {
                assert_eq!(
                    cameras.len(),
                    sys_cameras,
                    "Number of cameras found does not match system count"
                );
            }
            Err(e) => {
                println!("Failed to enumerate devices: {:?}", e);
            }
        }
    }
}
