// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! MPEG-1 video decoding.
//!
//! Inspired by Java MPEG-1 Video Decoder and Player by Zoltan Korandi
//! <https://sourceforge.net/projects/javampeg1video/>

use std::fmt;
use std::num::Wrapping;
use std::time::Duration;

use crate::bits::{BitReader, Vlc, vlc};
use crate::frame::{Frame, clamp};
use crate::seconds;

const PICTURE_TYPE_INTRA: u32 = 1;
const PICTURE_TYPE_PREDICTIVE: u32 = 2;
const PICTURE_TYPE_B: u32 = 3;

const START_SEQUENCE: u8 = 0xB3;
const START_SLICE_FIRST: u8 = 0x01;
const START_SLICE_LAST: u8 = 0xAF;
const START_PICTURE: u8 = 0x00;
const START_EXTENSION: u8 = 0xB5;
const START_USER_DATA: u8 = 0xB2;

/// Properties of a video stream, read from its sequence header.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct VideoInfo {
    /// The display width in pixels.
    pub width: usize,
    /// The display height in pixels.
    pub height: usize,
    /// The number of frames per second.
    pub framerate: f64,
    /// The width of a pixel relative to its height.
    pub pixel_aspect_ratio: f64,
}

#[derive(Clone, Copy, Default)]
struct Motion {
    full_px: bool,
    is_set: bool,
    r_size: u32,
    h: i32,
    v: i32,
}

/// Position of the current macroblock, for motion compensation.
#[derive(Clone, Copy)]
struct MacroblockPosition {
    mb_width: i32,
    mb_height: i32,
    mb_row: i32,
    mb_col: i32,
}

/// Decodes raw MPEG-1 video ("mpeg1video") data into [`Frame`]s.
///
/// Data is supplied with [`VideoDecoder::write`]. When [`VideoDecoder::decode`]
/// returns `None`, more data has to be written before the next frame can be
/// decoded. Call [`VideoDecoder::signal_end`] after the last write, so that
/// the final frames can be decoded.
///
/// To decode an MPEG Program Stream (`.mpg` file), use
/// [`Decoder`](crate::Decoder) instead.
pub struct VideoDecoder {
    buffer: BitReader<()>,
    info: Option<VideoInfo>,

    time: f64,
    frames_decoded: u64,
    mb_width: i32,
    mb_height: i32,
    mb_size: i32,

    luma_width: usize,
    chroma_width: usize,

    start_code: Option<u8>,
    picture_type: u32,

    motion_forward: Motion,
    motion_backward: Motion,

    quantizer_scale: i32,
    slice_begin: bool,
    macroblock_address: i32,

    mb_row: i32,
    mb_col: i32,

    macroblock_intra: bool,

    dc_predictor: [i32; 3],

    /// Frame buffers, referred to by index by `current`, `forward` and
    /// `backward`. `forward` and `backward` may point to the same frame.
    frames: [Frame; 3],
    current: usize,
    forward: usize,
    backward: usize,

    block_data: [i32; 64],
    intra_quant_matrix: [u8; 64],
    non_intra_quant_matrix: [u8; 64],

    has_reference_frame: bool,
    assume_no_b_frames: bool,
}

impl Default for VideoDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoDecoder {
    /// Creates a video decoder without any data.
    #[must_use]
    pub fn new() -> Self {
        VideoDecoder {
            buffer: BitReader::new(()),
            info: None,
            time: 0.0,
            frames_decoded: 0,
            mb_width: 0,
            mb_height: 0,
            mb_size: 0,
            luma_width: 0,
            chroma_width: 0,
            start_code: None,
            picture_type: 0,
            motion_forward: Motion::default(),
            motion_backward: Motion::default(),
            quantizer_scale: 0,
            slice_begin: false,
            macroblock_address: 0,
            mb_row: 0,
            mb_col: 0,
            macroblock_intra: false,
            dc_predictor: [0; 3],
            frames: std::array::from_fn(|_| Frame::new(0, 0, 0, 0)),
            current: 0,
            forward: 1,
            backward: 2,
            block_data: [0; 64],
            intra_quant_matrix: [0; 64],
            non_intra_quant_matrix: [0; 64],
            has_reference_frame: false,
            assume_no_b_frames: false,
        }
    }

    /// Appends video data to the internal buffer.
    pub fn write(&mut self, data: &[u8]) {
        self.buffer.write(data);
    }

    /// Signals that no more data will be written, until the next
    /// [`write`](Self::write). This allows the last frames of the stream to be
    /// decoded.
    pub fn signal_end(&mut self) {
        self.buffer.signal_end();
    }

    /// Returns whether all written data has been decoded after
    /// [`signal_end`](Self::signal_end) was called.
    pub fn has_ended(&self) -> bool {
        self.buffer.has_ended()
    }

    /// Returns whether a sequence header has been found, attempting to read it
    /// from the written data if needed.
    pub fn has_header(&mut self) -> bool {
        if self.info.is_some() {
            return true;
        }

        if self.start_code != Some(START_SEQUENCE) {
            self.start_code = self
                .buffer
                .find_start_code(START_SEQUENCE)
                .then_some(START_SEQUENCE);
        }
        if self.start_code.is_none() {
            return false;
        }

        self.decode_sequence_header()
    }

    /// The properties read from the sequence header, or `None` if it has not
    /// been read yet; see [`has_header`](Self::has_header).
    pub fn info(&self) -> Option<VideoInfo> {
        self.info
    }

    /// Sets "no delay" mode. When enabled, the decoder assumes that the video
    /// does *not* contain any B-frames. This is useful for reducing lag when
    /// streaming. The default is `false`.
    pub fn set_no_delay(&mut self, no_delay: bool) {
        self.assume_no_b_frames = no_delay;
    }

    /// The presentation time of the next frame.
    pub fn time(&self) -> Duration {
        seconds(self.time)
    }

    pub(crate) fn time_secs(&self) -> f64 {
        self.time
    }

    /// Sets the presentation time of the next frame. This is only useful when
    /// you manipulate the written data and want to enforce correct
    /// timestamps.
    pub fn set_time(&mut self, time: Duration) {
        self.set_time_secs(time.as_secs_f64());
    }

    pub(crate) fn set_time_secs(&mut self, time: f64) {
        self.frames_decoded = (self.framerate() * time) as u64;
        self.time = time;
    }

    fn framerate(&self) -> f64 {
        self.info.map_or(0.0, |info| info.framerate)
    }

    /// Drops all written data and resets the time to zero. The sequence header
    /// is kept.
    pub fn rewind(&mut self) {
        self.buffer.clear();
        self.time = 0.0;
        self.frames_decoded = 0;
        self.has_reference_frame = false;
        self.start_code = None;
    }

    /// Decodes and returns the next frame, and advances the time by one frame.
    ///
    /// Returns `None` if more data is needed, or if the end of the data has
    /// been reached after [`signal_end`](Self::signal_end).
    ///
    /// Since B-frames are decoded out of order, the frame returned is not
    /// necessarily the one just decoded. The returned frame is valid until the
    /// next call to `decode`; clone it if you need it for longer.
    pub fn decode(&mut self) -> Option<&Frame> {
        let index = self.decode_index()?;
        Some(&self.frames[index])
    }

