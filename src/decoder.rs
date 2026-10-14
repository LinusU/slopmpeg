// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! The high-level interface, combining the demuxer and decoders.

use std::fmt;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::Path;
use std::time::Duration;

use crate::audio::{AudioDecoder, AudioInfo, Samples};
use crate::demux::{Demuxer, StreamId};
use crate::frame::Frame;
use crate::video::{VideoDecoder, VideoInfo};
use crate::{Error, seconds};

/// Decoded data passed to the callback of [`Decoder::decode`] and
/// [`Decoder::seek`].
#[derive(Clone, Copy, Debug)]
pub enum Decoded<'a> {
    /// A video frame.
    Video(&'a Frame),
    /// A frame of audio samples.
    Audio(&'a Samples),
}

/// Decodes the video and audio of an MPEG-1 Program Stream (`.mpg` file).
///
/// There are two ways to decode:
///
/// 1. Call [`decode`](Self::decode) with the time elapsed since the last
///    call. It decodes everything up to that time and passes each video frame
///    and audio frame to a callback.
/// 2. Call [`decode_video`](Self::decode_video) and
///    [`decode_audio`](Self::decode_audio) to decode exactly one frame of
///    video or audio at a time. How you synchronize the two streams is up to
///    you.
///
/// If you only want video *or* audio, disable the other stream with
/// [`set_audio_enabled`](Self::set_audio_enabled) or
/// [`set_video_enabled`](Self::set_video_enabled).
///
/// Decoding only requires the source to implement [`Read`]. Seeking, rewinding
/// and finding the duration additionally require [`Seek`].
///
/// # Example
///
/// ```no_run
/// use std::time::Duration;
/// use slopmpeg::{Decoded, Decoder, PixelFormat};
///
/// let mut decoder = Decoder::open("video.mpg")?;
/// while !decoder.has_ended() {
///     decoder.decode(Duration::from_secs(1) / 30, |decoded| match decoded {
///         Decoded::Video(frame) => {
///             let rgba = frame.to_pixels(PixelFormat::Rgba);
///             // Display the frame
///         }
///         Decoded::Audio(samples) => {
///             let interleaved = samples.interleaved();
///             // Queue the samples for playback
///         }
///     })?;
/// }
/// # Ok::<(), slopmpeg::Error>(())
/// ```
pub struct Decoder<R> {
    demux: Demuxer<R>,
    time: f64,
    has_ended: bool,

    has_video: bool,
    video_enabled: bool,
    video: VideoDecoder,

    has_audio: bool,
    audio_enabled: bool,
    audio_stream_index: u8,
    audio_lead_time: f64,
    audio: AudioDecoder,
}

impl Decoder<File> {
    /// Opens the file at `path` and reads its headers.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read, or does not
    /// contain the expected headers.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::new(File::open(path)?)
    }
}

impl<R> Decoder<R> {
    fn video_stream(&self) -> Option<StreamId> {
        (self.has_video && self.video_enabled).then_some(StreamId::Video)
    }

    fn audio_stream(&self) -> Option<StreamId> {
        (self.has_audio && self.audio_enabled).then_some(StreamId::Audio(self.audio_stream_index))
    }

    /// The number of video streams (0–1) reported in the system header.
    pub fn num_video_streams(&self) -> usize {
        self.demux.num_video_streams()
    }

    /// The number of audio streams (0–4) reported in the system header.
    pub fn num_audio_streams(&self) -> usize {
        self.demux.num_audio_streams()
    }

    /// The properties of the video stream, or `None` if there is none.
    pub fn video_info(&self) -> Option<VideoInfo> {
        self.video.info()
    }

    /// The properties of the audio stream, or `None` if there is none.
    pub fn audio_info(&self) -> Option<AudioInfo> {
        self.audio.info()
    }

    /// Returns whether video decoding is enabled. The default is `true`.
    pub fn video_enabled(&self) -> bool {
        self.video_enabled
    }

    /// Enables or disables video decoding.
    pub fn set_video_enabled(&mut self, enabled: bool) {
        self.video_enabled = enabled;
    }

