// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! Demuxing of MPEG Program Streams into packets.

use std::fmt;
use std::io::{self, Read, Seek};
use std::time::Duration;

use crate::bits::{BitReader, ReadSource};
use crate::{Error, seconds};

const START_PACK: u8 = 0xBA;
const START_SYSTEM: u8 = 0xBB;

const PACKET_PRIVATE: u8 = 0xBD;
const PACKET_AUDIO_1: u8 = 0xC0;
const PACKET_AUDIO_4: u8 = 0xC3;
const PACKET_VIDEO_1: u8 = 0xE0;

/// The elementary stream a [`Packet`] belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StreamId {
    /// The private stream 1 (`0xBD`).
    Private,
    /// One of the four MPEG audio streams (`0xC0`–`0xC3`), numbered `0..=3`.
    Audio(u8),
    /// The first MPEG video stream (`0xE0`).
    Video,
}

impl StreamId {
    fn from_start_code(code: u8) -> Option<Self> {
        match code {
            PACKET_PRIVATE => Some(StreamId::Private),
            PACKET_AUDIO_1..=PACKET_AUDIO_4 => Some(StreamId::Audio(code - PACKET_AUDIO_1)),
            PACKET_VIDEO_1 => Some(StreamId::Video),
            _ => None,
        }
    }

    fn start_code(self) -> u8 {
        match self {
            StreamId::Private => PACKET_PRIVATE,
            StreamId::Audio(index) => PACKET_AUDIO_1.wrapping_add(index),
            StreamId::Video => PACKET_VIDEO_1,
        }
    }
}

/// A demuxed packet of elementary stream data.
#[derive(Clone, Copy)]
pub struct Packet<'a> {
    stream: StreamId,
    pts: Option<f64>,
    data: &'a [u8],
}

impl<'a> Packet<'a> {
    /// The stream this packet belongs to.
    pub fn stream(&self) -> StreamId {
        self.stream
    }

    /// The presentation time stamp of this packet, if it has one.
    pub fn pts(&self) -> Option<Duration> {
        self.pts.map(seconds)
    }

    pub(crate) fn pts_secs(&self) -> Option<f64> {
        self.pts
    }

    /// The payload of this packet.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

impl fmt::Debug for Packet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Packet")
            .field("stream", &self.stream)
            .field("pts", &self.pts())
            .field("len", &self.data.len())
            .finish()
    }
}

/// The header of a packet whose payload is still in the buffer.
#[derive(Clone, Copy)]
pub(crate) struct PacketHeader {
    stream: StreamId,
    pub(crate) pts: Option<f64>,
    length: usize,
}

/// Demuxes an MPEG-1 Program Stream into packets.
///
/// Decoding only requires the source to implement [`Read`]. Finding the start
/// time or duration, seeking, probing and rewinding additionally require
/// [`Seek`].
pub struct Demuxer<R> {
    buffer: BitReader<ReadSource<R>>,

    last_file_size: u64,
    last_decoded_pts: f64,
    start_time: Option<(StreamId, Option<f64>)>,
    duration: Option<(StreamId, Option<f64>)>,

    start_code: Option<u8>,
    num_audio_streams: usize,
    num_video_streams: usize,

    /// Length of the packet last returned, which is skipped on the next call.
    current_length: usize,
    /// A packet whose header was decoded, but whose payload is not yet
    /// available.
    next_packet: Option<PacketHeader>,
}

impl<R> Demuxer<R> {
    /// The number of video streams (0–1) reported in the system header.
    pub fn num_video_streams(&self) -> usize {
        self.num_video_streams
    }

    /// The number of audio streams (0–4) reported in the system header.
    pub fn num_audio_streams(&self) -> usize {
        self.num_audio_streams
    }

    /// Returns whether the end of the source has been reached. This is
    /// cleared by seeking or rewinding.
    pub fn has_ended(&self) -> bool {
        self.buffer.has_ended()
    }
}