    /// Like [`decode`](Self::decode), but returns the index of the frame.
    pub(crate) fn decode_index(&mut self) -> Option<usize> {
        if !self.has_header() {
            return None;
        }

        let index = loop {
            if self.start_code != Some(START_PICTURE) {
                self.start_code = self
                    .buffer
                    .find_start_code(START_PICTURE)
                    .then_some(START_PICTURE);

                if self.start_code.is_none() {
                    // If we reached the end of the file and the previously
                    // decoded frame was a reference frame, we still have to
                    // return it.
                    if self.has_reference_frame
                        && !self.assume_no_b_frames
                        && self.buffer.has_ended()
                        && (self.picture_type == PICTURE_TYPE_INTRA
                            || self.picture_type == PICTURE_TYPE_PREDICTIVE)
                    {
                        self.has_reference_frame = false;
                        break self.backward;
                    }

                    return None;
                }
            }

            // Make sure we have a full picture in the buffer before attempting
            // to decode it. Sadly, this can only be done by seeking for the
            // start code of the next picture. Also, if we didn't find the start
            // code for the next picture, but the source has ended, we assume
            // that this last picture is in the buffer.
            if !self.buffer.has_start_code(START_PICTURE) && !self.buffer.has_ended() {
                return None;
            }
            self.buffer.discard_read_bytes();

            self.decode_picture();

            if self.assume_no_b_frames {
                break self.backward;
            } else if self.picture_type == PICTURE_TYPE_B {
                break self.current;
            } else if self.has_reference_frame {
                break self.forward;
            } else {
                self.has_reference_frame = true;
            }
        };

        self.frames[index].time = self.time;
        self.frames_decoded += 1;
        self.time = self.frames_decoded as f64 / self.framerate();

        Some(index)
    }

    /// The frame with the given index, as returned by
    /// [`decode_index`](Self::decode_index).
    pub(crate) fn frame(&self, index: usize) -> &Frame {
        &self.frames[index]
    }

    fn decode_sequence_header(&mut self) -> bool {
        let max_header_size = 64 + 2 * 64 * 8; // 64 bit header + 2x 64 byte matrix
        if !self.buffer.has(max_header_size) {
            return false;
        }

        let width = self.buffer.read(12) as usize;
        let height = self.buffer.read(12) as usize;

        if width == 0 || height == 0 {
            // Not a valid header; look for another one
            self.start_code = None;
            return false;
        }

        // Get pixel aspect ratio
        let pixel_aspect_ratio_code = (self.buffer.read(4) as usize)
            .saturating_sub(1)
            .min(PIXEL_ASPECT_RATIO.len() - 1);
        let pixel_aspect_ratio = f64::from(PIXEL_ASPECT_RATIO[pixel_aspect_ratio_code]);

        // Get frame rate
        let framerate = PICTURE_RATE[self.buffer.read(4) as usize];

        // Skip bit_rate, marker, buffer_size and constrained bit
        self.buffer.skip(18 + 1 + 10 + 1);

        // Load custom intra quant matrix?
        self.intra_quant_matrix = if self.buffer.read_bit() {
            self.read_quant_matrix()
        } else {
            INTRA_QUANT_MATRIX
        };

        // Load custom non intra quant matrix?
        self.non_intra_quant_matrix = if self.buffer.read_bit() {
            self.read_quant_matrix()
        } else {
            NON_INTRA_QUANT_MATRIX
        };

        let mb_width = (width + 15) >> 4;
        let mb_height = (height + 15) >> 4;
        self.mb_width = mb_width as i32;
        self.mb_height = mb_height as i32;
        self.mb_size = self.mb_width * self.mb_height;

        self.luma_width = mb_width << 4;
        self.chroma_width = mb_width << 3;

        self.frames = std::array::from_fn(|_| Frame::new(width, height, mb_width, mb_height));

        self.info = Some(VideoInfo {
            width,
            height,
            framerate,
            pixel_aspect_ratio,
        });
        true
    }

    fn read_quant_matrix(&mut self) -> [u8; 64] {
        let mut matrix = [0; 64];
        for &idx in &ZIG_ZAG {
            matrix[usize::from(idx)] = self.buffer.read(8) as u8;
        }
        matrix
    }

    fn decode_picture(&mut self) {
        self.buffer.skip(10); // skip temporalReference
        self.picture_type = self.buffer.read(3);
        self.buffer.skip(16); // skip vbv_delay

        // D frames or unknown coding type
        if self.picture_type == 0 || self.picture_type > PICTURE_TYPE_B {
            return;
        }

        // Forward full_px, f_code
        if self.picture_type == PICTURE_TYPE_PREDICTIVE || self.picture_type == PICTURE_TYPE_B {
            self.motion_forward.full_px = self.buffer.read_bit();
            let f_code = self.buffer.read(3);
            if f_code == 0 {
                // Ignore picture with zero f_code
                return;
            }
            self.motion_forward.r_size = f_code - 1;
        }

        // Backward full_px, f_code
        if self.picture_type == PICTURE_TYPE_B {
            self.motion_backward.full_px = self.buffer.read_bit();
            let f_code = self.buffer.read(3);
            if f_code == 0 {
                // Ignore picture with zero f_code
                return;
            }
            self.motion_backward.r_size = f_code - 1;
        }

        let is_reference =
            self.picture_type == PICTURE_TYPE_INTRA || self.picture_type == PICTURE_TYPE_PREDICTIVE;

        let frame_temp = self.forward;
        if is_reference {
            self.forward = self.backward;
        }

        // Find first slice start code; skip extension and user data
        loop {
            self.start_code = self.buffer.next_start_code();
            if self.start_code != Some(START_EXTENSION) && self.start_code != Some(START_USER_DATA)
            {
                break;
            }
        }

        // Decode all slices
        while let Some(slice @ START_SLICE_FIRST..=START_SLICE_LAST) = self.start_code {
            self.decode_slice(i32::from(slice));
            if self.macroblock_address >= self.mb_size - 1 {
                break;
            }
            self.start_code = self.buffer.next_start_code();
        }

        // If this is a reference picture rotate the prediction pointers
        if is_reference {
            self.backward = self.current;
            self.current = frame_temp;
        }
    }

    fn reset_motion_vectors(&mut self) {
        self.motion_forward.h = 0;
        self.motion_forward.v = 0;
        self.motion_backward.h = 0;
        self.motion_backward.v = 0;
    }

    fn decode_slice(&mut self, slice: i32) {
        self.slice_begin = true;
        self.macroblock_address = (slice - 1) * self.mb_width - 1;

        // Reset motion vectors and DC predictors
        self.reset_motion_vectors();
        self.dc_predictor = [128; 3];

        self.quantizer_scale = self.buffer.read(5) as i32;

        // Skip extra
        while self.buffer.read_bit() {
            self.buffer.skip(8);
        }

        loop {
            self.decode_macroblock();
            if self.macroblock_address >= self.mb_size - 1 || !self.buffer.peek_non_zero(23) {
                break;
            }
        }
    }

    fn set_macroblock_position(&mut self) {
        self.mb_row = self.macroblock_address / self.mb_width;
        self.mb_col = self.macroblock_address % self.mb_width;
    }