    /// Returns whether audio decoding is enabled. The default is `true`.
    pub fn audio_enabled(&self) -> bool {
        self.audio_enabled
    }

    /// Enables or disables audio decoding.
    pub fn set_audio_enabled(&mut self, enabled: bool) {
        self.audio_enabled = enabled;
    }

    /// Selects the audio stream (0–3) to decode. The default is 0.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 3.
    pub fn set_audio_stream(&mut self, index: u8) {
        assert!(
            index <= 3,
            "audio stream index {index} is out of range (0-3)"
        );
        self.audio_stream_index = index;
    }

    /// The audio lead time: how far ahead of the video the audio is decoded
    /// by [`decode`](Self::decode). The default is zero.
    pub fn audio_lead_time(&self) -> Duration {
        seconds(self.audio_lead_time)
    }

    /// Sets the audio lead time. Typically this should be set to the duration
    /// of the buffer of the audio API that you use for output.
    pub fn set_audio_lead_time(&mut self, lead_time: Duration) {
        self.audio_lead_time = lead_time.as_secs_f64();
    }

    /// The current playback time.
    pub fn time(&self) -> Duration {
        seconds(self.time)
    }

    /// Returns whether the end of the stream has been reached.
    pub fn has_ended(&self) -> bool {
        self.has_ended
    }
}

impl<R: Read> Decoder<R> {
    /// Creates a decoder reading from `reader`, and reads the headers of the
    /// program stream and of the video and audio streams.
    ///
    /// # Errors
    ///
    /// Returns an error if reading fails, or if the stream does not contain
    /// the expected headers.
    pub fn new(reader: R) -> Result<Self, Error> {
        Self::from_demuxer(Demuxer::new(reader)?)
    }

    /// Creates a decoder using an existing demuxer, e.g. one on which
    /// [`Demuxer::probe`] was called, and reads the headers of the video and
    /// audio streams.
    ///
    /// # Errors
    ///
    /// Returns an error if reading fails, or if the stream headers cannot be
    /// found.
    pub fn from_demuxer(demux: Demuxer<R>) -> Result<Self, Error> {
        let mut decoder = Decoder {
            time: 0.0,
            has_ended: false,
            has_video: demux.num_video_streams() > 0,
            video_enabled: true,
            video: VideoDecoder::new(),
            has_audio: demux.num_audio_streams() > 0,
            audio_enabled: true,
            audio_stream_index: 0,
            audio_lead_time: 0.0,
            audio: AudioDecoder::new(),
            demux,
        };

        while decoder.has_video && !decoder.video.has_header() {
            if decoder.demux.has_ended() {
                return Err(Error::MissingVideoHeader);
            }
            decoder.read_packets(StreamId::Video)?;
        }

        while decoder.has_audio && !decoder.audio.has_header() {
            if decoder.demux.has_ended() {
                return Err(Error::MissingAudioHeader);
            }
            decoder.read_packets(StreamId::Audio(decoder.audio_stream_index))?;
        }

        Ok(decoder)
    }

    /// Advances the playback time by `elapsed`, decodes all video and audio
    /// up to the new time, and calls `on_decoded` for each video frame and
    /// audio frame, in order.
    ///
    /// Frame skipping is not implemented, i.e. every frame up to the current
    /// time is decoded.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn decode(
        &mut self,
        elapsed: Duration,
        mut on_decoded: impl FnMut(Decoded<'_>),
    ) -> io::Result<()> {
        self.decode_secs(elapsed.as_secs_f64(), &mut on_decoded)
    }

