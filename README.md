# slopmpeg 🦀

A questionable Rust port of [PL_MPEG](https://github.com/phoboslab/pl_mpeg), brought to you by artificial intelligence and insufficient supervision.

## About

**All credit for the original implementation belongs to Dominic Szablewski**, creator of PL_MPEG. This project is in no way affiliated with or endorsed by the original PL_MPEG author.

PL_MPEG is a small, dependency-free C library implementing MPEG-1 video decoding, MP2 audio decoding, and MPEG Program Stream demultiplexing.

slopmpeg is an attempt to translate that implementation into Rust, for ease of use in other Rust projects.

### ⚠️ Experimental software

This project is primarily an experiment in AI-driven software development.

- The code has received limited human review.
- Correctness and memory safety are not guaranteed.
- Do not use it to decode untrusted media without appropriate isolation.
- It is not affiliated with or endorsed by the original PL_MPEG author.

## Usage

```toml
[dependencies]
slopmpeg = { git = "https://github.com/LinusU/slopmpeg" }
```

```rust
use std::time::Duration;
use slopmpeg::{Decoded, Decoder, PixelFormat};

let mut decoder = Decoder::open("video.mpg")?;
while !decoder.has_ended() {
    decoder.decode(Duration::from_secs(1) / 30, |decoded| match decoded {
        Decoded::Video(frame) => {
            let rgba = frame.to_pixels(PixelFormat::Rgba);
            // Display the frame
        }
        Decoded::Audio(samples) => {
            let interleaved = samples.interleaved();
            // Queue the samples for playback
        }
    })?;
}
```

`Decoder` reads from any `std::io::Read`; seeking, rewinding and finding the duration also need `std::io::Seek`. `decode_video()` and `decode_audio()` decode one frame at a time instead. The lower-level `Demuxer`, `VideoDecoder` and `AudioDecoder` handle packets and raw elementary streams.

The crate has no dependencies and no unsafe code, and requires Rust 1.86 or later.

## Differences from PL_MPEG

The decoding logic follows PL_MPEG statement by statement, and the [Crimson Skies](https://en.wikipedia.org/wiki/Crimson_Skies_(video_game)) cutscenes decode bit for bit like they do with the C library. The API is reshaped for Rust:

- Data comes from an `io::Read` instead of `plm_buffer_t`; read errors are returned instead of ignored.
- Decoded frames and samples are passed to one closure instead of two callbacks.
- The video and audio decoders are fed with `write()`, instead of pulling data through load callbacks.
- Times are `Duration`s, and missing values are `Option`s instead of sentinel values.
- Looping is left to the caller: call `rewind()` when `has_ended()` returns `true`.
- Audio is always decoded to interleaved stereo; `Samples::left()` and `Samples::right()` give the separate channels.

## Testing

```sh
cargo test
```

The regression tests against the Crimson Skies videos only run when `CRIMSON_SKIES_MPG_DIR` points at the game's `GOSDATA/ASSETS/GRAPHICS/MPG` directory, since the videos can't be distributed here.

## License

slopmpeg is derived from PL_MPEG, licensed under the MIT License.

The original copyright notices and license are preserved. The complete upstream Git history is retained to make the project's origins and subsequent changes transparent.
