//! Video for Linux (2) backend
//!
//! V4L2 is the standard API for video input and output on Linux.
//!
//! # Related Links
//! * <https://linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/v4l/v4l2.html> - Video for Linux API

pub mod context;
pub mod device;
pub mod stream;

use std::{convert::TryInto, str};

use crate::format::PixelFormat;

impl From<&[u8; 4]> for PixelFormat {
    fn from(fourcc: &[u8; 4]) -> Self {
        // We use the Linux fourccs as defined here:
        // https://www.kernel.org/doc/html/v5.5/media/uapi/v4l/videodev.html.

        // Mono (single-component) formats
        if fourcc == b"GREY" {
            PixelFormat::Gray(8)
        } else if fourcc == b"Y16 " {
            PixelFormat::Gray(16)
        } else if fourcc == b"Z16 " {
            PixelFormat::Depth(16)
        }
        // RGB formats
        else if fourcc == b"BGR3" {
            PixelFormat::Bgr(24)
        } else if fourcc == b"AR24" {
            PixelFormat::Bgra(32)
        } else if fourcc == b"RGB3" {
            PixelFormat::Rgb(24)
        } else if fourcc == b"AB24" {
            PixelFormat::Rgba(32)
        }
        // Compressed formats
        else if fourcc == b"MJPG" {
            PixelFormat::Jpeg
        }
        // Misc
        else {
            PixelFormat::Custom(String::from(str::from_utf8(fourcc).unwrap()))
        }
    }
}

impl TryInto<&[u8; 4]> for PixelFormat {
    type Error = ();

    fn try_into(self) -> Result<&'static [u8; 4], Self::Error> {
        // We use the Linux fourccs as defined here:
        // https://www.kernel.org/doc/html/v5.5/media/uapi/v4l/videodev.html.

        match self {
            PixelFormat::Custom(_) => Err(()),
            PixelFormat::Gray(8) => Ok(b"GREY"),
            PixelFormat::Gray(16) => Ok(b"Y16 "),
            PixelFormat::Depth(16) => Ok(b"Z16 "),
            PixelFormat::Bgr(24) => Ok(b"BGR3"),
            PixelFormat::Bgra(32) => Ok(b"AR24"),
            PixelFormat::Rgb(24) => Ok(b"RGB3"),
            PixelFormat::Rgb(32) => Ok(b"AB24"),
            PixelFormat::Jpeg => Ok(b"MJPG"),
            _ => Err(()),
        }
    }
}
