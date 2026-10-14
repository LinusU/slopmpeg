// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! Decodes an MPEG-PS file and prints an FNV-1a hash of every video and audio
//! frame, for comparing the output against other decoders.
//!
//! Usage: `cargo run --release --example hash_frames -- <file.mpg>`

use std::time::Duration;

use slopmpeg::{Decoded, Decoder};

fn fnv1a(mut hash: u64, bytes: impl IntoIterator<Item = u8>) -> u64 {
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: hash_frames <file.mpg>")?;
    let mut decoder = Decoder::open(path)?;

    let video = decoder.video_info().ok_or("no video stream")?;
    let audio = decoder.audio_info().ok_or("no audio stream")?;
    let duration = decoder.duration()?.unwrap_or_default();
    println!(
        "I {}x{} {:.3} fps {} Hz v{} a{} dur {:.6}",
        video.width,
        video.height,
        video.framerate,
        audio.sample_rate,
        decoder.num_video_streams(),
        decoder.num_audio_streams(),
        duration.as_secs_f64(),
    );

    while !decoder.has_ended() {
        decoder.decode(
            Duration::from_secs_f64(1.0 / 30.0),
            |decoded| match decoded {
                Decoded::Video(frame) => {
                    let mut hash = FNV_OFFSET_BASIS;
                    for plane in [frame.y(), frame.cb(), frame.cr()] {
                        hash = fnv1a(hash, plane.data().iter().copied());
                    }
                    println!("V {:.6} {hash:016x}", frame.time().as_secs_f64());
                }
                Decoded::Audio(samples) => {
                    let bytes = samples
                        .interleaved()
                        .iter()
                        .flat_map(|sample| sample.to_le_bytes());
                    let hash = fnv1a(FNV_OFFSET_BASIS, bytes);
                    println!("A {:.6} {hash:016x}", samples.time().as_secs_f64());
                }
            },
        )?;
    }

    Ok(())
}
