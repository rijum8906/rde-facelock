//! Fuck code is awesome really fuck yes that's what I wanted really fuck yes
//!

// crates/facelock-camera/src/capture.rs

use crate::{DeviceId, Frame, PixelFormat, Resolution, error::CameraError};
use v4l::{
    buffer::Type,
    io::{mmap::Stream, traits::CaptureStream},
    video::Capture,
};

/// Open camera and configure the stream. Doesn't start streaming yet.
pub fn open(id: &DeviceId, resolution: Resolution) -> Result<Camera, CameraError> {
    let path = id.to_path();
    let dev = v4l::Device::with_path(&path)?;

    // Negotiate format: prefer MJPEG, fall back to YUYV.
    let requested = v4l::format::Format::new(
        resolution.width,
        resolution.height,
        v4l::FourCC::new(b"MJPG"),
    );
    let actual = dev.set_format(&requested)?; // returns what we actually got

    Ok(Camera {
        dev,
        format: PixelFormat::from(actual.fourcc),
        resolution: Resolution {
            width: actual.width,
            height: actual.height,
        },
    })
}

pub struct Camera {
    dev: v4l::Device,
    format: PixelFormat,
    resolution: Resolution,
}

impl Camera {
    pub fn next_frame(&mut self) -> Result<Frame, CameraError> {
        let mut stream = Stream::with_buffers(&self.dev, Type::VideoCapture, 4)?;
        let (buf, meta) = stream.next()?;
        Ok(Frame {
            data: buf.to_vec(),
            format: self.format,
            resolution: self.resolution,
            sequence: meta.sequence as u64,
            timestamp: std::time::Duration::from_micros(
                meta.timestamp.sec as u64 * 1_000_000 + meta.timestamp.usec as u64,
            ),
        })
    }
}

impl Camera {
    pub fn dev(&self) -> &v4l::Device {
        &self.dev
    }

    pub fn format(&self) -> PixelFormat {
        self.format
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }
}

#[cfg(test)]
mod tests {
    use crate::enumerate;

    use super::*;

    #[test]
    fn test_open_camera() {
        match enumerate() {
            Ok(devices) => {
                if devices.is_empty() {
                    eprintln!("No cameras found, skipping test.");
                } else {
                    let device = &devices[0];
                    let resolution = Resolution {
                        width: 640,
                        height: 480,
                    };
                    let mut camera = open(&device.id, resolution).expect("Failed to open camera");
                    let frame = camera.next_frame().expect("Failed to capture frame");
                    assert_eq!(frame.resolution.width, resolution.width);
                }
            }
            Err(e) => {
                panic!("Failed to enumerate cameras: {}", e);
            }
        }
    }
}