impl<R: Read> Demuxer<R> {
    /// Creates a demuxer reading from `reader`, and reads the pack and system
    /// headers.
    ///
    /// Positions used when seeking are relative to the position of `reader`
    /// at this point.
    ///
    /// # Errors
    ///
    /// Returns an error if reading fails, or if the stream does not start with
    /// MPEG-1 pack and system headers.
    pub fn new(reader: R) -> Result<Self, Error> {
        let mut demux = Demuxer {
            buffer: BitReader::new(ReadSource::new(reader)),
            last_file_size: 0,
            last_decoded_pts: 0.0,
            start_time: None,
            duration: None,
            start_code: None,
            num_audio_streams: 0,
            num_video_streams: 0,
            current_length: 0,
            next_packet: None,
        };
        demux.read_headers()?;
        Ok(demux)
    }

    pub(crate) fn take_error(&mut self) -> io::Result<()> {
        match self.buffer.source.take_error() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    fn read_headers(&mut self) -> Result<(), Error> {
        // Decode pack header
        if !self.buffer.find_start_code(START_PACK) {
            self.take_error()?;
            return Err(Error::MissingPackHeader);
        }

        if !self.buffer.has(64) {
            self.take_error()?;
            return Err(Error::MissingPackHeader);
        }

        if self.buffer.read(4) != 0x02 {
            return Err(Error::MissingPackHeader);
        }

        self.decode_time(); // system_clock_ref
        self.buffer.skip(1);
        self.buffer.skip(22); // mux_rate * 50
        self.buffer.skip(1);

        // Decode system header
        if !self.buffer.find_start_code(START_SYSTEM) || !self.buffer.has(56) {
            self.take_error()?;
            return Err(Error::MissingSystemHeader);
        }

        self.buffer.skip(16); // header_length
        self.buffer.skip(24); // rate bound
        self.num_audio_streams = self.buffer.read(6) as usize;
        self.buffer.skip(5); // misc flags
        self.num_video_streams = self.buffer.read(5) as usize;

        Ok(())
    }

    /// Decodes and returns the next packet, or `None` at the end of the
    /// source.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn next_packet(&mut self) -> io::Result<Option<Packet<'_>>> {
        let header = self.next_packet_header();
        self.take_error()?;
        Ok(header.map(|header| self.packet(header)))
    }

    fn next_packet_header(&mut self) -> Option<PacketHeader> {
        if self.current_length > 0 {
            let bits_till_next_packet = self.current_length << 3;
            if !self.buffer.has(bits_till_next_packet) {
                return None;
            }
            self.buffer.skip(bits_till_next_packet);
            self.current_length = 0;
        }

        // Pending packet waiting for data?
        if self.next_packet.is_some() {
            return self.get_packet();
        }

        // Pending packet waiting for header?
        if let Some(stream) = self.start_code.and_then(StreamId::from_start_code) {
            return self.decode_packet(stream);
        }

        loop {
            self.start_code = self.buffer.next_start_code();
            if let Some(stream) = StreamId::from_start_code(self.start_code?) {
                return self.decode_packet(stream);
            }
        }
    }

    fn decode_time(&mut self) -> f64 {
        let mut clock = u64::from(self.buffer.read(3)) << 30;
        self.buffer.skip(1);
        clock |= u64::from(self.buffer.read(15)) << 15;
        self.buffer.skip(1);
        clock |= u64::from(self.buffer.read(15));
        self.buffer.skip(1);
        clock as f64 / 90000.0
    }

