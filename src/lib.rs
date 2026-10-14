// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! MPEG-1 video decoder, MP2 audio decoder and MPEG Program Stream demuxer.
//!
//! A Rust port of [PL_MPEG](https://github.com/phoboslab/pl_mpeg) by Dominic
//! Szablewski. It has no dependencies and no unsafe code.
//!
//! The high-level [`Decoder`] combines the demuxer with the video and audio
//! decoders, and is all you need to play an `.mpg` file:
//!
//! ```no_run
//! use slopmpeg::{Decoder, PixelFormat};
//!
//! let mut decoder = Decoder::open("video.mpg")?;
//! decoder.set_audio_enabled(false);
//! while let Some(frame) = decoder.decode_video()? {
//!     let rgb = frame.to_pixels(PixelFormat::Rgb);
//!     println!("{:?}: {} bytes", frame.time(), rgb.len());
//! }
//! # Ok::<(), slopmpeg::Error>(())
//! ```
//!
//! The lower-level [`Demuxer`], [`VideoDecoder`] and [`AudioDecoder`] can be
//! used to analyze a program stream or extract its packets, or to decode raw
//! MPEG-1 video or MP2 audio data from another source.

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]

mod audio;
mod bits;
mod decoder;
mod demux;
mod frame;
mod video;

use std::fmt;
use std::io;
use std::time::Duration;

pub use audio::{AudioDecoder, AudioInfo, SAMPLES_PER_FRAME, Samples};
pub use decoder::{Decoded, Decoder};
pub use demux::{Demuxer, Packet, StreamId};
pub use frame::{Frame, PixelFormat, Plane};
pub use video::{VideoDecoder, VideoInfo};

/// An error from creating a [`Decoder`] or [`Demuxer`].
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Reading from the source failed.
    Io(io::Error),
    /// No MPEG-1 pack header was found; the source is not an MPEG-1 Program
    /// Stream.
    MissingPackHeader,
    /// No system header was found.
    MissingSystemHeader,
    /// The system header announces a video stream, but no sequence header was
    /// found for it.
    MissingVideoHeader,
    /// The system header announces an audio stream, but no frame header was
    /// found for it.
    MissingAudioHeader,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::MissingPackHeader => f.write_str("no MPEG-1 pack header found"),
            Error::MissingSystemHeader => f.write_str("no MPEG system header found"),
            Error::MissingVideoHeader => f.write_str("no video sequence header found"),
            Error::MissingAudioHeader => f.write_str("no audio frame header found"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

/// Converts seconds to a [`Duration`], saturating at zero and
/// [`Duration::MAX`].
fn seconds(secs: f64) -> Duration {
    Duration::try_from_secs_f64(secs).unwrap_or(if secs > 0.0 {
        Duration::MAX
    } else {
        Duration::ZERO
    })
}