    fn decode_macroblock(&mut self) {
        // Decode increment
        let mut increment = 0;
        let mut t = i32::from(self.buffer.read_vlc(&MACROBLOCK_ADDRESS_INCREMENT));

        while t == 34 {
            // macroblock_stuffing
            t = i32::from(self.buffer.read_vlc(&MACROBLOCK_ADDRESS_INCREMENT));
        }
        while t == 35 {
            // macroblock_escape
            increment += 33;
            t = i32::from(self.buffer.read_vlc(&MACROBLOCK_ADDRESS_INCREMENT));
        }
        increment += t;

        // Process any skipped macroblocks
        if self.slice_begin {
            // The first increment of each slice is relative to beginning of the
            // previous row, not the previous macroblock
            self.slice_begin = false;
            self.macroblock_address += increment;
        } else {
            if self.macroblock_address + increment >= self.mb_size {
                return; // invalid
            }
            if increment > 1 {
                // Skipped macroblocks reset DC predictors
                self.dc_predictor = [128; 3];

                // Skipped macroblocks in P-pictures reset motion vectors
                if self.picture_type == PICTURE_TYPE_PREDICTIVE {
                    self.motion_forward.h = 0;
                    self.motion_forward.v = 0;
                }
            }

            // Predict skipped macroblocks
            while increment > 1 {
                self.macroblock_address += 1;
                self.set_macroblock_position();

                self.predict_macroblock();
                increment -= 1;
            }
            self.macroblock_address += 1;
        }

        self.set_macroblock_position();

        if self.mb_col < 0
            || self.mb_row < 0
            || self.mb_col >= self.mb_width
            || self.mb_row >= self.mb_height
        {
            return; // corrupt stream;
        }

        // Process the current macroblock
        let table: &[Vlc<i16>] = match self.picture_type {
            PICTURE_TYPE_INTRA => &MACROBLOCK_TYPE_INTRA,
            PICTURE_TYPE_PREDICTIVE => &MACROBLOCK_TYPE_PREDICTIVE,
            _ => &MACROBLOCK_TYPE_B,
        };
        let macroblock_type = self.buffer.read_vlc(table);

        self.macroblock_intra = macroblock_type & 0x01 != 0;
        self.motion_forward.is_set = macroblock_type & 0x08 != 0;
        self.motion_backward.is_set = macroblock_type & 0x04 != 0;

        // Quantizer scale
        if macroblock_type & 0x10 != 0 {
            self.quantizer_scale = self.buffer.read(5) as i32;
        }

        if self.macroblock_intra {
            // Intra-coded macroblocks reset motion vectors
            self.reset_motion_vectors();
        } else {
            // Non-intra macroblocks reset DC predictors
            self.dc_predictor = [128; 3];

            self.decode_motion_vectors();
            self.predict_macroblock();
        }

        // Decode blocks
        let cbp = if macroblock_type & 0x02 != 0 {
            self.buffer.read_vlc(&CODE_BLOCK_PATTERN)
        } else if self.macroblock_intra {
            0x3f
        } else {
            0
        };

        for block in 0..6 {
            if cbp & (0x20 >> block) != 0 {
                self.decode_block(block);
            }
        }
    }

    fn decode_motion_vectors(&mut self) {
        // Forward
        if self.motion_forward.is_set {
            let r_size = self.motion_forward.r_size;
            self.motion_forward.h = self.decode_motion_vector(r_size, self.motion_forward.h);
            self.motion_forward.v = self.decode_motion_vector(r_size, self.motion_forward.v);
        } else if self.picture_type == PICTURE_TYPE_PREDICTIVE {
            // No motion information in P-picture, reset vectors
            self.motion_forward.h = 0;
            self.motion_forward.v = 0;
        }

        if self.motion_backward.is_set {
            let r_size = self.motion_backward.r_size;
            self.motion_backward.h = self.decode_motion_vector(r_size, self.motion_backward.h);
            self.motion_backward.v = self.decode_motion_vector(r_size, self.motion_backward.v);
        }
    }

    fn decode_motion_vector(&mut self, r_size: u32, motion: i32) -> i32 {
        let fscale = 1 << r_size;
        let m_code = i32::from(self.buffer.read_vlc(&MOTION));

        let d = if m_code != 0 && fscale != 1 {
            let r = self.buffer.read(r_size) as i32;
            let d = ((m_code.abs() - 1) << r_size) + r + 1;
            if m_code < 0 { -d } else { d }
        } else {
            m_code
        };

        let mut motion = motion + d;
        if motion > (fscale << 4) - 1 {
            motion -= fscale << 5;
        } else if motion < -(fscale << 4) {
            motion += fscale << 5;
        }

        motion
    }

    fn predict_macroblock(&mut self) {
        let mut fw_h = self.motion_forward.h;
        let mut fw_v = self.motion_forward.v;

        if self.motion_forward.full_px {
            fw_h <<= 1;
            fw_v <<= 1;
        }

        if self.picture_type == PICTURE_TYPE_B {
            let mut bw_h = self.motion_backward.h;
            let mut bw_v = self.motion_backward.v;

            if self.motion_backward.full_px {
                bw_h <<= 1;
                bw_v <<= 1;
            }

            if self.motion_forward.is_set {
                self.copy_macroblock(self.forward, fw_h, fw_v, false);
                if self.motion_backward.is_set {
                    self.copy_macroblock(self.backward, bw_h, bw_v, true);
                }
            } else {
                self.copy_macroblock(self.backward, bw_h, bw_v, false);
            }
        } else {
            self.copy_macroblock(self.forward, fw_h, fw_v, false);
        }
    }

    /// Copies (or, with `interpolate`, averages) the current macroblock from
    /// the `source` frame into the current frame.
    fn copy_macroblock(&mut self, source: usize, motion_h: i32, motion_v: i32, interpolate: bool) {
        let position = MacroblockPosition {
            mb_width: self.mb_width,
            mb_height: self.mb_height,
            mb_row: self.mb_row,
            mb_col: self.mb_col,
        };
        let [s, d] = self
            .frames
            .get_disjoint_mut([source, self.current])
            .expect("the current frame is never a reference frame");
        let (cr, cb) = (s.cr_offset(), s.cb_offset());

        process_macroblock(
            position,
            &s.data,
            &mut d.data,
            motion_h,
            motion_v,
            16,
            interpolate,
        );
        let (motion_h, motion_v) = (motion_h / 2, motion_v / 2);
        process_macroblock(
            position,
            &s.data[cr..],
            &mut d.data[cr..],
            motion_h,
            motion_v,
            8,
            interpolate,
        );
        process_macroblock(
            position,
            &s.data[cb..],
            &mut d.data[cb..],
            motion_h,
            motion_v,
            8,
            interpolate,
        );
    }

    fn decode_block(&mut self, block: usize) {
        let mut n = 0;

        // Decode DC coefficient of intra-coded blocks
        let quant_matrix = if self.macroblock_intra {
            // DC prediction
            let plane_index = block.saturating_sub(3);
            let predictor = self.dc_predictor[plane_index];
            let table: &[Vlc<i16>] = if plane_index == 0 {
                &DCT_SIZE_LUMINANCE
            } else {
                &DCT_SIZE_CHROMINANCE
            };
            let dct_size = self.buffer.read_vlc(table) as u32;

            // Read DC coeff
            self.block_data[0] = if dct_size > 0 {
                let differential = self.buffer.read(dct_size) as i32;
                if differential & (1 << (dct_size - 1)) != 0 {
                    predictor + differential
                } else {
                    predictor + (-(1 << dct_size) | (differential + 1))
                }
            } else {
                predictor
            };

            // Save predictor value
            self.dc_predictor[plane_index] = self.block_data[0];

            // Dequantize + premultiply
            self.block_data[0] <<= 3 + 5;

            n = 1;
            self.intra_quant_matrix
        } else {
            self.non_intra_quant_matrix
        };

        // Decode AC coefficients (+DC for non-intra)
        loop {
            let coeff = self.buffer.read_vlc(&DCT_COEFF);

            if coeff == 0x0001 && n > 0 && !self.buffer.read_bit() {
                // end_of_block
                break;
            }

            let run;
            let mut level;
            if coeff == 0xffff {
                // escape
                run = self.buffer.read(6) as usize;
                level = self.buffer.read(8) as i32;
                if level == 0 {
                    level = self.buffer.read(8) as i32;
                } else if level == 128 {
                    level = self.buffer.read(8) as i32 - 256;
                } else if level > 128 {
                    level -= 256;
                }
            } else {
                run = usize::from(coeff >> 8);
                level = i32::from(coeff & 0xff);
                if self.buffer.read_bit() {
                    level = -level;
                }
            }

            n += run;
            if n >= 64 {
                return; // invalid
            }

            let de_zig_zagged = usize::from(ZIG_ZAG[n]);
            n += 1;

            // Dequantize, oddify, clip
            level <<= 1;
            if !self.macroblock_intra {
                level += if level < 0 { -1 } else { 1 };
            }
            level = (level * self.quantizer_scale * i32::from(quant_matrix[de_zig_zagged])) >> 4;
            if level & 1 == 0 {
                level -= if level > 0 { 1 } else { -1 };
            }
            level = level.clamp(-2048, 2047);

            // Save premultiplied coefficient
            self.block_data[de_zig_zagged] = level * i32::from(PREMULTIPLIER_MATRIX[de_zig_zagged]);
        }

        // Move block to its place
        let frame = &mut self.frames[self.current];
        let (plane, dw, mut di);
        if block < 4 {
            plane = 0;
            dw = self.luma_width;
            di = (self.mb_row as usize * self.luma_width + self.mb_col as usize) << 4;
            if block & 1 != 0 {
                di += 8;
            }
            if block & 2 != 0 {
                di += self.luma_width << 3;
            }
        } else {
            plane = if block == 4 {
                frame.cb_offset()
            } else {
                frame.cr_offset()
            };
            dw = self.chroma_width;
            di = ((self.mb_row as usize * self.luma_width) << 2) + ((self.mb_col as usize) << 3);
        }
        let d = &mut frame.data[plane..];

        let s = &mut self.block_data;
        if self.macroblock_intra {
            // Overwrite (no prediction)
            if n == 1 {
                let clamped = clamp((s[0] + 128) >> 8);
                block_set(d, di, dw, 0, 8, 8, |_, _| clamped);
                s[0] = 0;
            } else {
                idct(s);
                block_set(d, di, dw, 0, 8, 8, |si, _| clamp(s[si]));
                *s = [0; 64];
            }
        } else {
            // Add data to the predicted macroblock
            if n == 1 {
                let value = (s[0] + 128) >> 8;
                block_set(d, di, dw, 0, 8, 8, |_, dest| clamp(i32::from(dest) + value));
                s[0] = 0;
            } else {
                idct(s);
                block_set(d, di, dw, 0, 8, 8, |si, dest| {
                    clamp(i32::from(dest) + s[si])
                });
                *s = [0; 64];
            }
        }
    }
}