    fn decode_packet(&mut self, stream: StreamId) -> Option<PacketHeader> {
        if !self.buffer.has(16 << 3) {
            return None;
        }

        self.start_code = None;

        let mut length = self.buffer.read(16) as usize;
        length = length.saturating_sub(self.buffer.skip_bytes(0xff)); // stuffing

        // skip P-STD
        if self.buffer.read(2) == 0x01 {
            self.buffer.skip(16);
            length = length.saturating_sub(2);
        }

        let pts = match self.buffer.read(2) {
            0x03 => {
                let pts = self.decode_time();
                self.last_decoded_pts = pts;
                self.buffer.skip(40); // skip dts
                length = length.saturating_sub(10);
                Some(pts)
            }
            0x02 => {
                let pts = self.decode_time();
                self.last_decoded_pts = pts;
                length = length.saturating_sub(5);
                Some(pts)
            }
            0x00 => {
                self.buffer.skip(4);
                length = length.saturating_sub(1);
                None
            }
            _ => {
                // Invalid; the payload is still returned on the next call
                self.next_packet = Some(PacketHeader {
                    stream,
                    pts: None,
                    length,
                });
                return None;
            }
        };

        self.next_packet = Some(PacketHeader {
            stream,
            pts,
            length,
        });
        self.get_packet()
    }

    fn get_packet(&mut self) -> Option<PacketHeader> {
        let header = self.next_packet?;
        if !self.buffer.has(header.length << 3) {
            return None;
        }

        self.current_length = header.length;
        self.next_packet = None;
        Some(header)
    }

    /// The payload of the packet `header` was just decoded for.
    fn packet(&self, header: PacketHeader) -> Packet<'_> {
        let start = self.buffer.bit_index >> 3;
        Packet {
            stream: header.stream,
            pts: header.pts,
            data: &self.buffer.bytes[start..start + header.length],
        }
    }

    /// Decodes packets until one with a PTS from the given stream is found.
    fn find_first_pts(&mut self, stream: StreamId) -> Option<f64> {
        while let Some(packet) = self.next_packet_header() {
            if packet.stream == stream && packet.pts.is_some() {
                return packet.pts;
            }
        }
        None
    }
}

