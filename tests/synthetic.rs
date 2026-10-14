// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! Decodes a tiny program stream built by hand: a single grey 16×16 intra
//! frame and three frames of silent mono MP2 audio.

use std::io::{self, Cursor, Read};
use std::time::Duration;

use slopmpeg::{Decoded, Decoder, Demuxer, Error, PixelFormat, StreamId, VideoDecoder};

#[derive(Default)]
struct BitWriter {
    bytes: Vec<u8>,
    bits: usize,
}

impl BitWriter {
    fn put(&mut self, count: usize, value: u64) {
        for i in (0..count).rev() {
            if self.bits % 8 == 0 {
                self.bytes.push(0);
            }
            if value >> i & 1 != 0 {
                *self.bytes.last_mut().unwrap() |= 0x80 >> (self.bits % 8);
            }
            self.bits += 1;
        }
    }

    fn put_bits(&mut self, bits: &str) {
        for bit in bits.chars().filter(|c| !c.is_whitespace()) {
            self.put(1, u64::from(bit == '1'));
        }
    }

    fn start_code(&mut self, code: u8) {
        assert_eq!(self.bits % 8, 0);
        self.bytes.extend([0x00, 0x00, 0x01, code]);
        self.bits += 32;
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

fn video_elementary_stream() -> Vec<u8> {
    let mut w = BitWriter::default();

    // Sequence header: 16×16, square pixels, 25 fps, default matrices
    w.start_code(0xB3);
    w.put(12, 16);
    w.put(12, 16);
    w.put(4, 1);
    w.put(4, 3);
    w.put(18, 0x3FFFF);
    w.put(1, 1);
    w.put(10, 16);
    w.put_bits("0 0 0");

    // User data, so that the full header size is available
    w.start_code(0xB2);
    for _ in 0..140 {
        w.put(8, 0xAA);
    }

    // Intra picture
    w.start_code(0x00);
    w.put(10, 0); // temporal reference
    w.put(3, 1); // intra
    w.put(16, 0xFFFF); // vbv delay
    w.put(3, 0); // extra bit + align

    // One slice with one macroblock, whose six blocks only have a DC
    // coefficient equal to the predictor
    w.start_code(0x01);
    w.put(5, 1); // quantizer scale
    w.put(1, 0); // no extra information
    w.put_bits("1"); // address increment 1
    w.put_bits("1"); // intra
    for _ in 0..4 {
        w.put_bits("100 10"); // luma DC size 0, end of block
    }
    for _ in 0..2 {
        w.put_bits("00 10"); // chroma DC size 0, end of block
    }
    w.put(4, 0); // align

    w.start_code(0xB7); // sequence end
    w.finish()
}

fn audio_elementary_stream() -> Vec<u8> {
    let mut frames = Vec::new();
    for _ in 0..3 {
        // MPEG-1 layer II, no CRC, 32 kbit/s, 44.1 kHz, mono
        let mut frame = vec![0xFF, 0xFD, 0x10, 0xC0];
        // All bit allocations zero: silence
        frame.resize(144000 * 32 / 44100, 0);
        frames.extend(frame);
    }
    frames
}

fn pes_packet(stream_id: u8, payload: &[u8]) -> Vec<u8> {
    let length = u16::try_from(payload.len() + 1).unwrap();
    let mut packet = vec![0x00, 0x00, 0x01, stream_id];
    packet.extend(length.to_be_bytes());
    packet.push(0x0F); // no PTS
    packet.extend(payload);
    packet
}

fn program_stream() -> Vec<u8> {
    let mut w = BitWriter::default();

    // Pack header
    w.start_code(0xBA);
    w.put_bits("0010 000 1");
    w.put(15, 0);
    w.put(1, 1);
    w.put(15, 0);
    w.put_bits("1 1");
    w.put(22, 1);
    w.put(1, 1);

    // System header with one audio and one video stream
    w.start_code(0xBB);
    w.put(16, 6);
    w.put(24, 0x800001);
    w.put(6, 1);
    w.put(5, 0);
    w.put(5, 1);
    w.put(8, 0xFF);

    let mut stream = w.finish();
    stream.extend(pes_packet(0xE0, &video_elementary_stream()));
    stream.extend(pes_packet(0xC0, &audio_elementary_stream()));
    stream.extend([0x00, 0x00, 0x01, 0xB9]);
    stream
}

/// A reader that does not implement `Seek`.
struct ReadOnly<R>(R);

impl<R: Read> Read for ReadOnly<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

#[test]
fn decodes_program_stream() {
    let mut decoder = Decoder::new(ReadOnly(Cursor::new(program_stream()))).unwrap();

    let video = decoder.video_info().unwrap();
    assert_eq!((video.width, video.height), (16, 16));
    assert_eq!(video.framerate, 25.0);
    assert_eq!(video.pixel_aspect_ratio, 1.0);
    assert_eq!(decoder.audio_info().unwrap().sample_rate, 44100);
    assert_eq!(decoder.num_video_streams(), 1);
    assert_eq!(decoder.num_audio_streams(), 1);

    let mut frames = Vec::new();
    let mut samples = Vec::new();
    while !decoder.has_ended() {
        decoder
            .decode(Duration::from_millis(40), |decoded| match decoded {
                Decoded::Video(frame) => frames.push(frame.clone()),
                Decoded::Audio(audio) => samples.push(audio.clone()),
            })
            .unwrap();
    }

    assert_eq!(frames.len(), 1);
    let frame = &frames[0];
    assert_eq!(frame.time(), Duration::ZERO);
    assert_eq!((frame.width(), frame.height()), (16, 16));
    assert!(frame.y().data().iter().all(|&y| y == 128));
    assert!(frame.cb().data().iter().all(|&c| c == 128));
    assert!(frame.cr().data().iter().all(|&c| c == 128));
    let rgb = frame.to_pixels(PixelFormat::Rgb);
    assert_eq!(rgb.len(), 16 * 16 * 3);
    assert!(rgb.iter().all(|&c| c == 130));

    assert_eq!(samples.len(), 3);
    assert_eq!(samples[1].time(), Duration::from_secs_f64(1152.0 / 44100.0));
    for samples in &samples {
        assert_eq!(samples.interleaved().len(), 2 * slopmpeg::SAMPLES_PER_FRAME);
        assert!(samples.interleaved().iter().all(|&s| s == 0.0));
    }
}

#[test]
fn decodes_video_only() {
    let mut decoder = Decoder::new(Cursor::new(program_stream())).unwrap();
    decoder.set_audio_enabled(false);

    assert!(decoder.decode_video().unwrap().is_some());
    assert!(decoder.decode_video().unwrap().is_none());
    assert!(decoder.has_ended());
    assert!(decoder.decode_audio().unwrap().is_none());

    decoder.rewind().unwrap();
    assert!(!decoder.has_ended());
    assert!(decoder.decode_video().unwrap().is_some());
}

#[test]
fn demuxes_packets() {
    let mut demuxer = Demuxer::new(Cursor::new(program_stream())).unwrap();
    let mut streams = Vec::new();
    while let Some(packet) = demuxer.next_packet().unwrap() {
        assert_eq!(packet.pts(), None);
        streams.push((packet.stream(), packet.data().len()));
    }
    assert!(demuxer.has_ended());
    assert_eq!(
        streams,
        [
            (StreamId::Video, video_elementary_stream().len()),
            (StreamId::Audio(0), audio_elementary_stream().len()),
        ]
    );
}

#[test]
fn decodes_raw_video() {
    let mut decoder = VideoDecoder::new();
    let stream = video_elementary_stream();
    let (head, tail) = stream.split_at(20);

    decoder.write(head);
    assert!(!decoder.has_header());
    assert!(decoder.decode().is_none());

    decoder.write(tail);
    assert!(decoder.has_header());
    assert!(decoder.decode().is_none());

    decoder.signal_end();
    let frame = decoder.decode().unwrap();
    assert!(frame.y().data().iter().all(|&y| y == 128));
    assert!(decoder.decode().is_none());
    assert!(decoder.has_ended());
}

#[test]
fn rejects_other_data() {
    let error = Decoder::new(Cursor::new(vec![0x42; 1000])).unwrap_err();
    assert!(matches!(error, Error::MissingPackHeader), "{error:?}");
}