impl fmt::Debug for VideoDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VideoDecoder")
            .field("info", &self.info)
            .field("time", &self.time())
            .finish_non_exhaustive()
    }
}

/// Writes a `block_size`×`block_size` block to `d` at `di`, row by row. `op`
/// receives the matching index into the source (whose rows are
/// `source_width` apart) and the current value in `d`.
#[inline(always)]
fn block_set(
    d: &mut [u8],
    mut di: usize,
    dest_width: usize,
    mut si: usize,
    source_width: usize,
    block_size: usize,
    op: impl Fn(usize, u8) -> u8,
) {
    let dest_scan = dest_width - block_size;
    let source_scan = source_width - block_size;
    for _ in 0..block_size {
        for _ in 0..block_size {
            d[di] = op(si, d[di]);
            si += 1;
            di += 1;
        }
        si += source_scan;
        di += dest_scan;
    }
}

/// Copies or interpolates one plane of a macroblock from `s` into `d`, with
/// half-pixel precision motion vectors.
fn process_macroblock(
    position: MacroblockPosition,
    s: &[u8],
    d: &mut [u8],
    motion_h: i32,
    motion_v: i32,
    block_size: i32,
    interpolate: bool,
) {
    let MacroblockPosition {
        mb_width,
        mb_height,
        mb_row,
        mb_col,
    } = position;
    let dw = i64::from(mb_width * block_size);

    let hp = i64::from(motion_h >> 1);
    let vp = i64::from(motion_v >> 1);
    let odd_h = (motion_h & 1) == 1;
    let odd_v = (motion_v & 1) == 1;

    let block_size_wide = i64::from(block_size);
    let si =
        (i64::from(mb_row) * block_size_wide + vp) * dw + i64::from(mb_col) * block_size_wide + hp;
    let di = (i64::from(mb_row) * dw + i64::from(mb_col)) * block_size_wide;

    let max_address =
        dw * (i64::from(mb_height) * block_size_wide - block_size_wide + 1) - block_size_wide;
    if si < 0 || di < 0 || si > max_address || di > max_address {
        return; // corrupt video
    }

    let (si, di, dw) = (si as usize, di as usize, dw as usize);
    let bs = block_size as usize;
    let at = |i: usize| u32::from(s[i]);

    match (interpolate, odd_h, odd_v) {
        (false, false, false) => block_set(d, di, dw, si, dw, bs, |si, _| s[si]),
        (false, false, true) => block_set(d, di, dw, si, dw, bs, |si, _| {
            ((at(si) + at(si + dw) + 1) >> 1) as u8
        }),
        (false, true, false) => block_set(d, di, dw, si, dw, bs, |si, _| {
            ((at(si) + at(si + 1) + 1) >> 1) as u8
        }),
        (false, true, true) => block_set(d, di, dw, si, dw, bs, |si, _| {
            ((at(si) + at(si + 1) + at(si + dw) + at(si + dw + 1) + 2) >> 2) as u8
        }),
        (true, false, false) => block_set(d, di, dw, si, dw, bs, |si, dest| {
            ((u32::from(dest) + at(si) + 1) >> 1) as u8
        }),
        (true, false, true) => block_set(d, di, dw, si, dw, bs, |si, dest| {
            ((u32::from(dest) + ((at(si) + at(si + dw) + 1) >> 1) + 1) >> 1) as u8
        }),
        (true, true, false) => block_set(d, di, dw, si, dw, bs, |si, dest| {
            ((u32::from(dest) + ((at(si) + at(si + 1) + 1) >> 1) + 1) >> 1) as u8
        }),
        (true, true, true) => block_set(d, di, dw, si, dw, bs, |si, dest| {
            let average = (at(si) + at(si + 1) + at(si + dw) + at(si + dw + 1) + 2) >> 2;
            ((u32::from(dest) + average + 1) >> 1) as u8
        }),
    }
}

/// Inverse discrete cosine transform of a premultiplied 8×8 block, in place.
#[allow(clippy::identity_op, clippy::erasing_op)] // Keep the row/column layout readable
fn idct(block: &mut [i32; 64]) {
    let mut b = block.map(Wrapping);
    let w = Wrapping;

    // Transform columns
    for i in 0..8 {
        let b1 = b[4 * 8 + i];
        let b3 = b[2 * 8 + i] + b[6 * 8 + i];
        let b4 = b[5 * 8 + i] - b[3 * 8 + i];
        let tmp1 = b[1 * 8 + i] + b[7 * 8 + i];
        let tmp2 = b[3 * 8 + i] + b[5 * 8 + i];
        let b6 = b[1 * 8 + i] - b[7 * 8 + i];
        let b7 = tmp1 + tmp2;
        let m0 = b[0 * 8 + i];
        let x4 = ((b6 * w(473) - b4 * w(196) + w(128)) >> 8) - b7;
        let x0 = x4 - (((tmp1 - tmp2) * w(362) + w(128)) >> 8);
        let x1 = m0 - b1;
        let x2 = (((b[2 * 8 + i] - b[6 * 8 + i]) * w(362) + w(128)) >> 8) - b3;
        let x3 = m0 + b1;
        let y3 = x1 + x2;
        let y4 = x3 + b3;
        let y5 = x1 - x2;
        let y6 = x3 - b3;
        let y7 = -x0 - ((b4 * w(473) + b6 * w(196) + w(128)) >> 8);
        b[0 * 8 + i] = b7 + y4;
        b[1 * 8 + i] = x4 + y3;
        b[2 * 8 + i] = y5 - x0;
        b[3 * 8 + i] = y6 - y7;
        b[4 * 8 + i] = y6 + y7;
        b[5 * 8 + i] = x0 + y5;
        b[6 * 8 + i] = y3 - x4;
        b[7 * 8 + i] = y4 - b7;
    }

    // Transform rows
    for i in (0..64).step_by(8) {
        let b1 = b[4 + i];
        let b3 = b[2 + i] + b[6 + i];
        let b4 = b[5 + i] - b[3 + i];
        let tmp1 = b[1 + i] + b[7 + i];
        let tmp2 = b[3 + i] + b[5 + i];
        let b6 = b[1 + i] - b[7 + i];
        let b7 = tmp1 + tmp2;
        let m0 = b[i];
        let x4 = ((b6 * w(473) - b4 * w(196) + w(128)) >> 8) - b7;
        let x0 = x4 - (((tmp1 - tmp2) * w(362) + w(128)) >> 8);
        let x1 = m0 - b1;
        let x2 = (((b[2 + i] - b[6 + i]) * w(362) + w(128)) >> 8) - b3;
        let x3 = m0 + b1;
        let y3 = x1 + x2;
        let y4 = x3 + b3;
        let y5 = x1 - x2;
        let y6 = x3 - b3;
        let y7 = -x0 - ((b4 * w(473) + b6 * w(196) + w(128)) >> 8);
        b[i] = (b7 + y4 + w(128)) >> 8;
        b[1 + i] = (x4 + y3 + w(128)) >> 8;
        b[2 + i] = (y5 - x0 + w(128)) >> 8;
        b[3 + i] = (y6 - y7 + w(128)) >> 8;
        b[4 + i] = (y6 + y7 + w(128)) >> 8;
        b[5 + i] = (x0 + y5 + w(128)) >> 8;
        b[6 + i] = (y3 - x4 + w(128)) >> 8;
        b[7 + i] = (y4 - b7 + w(128)) >> 8;
    }

    *block = b.map(|Wrapping(value)| value);
}