impl<R: Read + Seek> Demuxer<R> {
    /// Rewinds to the start of the source.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn rewind(&mut self) -> io::Result<()> {
        self.buffer_seek(0);
        self.take_error()
    }

    fn buffer_seek(&mut self, position: u64) {
        self.buffer.seek(position);
        self.current_length = 0;
        self.next_packet = None;
        self.start_code = None;
    }

    /// Scans up to `probesize` bytes for the actual number of video and audio
    /// streams, replacing the counts read from the system header. For certain
    /// files (e.g. VideoCD) this can be more accurate. Usually a few hundred KB
    /// should be enough to find all streams.
    ///
    /// Returns whether any streams were found.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn probe(&mut self, probesize: u64) -> io::Result<bool> {
        let previous_pos = self.buffer.tell();

        let mut video_stream = false;
        let mut audio_streams = [false; 4];
        loop {
            self.start_code = self.buffer.next_start_code();
            match self.start_code {
                Some(PACKET_VIDEO_1) => video_stream = true,
                Some(code @ PACKET_AUDIO_1..=PACKET_AUDIO_4) => {
                    audio_streams[usize::from(code - PACKET_AUDIO_1)] = true;
                }
                Some(_) => {}
                None => break,
            }
            if self.buffer.tell() - previous_pos >= probesize {
                break;
            }
        }

        self.num_video_streams = usize::from(video_stream);
        self.num_audio_streams = audio_streams.iter().filter(|&&found| found).count();

        self.buffer_seek(previous_pos);
        self.take_error()?;
        Ok(self.num_video_streams > 0 || self.num_audio_streams > 0)
    }

    /// The PTS of the first packet of the given stream, or `None` if there is
    /// no such packet.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn start_time(&mut self, stream: StreamId) -> io::Result<Option<Duration>> {
        let start_time = self.start_time_secs(stream);
        self.take_error()?;
        Ok(start_time.map(seconds))
    }

    pub(crate) fn start_time_secs(&mut self, stream: StreamId) -> Option<f64> {
        if let Some((cached_stream, start_time)) = self.start_time {
            if cached_stream == stream {
                return start_time;
            }
        }

        let previous_pos = self.buffer.tell();
        let previous_start_code = self.start_code;

        self.buffer_seek(0);
        let start_time = self.find_first_pts(stream);
        self.start_time = Some((stream, start_time));

        self.buffer_seek(previous_pos);
        self.start_code = previous_start_code;
        start_time
    }

    /// The span between the first and the last PTS of the given stream, or
    /// `None` if there are no packets with a PTS for it.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn duration(&mut self, stream: StreamId) -> io::Result<Option<Duration>> {
        let duration = self.duration_secs(stream);
        self.take_error()?;
        Ok(duration.map(seconds))
    }

    pub(crate) fn duration_secs(&mut self, stream: StreamId) -> Option<f64> {
        let file_size = self.buffer.length();

        if let Some((cached_stream, duration)) = self.duration {
            if cached_stream == stream && self.last_file_size == file_size {
                return duration;
            }
        }

        let previous_pos = self.buffer.tell();
        let previous_start_code = self.start_code;

        // Find last video PTS. Start searching 64kb from the end and go further
        // back if needed.
        let mut duration = None;
        let start_range = 64 * 1024;
        let max_range = 4096 * 1024;
        let mut range = start_range;
        while range <= max_range {
            let seek_pos = match file_size.checked_sub(range) {
                Some(seek_pos) => seek_pos,
                None => {
                    range = max_range; // Make sure to bail after this round
                    0
                }
            };
            self.buffer_seek(seek_pos);

            let mut last_pts = None;
            while let Some(packet) = self.next_packet_header() {
                if packet.stream == stream && packet.pts.is_some() {
                    last_pts = packet.pts;
                }
            }
            if let Some(last_pts) = last_pts {
                duration = self.start_time_secs(stream).map(|start| last_pts - start);
                break;
            }
            range *= 2;
        }

        self.buffer_seek(previous_pos);
        self.start_code = previous_start_code;
        self.duration = Some((stream, duration));
        self.last_file_size = file_size;
        duration
    }

    /// Seeks to a packet of the given stream with a PTS just before `time`,
    /// and returns it. If `force_intra` is `true`, only packets containing an
    /// intra frame are considered; this only makes sense for
    /// [`StreamId::Video`].
    ///
    /// The time is relative to the first PTS of the stream.
    ///
    /// Returns `None` if no suitable packet was found.
    ///
    /// # Errors
    ///
    /// Returns any error from the underlying reader.
    pub fn seek(
        &mut self,
        time: Duration,
        stream: StreamId,
        force_intra: bool,
    ) -> io::Result<Option<Packet<'_>>> {
        let header = self.seek_header(time.as_secs_f64(), stream, force_intra)?;
        Ok(header.map(|header| self.packet(header)))
    }

    /// Like [`seek`](Self::seek), but returns the header of the packet found.
    pub(crate) fn seek_header(
        &mut self,
        seek_time: f64,
        stream: StreamId,
        force_intra: bool,
    ) -> io::Result<Option<PacketHeader>> {
        let header = self.seek_packet(seek_time, stream, force_intra);
        self.take_error()?;
        Ok(header)
    }

    fn seek_packet(
        &mut self,
        mut seek_time: f64,
        stream: StreamId,
        force_intra: bool,
    ) -> Option<PacketHeader> {
        // Using the current time, current byte position and the average bytes
        // per second for this file, try to jump to a byte position that
        // hopefully has packets containing timestamps within one second before
        // to the desired seek_time.

        // If we hit close to the seek_time scan through all packets to find the
        // last one (just before the seek_time) containing an intra frame.
        // Otherwise we should at least be closer than before. Calculate the
        // bytes per second for the jumped range and jump again.

        // The number of retries here is hard-limited to a generous amount.
        // Usually the correct range is found after 1--5 jumps, even for files
        // with very variable bitrates. If significantly more jumps are needed,
        // there's probably something wrong with the file and we just avoid
        // getting into an infinite loop. 32 retries should be enough for
        // anybody.

        let duration = self.duration_secs(stream)?;
        let start_time = self.start_time_secs(stream)?;
        let file_size = self.buffer.length() as i64;
        let mut byterate = (file_size as f64 / duration) as i64;

        let mut cur_time = self.last_decoded_pts;
        let mut scan_span = 1.0;

        seek_time = seek_time.clamp(0.0, duration.max(0.0));
        seek_time += start_time;

        for _ in 0..32 {
            let mut found_packet_with_pts = false;
            let mut found_packet_in_range = false;
            let mut last_valid_packet_start = None;
            let mut first_packet_time = None;

            let cur_pos = self.buffer.tell() as i64;

            // Estimate byte offset and jump to it.
            let offset = ((seek_time - cur_time - scan_span) * byterate as f64) as i64;
            let seek_pos = (cur_pos + offset).min(file_size - 256).max(0);

            self.buffer_seek(seek_pos as u64);

            // Scan through all packets up to the seek_time to find the last
            // packet containing an intra frame.
            while self.buffer.find_start_code(stream.start_code()) {
                let packet_start = self.buffer.tell();
                let Some(header) = self.decode_packet(stream) else {
                    continue;
                };

                // Skip packet if it has no PTS
                let Some(pts) = header.pts else {
                    continue;
                };

                // Bail scanning through packets if we hit one that is outside
                // seek_time - scan_span.
                // We also adjust the cur_time and byterate values here so the
                // next iteration can be a bit more precise.
                if pts > seek_time || pts < seek_time - scan_span {
                    found_packet_with_pts = true;
                    byterate = ((seek_pos - cur_pos) as f64 / (pts - cur_time)) as i64;
                    cur_time = pts;
                    break;
                }

                // If we are still here, it means this packet is in close range
                // to the seek_time. If this is the first packet for this jump
                // position record the PTS. If we later have to back off, when
                // there was no intra frame in this range, we can lower the
                // seek_time to not scan this range again.
                if !found_packet_in_range {
                    found_packet_in_range = true;
                    first_packet_time = Some(pts);
                }

                // Check if this is an intra frame packet. If so, record the
                // buffer position of the start of this packet. We want to jump
                // back to it later, when we know it's the last intra frame
                // before desired seek time.
                if force_intra {
                    let data = self.packet(header).data;
                    let picture = data
                        .windows(6)
                        .find(|window| window[..4] == [0x00, 0x00, 0x01, 0x00]);
                    // Bits 11--13 in the picture header contain the frame
                    // type, where 1=Intra
                    if picture.is_some_and(|window| window[5] & 0x38 == 8) {
                        last_valid_packet_start = Some(packet_start);
                    }
                }
                // If we don't want intra frames, just use the last PTS found.
                else {
                    last_valid_packet_start = Some(packet_start);
                }
            }

            // If there was at least one intra frame in the range scanned above,
            // our search is over. Jump back to the packet and decode it again.
            if let Some(packet_start) = last_valid_packet_start {
                self.buffer_seek(packet_start);
                return self.decode_packet(stream);
            }
            // If we hit the right range, but still found no intra frame, we
            // have to increases the scan_span. This is done exponentially to
            // also handle video files with very few intra frames.
            else if let Some(first_packet_time) = first_packet_time {
                scan_span *= 2.0;
                seek_time = first_packet_time;
            }
            // If we didn't find any packet with a PTS, it probably means we
            // reached the end of the file. Estimate byterate and cur_time
            // accordingly.
            else if !found_packet_with_pts {
                byterate = ((seek_pos - cur_pos) as f64 / (duration - cur_time)) as i64;
                cur_time = duration;
            }
        }

        None
    }

    /// The payload of a packet found by [`seek_header`](Self::seek_header).
    pub(crate) fn header_data(&self, header: PacketHeader) -> &[u8] {
        self.packet(header).data
    }
}

impl<R> fmt::Debug for Demuxer<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Demuxer")
            .field("num_video_streams", &self.num_video_streams)
            .field("num_audio_streams", &self.num_audio_streams)
            .field("has_ended", &self.buffer.has_ended())
            .finish_non_exhaustive()
    }
}
