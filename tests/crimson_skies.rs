// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! Checks that the videos of Crimson Skies decode exactly like they do with
//! the original PL_MPEG C library.
//!
//! The videos are not distributed with this crate. Point the
//! `CRIMSON_SKIES_MPG_DIR` environment variable at the game's
//! `GOSDATA/ASSETS/GRAPHICS/MPG` directory to run this test; it is skipped
//! otherwise.
//!
//! The expected hashes were produced with pl_mpeg.h at commit c871f2b,
//! compiled with `-ffp-contract=off`. For each stream, every decoded frame is
//! hashed with 64-bit FNV-1a (video: the Y, Cb and Cr planes; audio: the
//! interleaved samples as little endian `f32`s), and those hashes are hashed
//! again, in order, into the value below.

use std::path::Path;
use std::time::Duration;

use slopmpeg::{Decoded, Decoder, Frame};

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

fn fnv1a(mut hash: u64, bytes: impl IntoIterator<Item = u8>) -> u64 {
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// File name, video frames, video hash, audio frames, audio hash.
#[rustfmt::skip]
const EXPECTED: [(&str, usize, u64, usize, u64); 10] = [
    ("chap0.mpg",    4349, 0xc99fa011918d49e5, 5549, 0x1bac8807b59d6a58),
    ("chap1.mpg",    3926, 0xdbe7dab807ca0a0e, 5010, 0xbcfc6151453c803a),
    ("chap2.mpg",    3698, 0x8ef232e7b1330182, 4719, 0x8611b77227dbd170),
    ("chap3.mpg",    2784, 0x47fc2f44abc201bf, 3553, 0xd24ba444c37741c6),
    ("chap4.mpg",    4124, 0xc34ec1586ff031c6, 5263, 0x73a698dc8903f630),
    ("chap5.mpg",    2874, 0x723e550915d434f9, 3668, 0x6075ac764b12e5fe),
    ("crimflag.mpg", 240,  0xa2aec7349c39cb39, 307,  0x352a88d2be2967c7),
    ("final.mpg",    3330, 0x1a3dda5dded91562, 4250, 0xd8c7a9e07d5cd39f),
    ("msopen1.mpg",  404,  0x0b3fd926b6aa620d, 518,  0xbb6908b1686c866a),
    ("zipper.mpg",   608,  0x5bd9e79a2d0a219f, 776,  0x15a782aac32193cf),
];

#[test]
fn decodes_like_pl_mpeg() {
    let Some(dir) = std::env::var_os("CRIMSON_SKIES_MPG_DIR") else {
        eprintln!("CRIMSON_SKIES_MPG_DIR is not set, skipping");
        return;
    };

    for (name, video_frames, video_hash, audio_frames, audio_hash) in EXPECTED {
        let mut decoder = Decoder::open(Path::new(&dir).join(name)).unwrap();

        let video = decoder.video_info().unwrap();
        assert_eq!((video.width, video.height), (320, 240), "{name}");
        assert_eq!(decoder.audio_info().unwrap().sample_rate, 44100, "{name}");

        let (mut video_count, mut video_digest) = (0, FNV_OFFSET_BASIS);
        let (mut audio_count, mut audio_digest) = (0, FNV_OFFSET_BASIS);
        while !decoder.has_ended() {
            decoder
                .decode(Duration::from_secs(1) / 30, |decoded| match decoded {
                    Decoded::Video(frame) => {
                        video_digest = fnv1a(video_digest, frame_hash(frame).to_le_bytes());
                        video_count += 1;
                    }
                    Decoded::Audio(samples) => {
                        let bytes = samples
                            .interleaved()
                            .iter()
                            .flat_map(|sample| sample.to_le_bytes());
                        let hash = fnv1a(FNV_OFFSET_BASIS, bytes);
                        audio_digest = fnv1a(audio_digest, hash.to_le_bytes());
                        audio_count += 1;
                    }
                })
                .unwrap();
        }

        assert_eq!(
            (video_count, video_digest),
            (video_frames, video_hash),
            "{name} video"
        );
        assert_eq!(
            (audio_count, audio_digest),
            (audio_frames, audio_hash),
            "{name} audio"
        );
    }
}

fn frame_hash(frame: &Frame) -> u64 {
    [frame.y(), frame.cb(), frame.cr()]
        .iter()
        .fold(FNV_OFFSET_BASIS, |hash, plane| {
            fnv1a(hash, plane.data().iter().copied())
        })
}

/// File name, exact, seek time in ms, resulting frame time in units of
/// 100 µs, resulting frame hash. The seeks are done in this order, and each is
/// followed by decoding three more frames.
#[rustfmt::skip]
const EXPECTED_SEEKS: [(&str, bool, u64, u64, u64); 24] = [
    ("crimflag.mpg", false, 0, 0, 0xc089691c6555e8ac),
    ("crimflag.mpg", false, 1500, 12000, 0x9b4ae1adcbf29f8d),
    ("crimflag.mpg", false, 3300, 32000, 0x9950212943b800ae),
    ("crimflag.mpg", false, 5000, 48000, 0x9d19f6a068b0a62d),
    ("crimflag.mpg", false, 7900, 76000, 0xa16599615ee524c4),
    ("crimflag.mpg", false, 100000, 76000, 0xeb9ace368d2f03b3),
    ("crimflag.mpg", true , 0, 0, 0xc089691c6555e8ac),
    ("crimflag.mpg", true , 1500, 15000, 0xe6e676bf1159a13e),
    ("crimflag.mpg", true , 3300, 33000, 0x0d5605908975dd7e),
    ("crimflag.mpg", true , 5000, 50000, 0xc674784020b5160e),
    ("crimflag.mpg", true , 7900, 79000, 0x64073494f06af405),
    ("crimflag.mpg", true , 100000, 79667, 0xd97838518e3b4a48),
    ("chap3.mpg", false, 0, 0, 0x6cda39432e27f725),
    ("chap3.mpg", false, 1500, 15000, 0x78b9cd9508df00cd),
    ("chap3.mpg", false, 3300, 30000, 0x9f0aa27eb3acd130),
    ("chap3.mpg", false, 5000, 50000, 0x1ac3cdf8b1f206c3),
    ("chap3.mpg", false, 7900, 75000, 0xf4c24cfd23057788),
    ("chap3.mpg", false, 100000, 925000, 0x2b334d30195f00a1),
    ("chap3.mpg", true , 0, 0, 0x6cda39432e27f725),
    ("chap3.mpg", true , 1500, 15000, 0x78b9cd9508df00cd),
    ("chap3.mpg", true , 3300, 33000, 0x4c2aeb20bd22b157),
    ("chap3.mpg", true , 5000, 50000, 0x64bd3aae7c55275c),
    ("chap3.mpg", true , 7900, 79000, 0xe30838dd370743e1),
    ("chap3.mpg", true , 100000, 925333, 0x33657c57e4940e54),];

#[test]
fn seeks_like_pl_mpeg() {
    let Some(dir) = std::env::var_os("CRIMSON_SKIES_MPG_DIR") else {
        eprintln!("CRIMSON_SKIES_MPG_DIR is not set, skipping");
        return;
    };

    let mut decoder = None;
    let mut current = "";
    for (name, exact, seek_ms, frame_time, hash) in EXPECTED_SEEKS {
        if name != current {
            decoder = Some(Decoder::open(Path::new(&dir).join(name)).unwrap());
            current = name;
        }
        let decoder = decoder.as_mut().unwrap();

        let frame = decoder
            .seek_frame(Duration::from_millis(seek_ms), exact)
            .unwrap()
            .unwrap();
        let time = (frame.time().as_secs_f64() * 10_000.0).round() as u64;
        assert_eq!(
            (time, frame_hash(frame)),
            (frame_time, hash),
            "{name} {seek_ms} ms, exact: {exact}"
        );

        for _ in 0..3 {
            decoder.decode_video().unwrap();
        }
    }
}