#[rustfmt::skip]
const PIXEL_ASPECT_RATIO: [f32; 14] = [
    1.0000, /* square pixels */
    0.6735, /* 3:4? */
    0.7031, /* MPEG-1 / MPEG-2 video encoding divergence? */
    0.7615, 0.8055, 0.8437, 0.8935, 0.9157, 0.9815,
    1.0255, 1.0695, 1.0950, 1.1575, 1.2051,
];

#[rustfmt::skip]
const PICTURE_RATE: [f64; 16] = [
    0.000, 23.976, 24.000, 25.000, 29.970, 30.000, 50.000, 59.940,
    60.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000,
];

#[rustfmt::skip]
const ZIG_ZAG: [u8; 64] = [
     0,  1,  8, 16,  9,  2,  3, 10,
    17, 24, 32, 25, 18, 11,  4,  5,
    12, 19, 26, 33, 40, 48, 41, 34,
    27, 20, 13,  6,  7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36,
    29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46,
    53, 60, 61, 54, 47, 55, 62, 63,
];

#[rustfmt::skip]
const INTRA_QUANT_MATRIX: [u8; 64] = [
     8, 16, 19, 22, 26, 27, 29, 34,
    16, 16, 22, 24, 27, 29, 34, 37,
    19, 22, 26, 27, 29, 34, 34, 38,
    22, 22, 26, 27, 29, 34, 37, 40,
    22, 26, 27, 29, 32, 35, 40, 48,
    26, 27, 29, 32, 35, 40, 48, 58,
    26, 27, 29, 34, 38, 46, 56, 69,
    27, 29, 35, 38, 46, 56, 69, 83,
];

const NON_INTRA_QUANT_MATRIX: [u8; 64] = [16; 64];

#[rustfmt::skip]
const PREMULTIPLIER_MATRIX: [u8; 64] = [
    32, 44, 42, 38, 32, 25, 17,  9,
    44, 62, 58, 52, 44, 35, 24, 12,
    42, 58, 55, 49, 42, 33, 23, 12,
    38, 52, 49, 44, 38, 30, 20, 10,
    32, 44, 42, 38, 32, 25, 17,  9,
    25, 35, 33, 30, 25, 20, 14,  7,
    17, 24, 23, 20, 17, 14,  9,  5,
     9, 12, 12, 10,  9,  7,  5,  2,
];

#[rustfmt::skip]
const MACROBLOCK_ADDRESS_INCREMENT: [Vlc<i16>; 80] = [
    vlc(1 << 1, 0), vlc(0, 1), // 0: x
    vlc(2 << 1, 0), vlc(3 << 1, 0), // 1: 0x
    vlc(4 << 1, 0), vlc(5 << 1, 0), // 2: 00x
    vlc(0, 3), vlc(0, 2), // 3: 01x
    vlc(6 << 1, 0), vlc(7 << 1, 0), // 4: 000x
    vlc(0, 5), vlc(0, 4), // 5: 001x
    vlc(8 << 1, 0), vlc(9 << 1, 0), // 6: 0000x
    vlc(0, 7), vlc(0, 6), // 7: 0001x
    vlc(10 << 1, 0), vlc(11 << 1, 0), // 8: 0000 0x
    vlc(12 << 1, 0), vlc(13 << 1, 0), // 9: 0000 1x
    vlc(14 << 1, 0), vlc(15 << 1, 0), // 10: 0000 00x
    vlc(16 << 1, 0), vlc(17 << 1, 0), // 11: 0000 01x
    vlc(18 << 1, 0), vlc(19 << 1, 0), // 12: 0000 10x
    vlc(0, 9), vlc(0, 8), // 13: 0000 11x
    vlc(-1, 0), vlc(20 << 1, 0), // 14: 0000 000x
    vlc(-1, 0), vlc(21 << 1, 0), // 15: 0000 001x
    vlc(22 << 1, 0), vlc(23 << 1, 0), // 16: 0000 010x
    vlc(0, 15), vlc(0, 14), // 17: 0000 011x
    vlc(0, 13), vlc(0, 12), // 18: 0000 100x
    vlc(0, 11), vlc(0, 10), // 19: 0000 101x
    vlc(24 << 1, 0), vlc(25 << 1, 0), // 20: 0000 0001x
    vlc(26 << 1, 0), vlc(27 << 1, 0), // 21: 0000 0011x
    vlc(28 << 1, 0), vlc(29 << 1, 0), // 22: 0000 0100x
    vlc(30 << 1, 0), vlc(31 << 1, 0), // 23: 0000 0101x
    vlc(32 << 1, 0), vlc(-1, 0), // 24: 0000 0001 0x
    vlc(-1, 0), vlc(33 << 1, 0), // 25: 0000 0001 1x
    vlc(34 << 1, 0), vlc(35 << 1, 0), // 26: 0000 0011 0x
    vlc(36 << 1, 0), vlc(37 << 1, 0), // 27: 0000 0011 1x
    vlc(38 << 1, 0), vlc(39 << 1, 0), // 28: 0000 0100 0x
    vlc(0, 21), vlc(0, 20), // 29: 0000 0100 1x
    vlc(0, 19), vlc(0, 18), // 30: 0000 0101 0x
    vlc(0, 17), vlc(0, 16), // 31: 0000 0101 1x
    vlc(0, 35), vlc(-1, 0), // 32: 0000 0001 00x
    vlc(-1, 0), vlc(0, 34), // 33: 0000 0001 11x
    vlc(0, 33), vlc(0, 32), // 34: 0000 0011 00x
    vlc(0, 31), vlc(0, 30), // 35: 0000 0011 01x
    vlc(0, 29), vlc(0, 28), // 36: 0000 0011 10x
    vlc(0, 27), vlc(0, 26), // 37: 0000 0011 11x
    vlc(0, 25), vlc(0, 24), // 38: 0000 0100 00x
    vlc(0, 23), vlc(0, 22), // 39: 0000 0100 01x
];