    fn decode_secs(
        &mut self,
        tick: f64,
        on_decoded: &mut impl FnMut(Decoded<'_>),
    ) -> io::Result<()> {
        let decode_video = self.video_stream().is_some();
        let decode_audio = self.audio_stream().is_some();

        if !decode_video && !decode_audio {
            // Nothing to do here
            return Ok(());
        }

        let mut decode_video_failed = false;
        let mut decode_audio_failed = false;

        let video_target_time = self.time + tick;
        let audio_target_time = self.time + tick + self.audio_lead_time;

        loop {
            let mut did_decode = false;

            if decode_video && self.video.time_secs() < video_target_time {
                if let Some(index) = self.next_video_frame()? {
                    on_decoded(Decoded::Video(self.video.frame(index)));
                    did_decode = true;
                } else {
                    decode_video_failed = true;
                }
            }

            if decode_audio && self.audio.time_secs() < audio_target_time {
                if self.next_audio_frame()? {
                    on_decoded(Decoded::Audio(self.audio.samples()));
                    did_decode = true;
                } else {
                    decode_audio_failed = true;
                }
            }

            if !did_decode {
                break;
            }
        }

        // Did all sources we wanted to decode fail and the demuxer is at the end?
        if (!decode_video || decode_video_failed)
            && (!decode_audio || decode_audio_failed)
            && self.demux.has_ended()
        {
            self.has_ended = true;
            return Ok(());
        }

        self.time += tick;
        Ok(())
    }

    /// Decodes and returns one video frame, and sets the playback time to the
    /// time of the frame.
    ///
    /// Returns `None` if video is disabled or no more frames can be decoded.
    /// The returned frame is valid until the next call to `decode_video`.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn decode_video(&mut self) -> io::Result<Option<&Frame>> {
        if self.video_stream().is_none() {
            return Ok(None);
        }

        let Some(index) = self.next_video_frame()? else {
            if self.demux.has_ended() {
                self.has_ended = true;
            }
            return Ok(None);
        };

        let frame = self.video.frame(index);
        self.time = frame.time;
        Ok(Some(frame))
    }

    /// Decodes and returns one frame of audio samples, and sets the playback
    /// time to the time of the samples.
    ///
    /// Returns `None` if audio is disabled or no more samples can be decoded.
    /// The returned samples are valid until the next call to `decode_audio`.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn decode_audio(&mut self) -> io::Result<Option<&Samples>> {
        if self.audio_stream().is_none() {
            return Ok(None);
        }

        if !self.next_audio_frame()? {
            if self.demux.has_ended() {
                self.has_ended = true;
            }
            return Ok(None);
        }

        let samples = self.audio.samples();
        self.time = samples.time;
        Ok(Some(samples))
    }

    /// Decodes the next video frame, feeding the video decoder with packets
    /// as needed, and returns its index.
    fn next_video_frame(&mut self) -> io::Result<Option<usize>> {
        loop {
            if let Some(index) = self.video.decode_index() {
                return Ok(Some(index));
            }
            if self.demux.has_ended() {
                return Ok(None);
            }
            self.read_packets(StreamId::Video)?;
        }
    }

    /// Decodes the next audio frame, feeding the audio decoder with packets
    /// as needed. Returns whether a frame was decoded.
    fn next_audio_frame(&mut self) -> io::Result<bool> {
        loop {
            if self.audio.decode_frame_data() {
                return Ok(true);
            }
            if self.demux.has_ended() {
                return Ok(false);
            }
            self.read_packets(StreamId::Audio(self.audio_stream_index))?;
        }
    }

    /// Demuxes packets into the enabled decoders, until a packet of the
    /// requested stream was found or the end of the source is reached.
    fn read_packets(&mut self, requested: StreamId) -> io::Result<()> {
        let video_stream = self.video_stream();
        let audio_stream = self.audio_stream();

        while let Some(packet) = self.demux.next_packet()? {
            let stream = Some(packet.stream());
            if stream == video_stream {
                self.video.write(packet.data());
            } else if stream == audio_stream {
                self.audio.write(packet.data());
            }

            if packet.stream() == requested {
                return Ok(());
            }
        }

        if self.demux.has_ended() {
            self.video.signal_end();
            self.audio.signal_end();
        }
        Ok(())
    }
}