#[rustfmt::skip]
const MACROBLOCK_TYPE_INTRA: [Vlc<i16>; 4] = [
    vlc(1 << 1, 0), vlc(0, 0x01), // 0: x
    vlc(-1, 0), vlc(0, 0x11), // 1: 0x
];

#[rustfmt::skip]
const MACROBLOCK_TYPE_PREDICTIVE: [Vlc<i16>; 14] = [
    vlc(1 << 1, 0), vlc(0, 0x0a), // 0: x
    vlc(2 << 1, 0), vlc(0, 0x02), // 1: 0x
    vlc(3 << 1, 0), vlc(0, 0x08), // 2: 00x
    vlc(4 << 1, 0), vlc(5 << 1, 0), // 3: 000x
    vlc(6 << 1, 0), vlc(0, 0x12), // 4: 0000x
    vlc(0, 0x1a), vlc(0, 0x01), // 5: 0001x
    vlc(-1, 0), vlc(0, 0x11), // 6: 0000 0x
];

#[rustfmt::skip]
const MACROBLOCK_TYPE_B: [Vlc<i16>; 22] = [
    vlc(1 << 1, 0), vlc(2 << 1, 0), // 0: x
    vlc(3 << 1, 0), vlc(4 << 1, 0), // 1: 0x
    vlc(0, 0x0c), vlc(0, 0x0e), // 2: 1x
    vlc(5 << 1, 0), vlc(6 << 1, 0), // 3: 00x
    vlc(0, 0x04), vlc(0, 0x06), // 4: 01x
    vlc(7 << 1, 0), vlc(8 << 1, 0), // 5: 000x
    vlc(0, 0x08), vlc(0, 0x0a), // 6: 001x
    vlc(9 << 1, 0), vlc(10 << 1, 0), // 7: 0000x
    vlc(0, 0x1e), vlc(0, 0x01), // 8: 0001x
    vlc(-1, 0), vlc(0, 0x11), // 9: 0000 0x
    vlc(0, 0x16), vlc(0, 0x1a), // 10: 0000 1x
];

#[rustfmt::skip]
const CODE_BLOCK_PATTERN: [Vlc<i16>; 126] = [
    vlc(1 << 1, 0), vlc(2 << 1, 0), // 0: x
    vlc(3 << 1, 0), vlc(4 << 1, 0), // 1: 0x
    vlc(5 << 1, 0), vlc(6 << 1, 0), // 2: 1x
    vlc(7 << 1, 0), vlc(8 << 1, 0), // 3: 00x
    vlc(9 << 1, 0), vlc(10 << 1, 0), // 4: 01x
    vlc(11 << 1, 0), vlc(12 << 1, 0), // 5: 10x
    vlc(13 << 1, 0), vlc(0, 60), // 6: 11x
    vlc(14 << 1, 0), vlc(15 << 1, 0), // 7: 000x
    vlc(16 << 1, 0), vlc(17 << 1, 0), // 8: 001x
    vlc(18 << 1, 0), vlc(19 << 1, 0), // 9: 010x
    vlc(20 << 1, 0), vlc(21 << 1, 0), // 10: 011x
    vlc(22 << 1, 0), vlc(23 << 1, 0), // 11: 100x
    vlc(0, 32), vlc(0, 16), // 12: 101x
    vlc(0, 8), vlc(0, 4), // 13: 110x
    vlc(24 << 1, 0), vlc(25 << 1, 0), // 14: 0000x
    vlc(26 << 1, 0), vlc(27 << 1, 0), // 15: 0001x
    vlc(28 << 1, 0), vlc(29 << 1, 0), // 16: 0010x
    vlc(30 << 1, 0), vlc(31 << 1, 0), // 17: 0011x
    vlc(0, 62), vlc(0, 2), // 18: 0100x
    vlc(0, 61), vlc(0, 1), // 19: 0101x
    vlc(0, 56), vlc(0, 52), // 20: 0110x
    vlc(0, 44), vlc(0, 28), // 21: 0111x
    vlc(0, 40), vlc(0, 20), // 22: 1000x
    vlc(0, 48), vlc(0, 12), // 23: 1001x
    vlc(32 << 1, 0), vlc(33 << 1, 0), // 24: 0000 0x
    vlc(34 << 1, 0), vlc(35 << 1, 0), // 25: 0000 1x
    vlc(36 << 1, 0), vlc(37 << 1, 0), // 26: 0001 0x
    vlc(38 << 1, 0), vlc(39 << 1, 0), // 27: 0001 1x
    vlc(40 << 1, 0), vlc(41 << 1, 0), // 28: 0010 0x
    vlc(42 << 1, 0), vlc(43 << 1, 0), // 29: 0010 1x
    vlc(0, 63), vlc(0, 3), // 30: 0011 0x
    vlc(0, 36), vlc(0, 24), // 31: 0011 1x
    vlc(44 << 1, 0), vlc(45 << 1, 0), // 32: 0000 00x
    vlc(46 << 1, 0), vlc(47 << 1, 0), // 33: 0000 01x
    vlc(48 << 1, 0), vlc(49 << 1, 0), // 34: 0000 10x
    vlc(50 << 1, 0), vlc(51 << 1, 0), // 35: 0000 11x
    vlc(52 << 1, 0), vlc(53 << 1, 0), // 36: 0001 00x
    vlc(54 << 1, 0), vlc(55 << 1, 0), // 37: 0001 01x
    vlc(56 << 1, 0), vlc(57 << 1, 0), // 38: 0001 10x
    vlc(58 << 1, 0), vlc(59 << 1, 0), // 39: 0001 11x
    vlc(0, 34), vlc(0, 18), // 40: 0010 00x
    vlc(0, 10), vlc(0, 6), // 41: 0010 01x
    vlc(0, 33), vlc(0, 17), // 42: 0010 10x
    vlc(0, 9), vlc(0, 5), // 43: 0010 11x
    vlc(-1, 0), vlc(60 << 1, 0), // 44: 0000 000x
    vlc(61 << 1, 0), vlc(62 << 1, 0), // 45: 0000 001x
    vlc(0, 58), vlc(0, 54), // 46: 0000 010x
    vlc(0, 46), vlc(0, 30), // 47: 0000 011x
    vlc(0, 57), vlc(0, 53), // 48: 0000 100x
    vlc(0, 45), vlc(0, 29), // 49: 0000 101x
    vlc(0, 38), vlc(0, 26), // 50: 0000 110x
    vlc(0, 37), vlc(0, 25), // 51: 0000 111x
    vlc(0, 43), vlc(0, 23), // 52: 0001 000x
    vlc(0, 51), vlc(0, 15), // 53: 0001 001x
    vlc(0, 42), vlc(0, 22), // 54: 0001 010x
    vlc(0, 50), vlc(0, 14), // 55: 0001 011x
    vlc(0, 41), vlc(0, 21), // 56: 0001 100x
    vlc(0, 49), vlc(0, 13), // 57: 0001 101x
    vlc(0, 35), vlc(0, 19), // 58: 0001 110x
    vlc(0, 11), vlc(0, 7), // 59: 0001 111x
    vlc(0, 39), vlc(0, 27), // 60: 0000 0001x
    vlc(0, 59), vlc(0, 55), // 61: 0000 0010x
    vlc(0, 47), vlc(0, 31), // 62: 0000 0011x
];

#[rustfmt::skip]
const MOTION: [Vlc<i16>; 68] = [
    vlc(1 << 1, 0), vlc(0, 0), // 0: x
    vlc(2 << 1, 0), vlc(3 << 1, 0), // 1: 0x
    vlc(4 << 1, 0), vlc(5 << 1, 0), // 2: 00x
    vlc(0, 1), vlc(0, -1), // 3: 01x
    vlc(6 << 1, 0), vlc(7 << 1, 0), // 4: 000x
    vlc(0, 2), vlc(0, -2), // 5: 001x
    vlc(8 << 1, 0), vlc(9 << 1, 0), // 6: 0000x
    vlc(0, 3), vlc(0, -3), // 7: 0001x
    vlc(10 << 1, 0), vlc(11 << 1, 0), // 8: 0000 0x
    vlc(12 << 1, 0), vlc(13 << 1, 0), // 9: 0000 1x
    vlc(-1, 0), vlc(14 << 1, 0), // 10: 0000 00x
    vlc(15 << 1, 0), vlc(16 << 1, 0), // 11: 0000 01x
    vlc(17 << 1, 0), vlc(18 << 1, 0), // 12: 0000 10x
    vlc(0, 4), vlc(0, -4), // 13: 0000 11x
    vlc(-1, 0), vlc(19 << 1, 0), // 14: 0000 001x
    vlc(20 << 1, 0), vlc(21 << 1, 0), // 15: 0000 010x
    vlc(0, 7), vlc(0, -7), // 16: 0000 011x
    vlc(0, 6), vlc(0, -6), // 17: 0000 100x
    vlc(0, 5), vlc(0, -5), // 18: 0000 101x
    vlc(22 << 1, 0), vlc(23 << 1, 0), // 19: 0000 0011x
    vlc(24 << 1, 0), vlc(25 << 1, 0), // 20: 0000 0100x
    vlc(26 << 1, 0), vlc(27 << 1, 0), // 21: 0000 0101x
    vlc(28 << 1, 0), vlc(29 << 1, 0), // 22: 0000 0011 0x
    vlc(30 << 1, 0), vlc(31 << 1, 0), // 23: 0000 0011 1x
    vlc(32 << 1, 0), vlc(33 << 1, 0), // 24: 0000 0100 0x
    vlc(0, 10), vlc(0, -10), // 25: 0000 0100 1x
    vlc(0, 9), vlc(0, -9), // 26: 0000 0101 0x
    vlc(0, 8), vlc(0, -8), // 27: 0000 0101 1x
    vlc(0, 16), vlc(0, -16), // 28: 0000 0011 00x
    vlc(0, 15), vlc(0, -15), // 29: 0000 0011 01x
    vlc(0, 14), vlc(0, -14), // 30: 0000 0011 10x
    vlc(0, 13), vlc(0, -13), // 31: 0000 0011 11x
    vlc(0, 12), vlc(0, -12), // 32: 0000 0100 00x
    vlc(0, 11), vlc(0, -11), // 33: 0000 0100 01x
];

#[rustfmt::skip]
const DCT_SIZE_LUMINANCE: [Vlc<i16>; 18] = [
    vlc(1 << 1, 0), vlc(2 << 1, 0), // 0: x
    vlc(0, 1), vlc(0, 2), // 1: 0x
    vlc(3 << 1, 0), vlc(4 << 1, 0), // 2: 1x
    vlc(0, 0), vlc(0, 3), // 3: 10x
    vlc(0, 4), vlc(5 << 1, 0), // 4: 11x
    vlc(0, 5), vlc(6 << 1, 0), // 5: 111x
    vlc(0, 6), vlc(7 << 1, 0), // 6: 1111x
    vlc(0, 7), vlc(8 << 1, 0), // 7: 1111 1x
    vlc(0, 8), vlc(-1, 0), // 8: 1111 11x
];

#[rustfmt::skip]
const DCT_SIZE_CHROMINANCE: [Vlc<i16>; 18] = [
    vlc(1 << 1, 0), vlc(2 << 1, 0), // 0: x
    vlc(0, 0), vlc(0, 1), // 1: 0x
    vlc(0, 2), vlc(3 << 1, 0), // 2: 1x
    vlc(0, 3), vlc(4 << 1, 0), // 3: 11x
    vlc(0, 4), vlc(5 << 1, 0), // 4: 111x
    vlc(0, 5), vlc(6 << 1, 0), // 5: 1111x
    vlc(0, 6), vlc(7 << 1, 0), // 6: 1111 1x
    vlc(0, 7), vlc(8 << 1, 0), // 7: 1111 11x
    vlc(0, 8), vlc(-1, 0), // 8: 1111 111x
];

// dct_coeff bitmap:
//   0xff00  run
//   0x00ff  level
//
// Decoded values are unsigned. Sign bit follows in the stream.