impl<R: Read + Seek> Decoder<R> {
    /// The duration of the video stream, or `None` if it cannot be
    /// determined.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn duration(&mut self) -> io::Result<Option<Duration>> {
        self.demux.duration(StreamId::Video)
    }

    /// Rewinds to the start of the stream.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn rewind(&mut self) -> io::Result<()> {
        self.video.rewind();
        self.audio.rewind();

        self.demux.rewind()?;
        self.time = 0.0;
        self.has_ended = false;
        Ok(())
    }

    /// Seeks to `time`, clamped to the duration of the video, and returns the
    /// video frame found there, without decoding or syncing any audio.
    ///
    /// If `exact` is `true`, this seeks to the exact time; otherwise it seeks
    /// to the last intra frame at or before it. Exact seeking can be slow,
    /// because all frames following that intra frame have to be decoded.
    ///
    /// Returns `None` if video is disabled or no frame could be found.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn seek_frame(&mut self, time: Duration, exact: bool) -> io::Result<Option<&Frame>> {
        let index = self.seek_frame_index(time.as_secs_f64(), exact)?;
        Ok(index.map(|index| self.video.frame(index)))
    }

    fn seek_frame_index(&mut self, time: f64, exact: bool) -> io::Result<Option<usize>> {
        let Some(stream) = self.video_stream() else {
            return Ok(None);
        };

        let start_time = self.demux.start_time_secs(stream);
        let duration = self.demux.duration_secs(stream);
        self.demux.take_error()?;
        let (Some(start_time), Some(duration)) = (start_time, duration) else {
            return Ok(None);
        };
        let time = time.clamp(0.0, duration.max(0.0));

        let Some(header) = self.demux.seek_header(time, stream, true)? else {
            return Ok(None);
        };

        // Clear video buffer and decode the found packet
        self.video.rewind();
        self.video
            .set_time_secs(header.pts.unwrap_or(start_time) - start_time);
        self.video.write(self.demux.header_data(header));

        // Disable writing to the audio buffer while decoding video
        let audio_enabled = self.audio_enabled;
        self.audio_enabled = false;

        let mut frame = self.next_video_frame();

        // If we want to seek to an exact frame, we have to decode all frames
        // on top of the intra frame we just jumped to.
        if exact {
            while let Ok(Some(index)) = frame {
                if self.video.frame(index).time >= time {
                    break;
                }
                frame = self.next_video_frame();
            }
        }

        // Enable writing to the audio buffer again
        self.audio_enabled = audio_enabled;

        let index = frame?;
        if let Some(index) = index {
            self.time = self.video.frame(index).time;
        }

        self.has_ended = false;
        Ok(index)
    }

    /// Seeks to `time`, clamped to the duration of the video, and calls
    /// `on_decoded` with the video frame found there. If audio is enabled,
    /// audio is synced up and `on_decoded` is also called with audio samples,
    /// until the audio lead time is satisfied.
    ///
    /// See [`seek_frame`](Self::seek_frame) for the meaning of `exact`.
    ///
    /// Returns whether seeking succeeded.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn seek(
        &mut self,
        time: Duration,
        exact: bool,
        mut on_decoded: impl FnMut(Decoded<'_>),
    ) -> io::Result<bool> {
        let Some(index) = self.seek_frame_index(time.as_secs_f64(), exact)? else {
            return Ok(false);
        };

        on_decoded(Decoded::Video(self.video.frame(index)));

        // If audio is not enabled we are done here.
        let Some(audio_stream) = self.audio_stream() else {
            return Ok(true);
        };

        // Sync up Audio. This demuxes more packets until the first audio packet
        // with a PTS greater than the current time is found. decode() is then
        // called to decode enough audio data to satisfy the audio_lead_time.

        let start_time = self.demux.start_time_secs(StreamId::Video).unwrap_or(0.0);
        self.demux.take_error()?;
        self.audio.rewind();

        while let Some(packet) = self.demux.next_packet()? {
            if packet.stream() == StreamId::Video {
                self.video.write(packet.data());
            } else if packet.stream() == audio_stream {
                let Some(pts) = packet.pts_secs() else {
                    continue;
                };
                let pts = pts - start_time;
                if pts > self.time {
                    self.audio.set_time_secs(pts);
                    self.audio.write(packet.data());
                    self.decode_secs(0.0, &mut on_decoded)?;
                    break;
                }
            }
        }

        Ok(true)
    }
}

impl<R> fmt::Debug for Decoder<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Decoder")
            .field("time", &seconds(self.time))
            .field("has_ended", &self.has_ended)
            .field("video", &self.video.info())
            .field("audio", &self.audio.info())
            .finish_non_exhaustive()
    }
}