#[rustfmt::skip]
const DCT_COEFF: [Vlc<u16>; 224] = [
    vlc(1 << 1, 0), vlc(0, 0x0001), // 0: x
    vlc(2 << 1, 0), vlc(3 << 1, 0), // 1: 0x
    vlc(4 << 1, 0), vlc(5 << 1, 0), // 2: 00x
    vlc(6 << 1, 0), vlc(0, 0x0101), // 3: 01x
    vlc(7 << 1, 0), vlc(8 << 1, 0), // 4: 000x
    vlc(9 << 1, 0), vlc(10 << 1, 0), // 5: 001x
    vlc(0, 0x0002), vlc(0, 0x0201), // 6: 010x
    vlc(11 << 1, 0), vlc(12 << 1, 0), // 7: 0000x
    vlc(13 << 1, 0), vlc(14 << 1, 0), // 8: 0001x
    vlc(15 << 1, 0), vlc(0, 0x0003), // 9: 0010x
    vlc(0, 0x0401), vlc(0, 0x0301), // 10: 0011x
    vlc(16 << 1, 0), vlc(0, 0xffff), // 11: 0000 0x
    vlc(17 << 1, 0), vlc(18 << 1, 0), // 12: 0000 1x
    vlc(0, 0x0701), vlc(0, 0x0601), // 13: 0001 0x
    vlc(0, 0x0102), vlc(0, 0x0501), // 14: 0001 1x
    vlc(19 << 1, 0), vlc(20 << 1, 0), // 15: 0010 0x
    vlc(21 << 1, 0), vlc(22 << 1, 0), // 16: 0000 00x
    vlc(0, 0x0202), vlc(0, 0x0901), // 17: 0000 10x
    vlc(0, 0x0004), vlc(0, 0x0801), // 18: 0000 11x
    vlc(23 << 1, 0), vlc(24 << 1, 0), // 19: 0010 00x
    vlc(25 << 1, 0), vlc(26 << 1, 0), // 20: 0010 01x
    vlc(27 << 1, 0), vlc(28 << 1, 0), // 21: 0000 000x
    vlc(29 << 1, 0), vlc(30 << 1, 0), // 22: 0000 001x
    vlc(0, 0x0d01), vlc(0, 0x0006), // 23: 0010 000x
    vlc(0, 0x0c01), vlc(0, 0x0b01), // 24: 0010 001x
    vlc(0, 0x0302), vlc(0, 0x0103), // 25: 0010 010x
    vlc(0, 0x0005), vlc(0, 0x0a01), // 26: 0010 011x
    vlc(31 << 1, 0), vlc(32 << 1, 0), // 27: 0000 0000x
    vlc(33 << 1, 0), vlc(34 << 1, 0), // 28: 0000 0001x
    vlc(35 << 1, 0), vlc(36 << 1, 0), // 29: 0000 0010x
    vlc(37 << 1, 0), vlc(38 << 1, 0), // 30: 0000 0011x
    vlc(39 << 1, 0), vlc(40 << 1, 0), // 31: 0000 0000 0x
    vlc(41 << 1, 0), vlc(42 << 1, 0), // 32: 0000 0000 1x
    vlc(43 << 1, 0), vlc(44 << 1, 0), // 33: 0000 0001 0x
    vlc(45 << 1, 0), vlc(46 << 1, 0), // 34: 0000 0001 1x
    vlc(0, 0x1001), vlc(0, 0x0502), // 35: 0000 0010 0x
    vlc(0, 0x0007), vlc(0, 0x0203), // 36: 0000 0010 1x
    vlc(0, 0x0104), vlc(0, 0x0f01), // 37: 0000 0011 0x
    vlc(0, 0x0e01), vlc(0, 0x0402), // 38: 0000 0011 1x
    vlc(47 << 1, 0), vlc(48 << 1, 0), // 39: 0000 0000 00x
    vlc(49 << 1, 0), vlc(50 << 1, 0), // 40: 0000 0000 01x
    vlc(51 << 1, 0), vlc(52 << 1, 0), // 41: 0000 0000 10x
    vlc(53 << 1, 0), vlc(54 << 1, 0), // 42: 0000 0000 11x
    vlc(55 << 1, 0), vlc(56 << 1, 0), // 43: 0000 0001 00x
    vlc(57 << 1, 0), vlc(58 << 1, 0), // 44: 0000 0001 01x
    vlc(59 << 1, 0), vlc(60 << 1, 0), // 45: 0000 0001 10x
    vlc(61 << 1, 0), vlc(62 << 1, 0), // 46: 0000 0001 11x
    vlc(-1, 0), vlc(63 << 1, 0), // 47: 0000 0000 000x
    vlc(64 << 1, 0), vlc(65 << 1, 0), // 48: 0000 0000 001x
    vlc(66 << 1, 0), vlc(67 << 1, 0), // 49: 0000 0000 010x
    vlc(68 << 1, 0), vlc(69 << 1, 0), // 50: 0000 0000 011x
    vlc(70 << 1, 0), vlc(71 << 1, 0), // 51: 0000 0000 100x
    vlc(72 << 1, 0), vlc(73 << 1, 0), // 52: 0000 0000 101x
    vlc(74 << 1, 0), vlc(75 << 1, 0), // 53: 0000 0000 110x
    vlc(76 << 1, 0), vlc(77 << 1, 0), // 54: 0000 0000 111x
    vlc(0, 0x000b), vlc(0, 0x0802), // 55: 0000 0001 000x
    vlc(0, 0x0403), vlc(0, 0x000a), // 56: 0000 0001 001x
    vlc(0, 0x0204), vlc(0, 0x0702), // 57: 0000 0001 010x
    vlc(0, 0x1501), vlc(0, 0x1401), // 58: 0000 0001 011x
    vlc(0, 0x0009), vlc(0, 0x1301), // 59: 0000 0001 100x
    vlc(0, 0x1201), vlc(0, 0x0105), // 60: 0000 0001 101x
    vlc(0, 0x0303), vlc(0, 0x0008), // 61: 0000 0001 110x
    vlc(0, 0x0602), vlc(0, 0x1101), // 62: 0000 0001 111x
    vlc(78 << 1, 0), vlc(79 << 1, 0), // 63: 0000 0000 0001x
    vlc(80 << 1, 0), vlc(81 << 1, 0), // 64: 0000 0000 0010x
    vlc(82 << 1, 0), vlc(83 << 1, 0), // 65: 0000 0000 0011x
    vlc(84 << 1, 0), vlc(85 << 1, 0), // 66: 0000 0000 0100x
    vlc(86 << 1, 0), vlc(87 << 1, 0), // 67: 0000 0000 0101x
    vlc(88 << 1, 0), vlc(89 << 1, 0), // 68: 0000 0000 0110x
    vlc(90 << 1, 0), vlc(91 << 1, 0), // 69: 0000 0000 0111x
    vlc(0, 0x0a02), vlc(0, 0x0902), // 70: 0000 0000 1000x
    vlc(0, 0x0503), vlc(0, 0x0304), // 71: 0000 0000 1001x
    vlc(0, 0x0205), vlc(0, 0x0107), // 72: 0000 0000 1010x
    vlc(0, 0x0106), vlc(0, 0x000f), // 73: 0000 0000 1011x
    vlc(0, 0x000e), vlc(0, 0x000d), // 74: 0000 0000 1100x
    vlc(0, 0x000c), vlc(0, 0x1a01), // 75: 0000 0000 1101x
    vlc(0, 0x1901), vlc(0, 0x1801), // 76: 0000 0000 1110x
    vlc(0, 0x1701), vlc(0, 0x1601), // 77: 0000 0000 1111x
    vlc(92 << 1, 0), vlc(93 << 1, 0), // 78: 0000 0000 0001 0x
    vlc(94 << 1, 0), vlc(95 << 1, 0), // 79: 0000 0000 0001 1x
    vlc(96 << 1, 0), vlc(97 << 1, 0), // 80: 0000 0000 0010 0x
    vlc(98 << 1, 0), vlc(99 << 1, 0), // 81: 0000 0000 0010 1x
    vlc(100 << 1, 0), vlc(101 << 1, 0), // 82: 0000 0000 0011 0x
    vlc(102 << 1, 0), vlc(103 << 1, 0), // 83: 0000 0000 0011 1x
    vlc(0, 0x001f), vlc(0, 0x001e), // 84: 0000 0000 0100 0x
    vlc(0, 0x001d), vlc(0, 0x001c), // 85: 0000 0000 0100 1x
    vlc(0, 0x001b), vlc(0, 0x001a), // 86: 0000 0000 0101 0x
    vlc(0, 0x0019), vlc(0, 0x0018), // 87: 0000 0000 0101 1x
    vlc(0, 0x0017), vlc(0, 0x0016), // 88: 0000 0000 0110 0x
    vlc(0, 0x0015), vlc(0, 0x0014), // 89: 0000 0000 0110 1x
    vlc(0, 0x0013), vlc(0, 0x0012), // 90: 0000 0000 0111 0x
    vlc(0, 0x0011), vlc(0, 0x0010), // 91: 0000 0000 0111 1x
    vlc(104 << 1, 0), vlc(105 << 1, 0), // 92: 0000 0000 0001 00x
    vlc(106 << 1, 0), vlc(107 << 1, 0), // 93: 0000 0000 0001 01x
    vlc(108 << 1, 0), vlc(109 << 1, 0), // 94: 0000 0000 0001 10x
    vlc(110 << 1, 0), vlc(111 << 1, 0), // 95: 0000 0000 0001 11x
    vlc(0, 0x0028), vlc(0, 0x0027), // 96: 0000 0000 0010 00x
    vlc(0, 0x0026), vlc(0, 0x0025), // 97: 0000 0000 0010 01x
    vlc(0, 0x0024), vlc(0, 0x0023), // 98: 0000 0000 0010 10x
    vlc(0, 0x0022), vlc(0, 0x0021), // 99: 0000 0000 0010 11x
    vlc(0, 0x0020), vlc(0, 0x010e), // 100: 0000 0000 0011 00x
    vlc(0, 0x010d), vlc(0, 0x010c), // 101: 0000 0000 0011 01x
    vlc(0, 0x010b), vlc(0, 0x010a), // 102: 0000 0000 0011 10x
    vlc(0, 0x0109), vlc(0, 0x0108), // 103: 0000 0000 0011 11x
    vlc(0, 0x0112), vlc(0, 0x0111), // 104: 0000 0000 0001 000x
    vlc(0, 0x0110), vlc(0, 0x010f), // 105: 0000 0000 0001 001x
    vlc(0, 0x0603), vlc(0, 0x1002), // 106: 0000 0000 0001 010x
    vlc(0, 0x0f02), vlc(0, 0x0e02), // 107: 0000 0000 0001 011x
    vlc(0, 0x0d02), vlc(0, 0x0c02), // 108: 0000 0000 0001 100x
    vlc(0, 0x0b02), vlc(0, 0x1f01), // 109: 0000 0000 0001 101x
    vlc(0, 0x1e01), vlc(0, 0x1d01), // 110: 0000 0000 0001 110x
    vlc(0, 0x1c01), vlc(0, 0x1b01), // 111: 0000 0000 0001 111x
];
