// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! MPEG-1 Audio Layer II decoding.
//!
//! Based on kjmp2 by Martin J. Fiedler
//! <http://keyj.emphy.de/kjmp2/>

use std::fmt;
use std::time::Duration;

use crate::bits::BitReader;
use crate::seconds;

/// The number of samples per channel in each decoded [`Samples`] frame.
pub const SAMPLES_PER_FRAME: usize = 1152;

const FRAME_SYNC: u32 = 0x7ff;

const MPEG_1: u32 = 0x3;

const LAYER_II: u32 = 0x2;

const MODE_JOINT_STEREO: u32 = 0x1;
const MODE_MONO: u32 = 0x3;

#[rustfmt::skip]
const SAMPLE_RATE: [u16; 8] = [
    44100, 48000, 32000, 0, // MPEG-1
    22050, 24000, 16000, 0, // MPEG-2
];

#[rustfmt::skip]
const BIT_RATE: [i16; 28] = [
    32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, // MPEG-1
     8, 16, 24, 32, 40, 48,  56,  64,  80,  96, 112, 128, 144, 160, // MPEG-2
];

const SCALEFACTOR_BASE: [i32; 3] = [0x02000000, 0x01965FEA, 0x01428A30];

/// Properties of an audio stream, read from its frame header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AudioInfo {
    /// The number of samples per second.
    pub sample_rate: u32,
}

/// One frame of decoded audio: [`SAMPLES_PER_FRAME`] stereo samples.
///
/// Mono streams are decoded to two identical channels.
#[derive(Clone)]
pub struct Samples {
    pub(crate) time: f64,
    interleaved: [f32; SAMPLES_PER_FRAME * 2],
}

impl Samples {
    /// The presentation time of the first sample.
    pub fn time(&self) -> Duration {
        seconds(self.time)
    }

    /// The samples, normalized to -1.0..=1.0, with the left and right channels
    /// interleaved.
    pub fn interleaved(&self) -> &[f32] {
        &self.interleaved
    }

    /// The samples of the left channel.
    pub fn left(&self) -> impl ExactSizeIterator<Item = f32> + '_ {
        self.interleaved.iter().step_by(2).copied()
    }

    /// The samples of the right channel.
    pub fn right(&self) -> impl ExactSizeIterator<Item = f32> + '_ {
        self.interleaved[1..].iter().step_by(2).copied()
    }
}

impl fmt::Debug for Samples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Samples")
            .field("time", &self.time())
            .finish_non_exhaustive()
    }
}

/// The large buffers of the synthesis filter, kept on the heap.
struct Synthesis {
    samples: Samples,
    d: [f32; 1024],
    v: [[f32; 1024]; 2],
    u: [f32; 32],
}

/// Decodes raw MPEG-1 Audio Layer II ("mp2") data into [`Samples`].
///
/// Data is supplied with [`AudioDecoder::write`]. When [`AudioDecoder::decode`]
/// returns `None`, more data has to be written before the next frame can be
/// decoded.
///
/// To decode an MPEG Program Stream (`.mpg` file), use
/// [`Decoder`](crate::Decoder) instead.
pub struct AudioDecoder {
    buffer: BitReader<()>,

    time: f64,
    samples_decoded: u64,
    samplerate_index: usize,
    bitrate_index: usize,
    mode: u32,
    bound: usize,
    v_pos: usize,
    next_frame_data_size: usize,
    has_header: bool,

    allocation: [[Option<&'static QuantizerSpec>; 32]; 2],
    scale_factor_info: [[u8; 32]; 2],
    scale_factor: [[[i32; 3]; 32]; 2],
    sample: [[[i32; 3]; 32]; 2],

    synthesis: Box<Synthesis>,
}

impl Default for AudioDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioDecoder {
    /// Creates an audio decoder without any data.
    #[must_use]
    pub fn new() -> Self {
        let mut d = [0.0; 1024];
        d[..512].copy_from_slice(&SYNTHESIS_WINDOW);
        d[512..].copy_from_slice(&SYNTHESIS_WINDOW);

        AudioDecoder {
            buffer: BitReader::new(()),
            time: 0.0,
            samples_decoded: 0,
            samplerate_index: 3, // Indicates 0
            bitrate_index: 0,
            mode: 0,
            bound: 0,
            v_pos: 0,
            next_frame_data_size: 0,
            has_header: false,
            allocation: [[None; 32]; 2],
            scale_factor_info: [[0; 32]; 2],
            scale_factor: [[[0; 3]; 32]; 2],
            sample: [[[0; 3]; 32]; 2],
            synthesis: Box::new(Synthesis {
                samples: Samples {
                    time: 0.0,
                    interleaved: [0.0; SAMPLES_PER_FRAME * 2],
                },
                d,
                v: [[0.0; 1024]; 2],
                u: [0.0; 32],
            }),
        }
    }

    /// Appends audio data to the internal buffer.
    pub fn write(&mut self, data: &[u8]) {
        self.buffer.write(data);
    }

    /// Signals that no more data will be written, until the next
    /// [`write`](Self::write).
    pub fn signal_end(&mut self) {
        self.buffer.signal_end();
    }

    /// Returns whether all written data has been decoded after
    /// [`signal_end`](Self::signal_end) was called.
    pub fn has_ended(&self) -> bool {
        self.buffer.has_ended()
    }

    /// Returns whether a frame header has been found, attempting to read it
    /// from the written data if needed.
    pub fn has_header(&mut self) -> bool {
        if self.has_header {
            return true;
        }

        self.next_frame_data_size = self.decode_header();
        self.has_header
    }

    /// The properties read from the frame header, or `None` if it has not been
    /// read yet; see [`has_header`](Self::has_header).
    pub fn info(&self) -> Option<AudioInfo> {
        self.has_header.then(|| AudioInfo {
            sample_rate: self.sample_rate(),
        })
    }

    fn sample_rate(&self) -> u32 {
        u32::from(SAMPLE_RATE[self.samplerate_index])
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
        self.samples_decoded = (time * f64::from(self.sample_rate())) as u64;
        self.time = time;
    }

    /// Drops all written data and resets the time to zero.
    pub fn rewind(&mut self) {
        self.buffer.clear();
        self.time = 0.0;
        self.samples_decoded = 0;
        self.next_frame_data_size = 0;
    }

    /// Decodes and returns the next frame of samples, and advances the time
    /// accordingly.
    ///
    /// Returns `None` if more data is needed. The returned samples are valid
    /// until the next call to `decode`.
    pub fn decode(&mut self) -> Option<&Samples> {
        self.decode_frame_data().then(|| self.samples())
    }

    /// The samples last returned by [`decode`](Self::decode).
    pub(crate) fn samples(&self) -> &Samples {
        &self.synthesis.samples
    }

    /// Like [`decode`](Self::decode), but only returns whether a frame was
    /// decoded.
    pub(crate) fn decode_frame_data(&mut self) -> bool {
        // Do we have at least enough information to decode the frame header?
        if self.next_frame_data_size == 0 {
            if !self.buffer.has(48) {
                return false;
            }
            self.next_frame_data_size = self.decode_header();
        }

        if self.next_frame_data_size == 0 || !self.buffer.has(self.next_frame_data_size << 3) {
            return false;
        }

        self.decode_frame();
        self.next_frame_data_size = 0;

        self.synthesis.samples.time = self.time;

        self.samples_decoded += SAMPLES_PER_FRAME as u64;
        self.time = self.samples_decoded as f64 / f64::from(self.sample_rate());

        true
    }

    fn find_frame_sync(&mut self) -> bool {
        let bytes = &self.buffer.bytes;
        let mut i = self.buffer.bit_index >> 3;
        while i + 1 < bytes.len() {
            if bytes[i] == 0xFF && (bytes[i + 1] & 0xFE) == 0xFC {
                self.buffer.bit_index = ((i + 1) << 3) + 3;
                return true;
            }
            i += 1;
        }
        self.buffer.bit_index = ((i + 1) << 3).min(bytes.len() << 3);
        false
    }

    /// Decodes a frame header and returns the size of the frame data that
    /// follows, or 0 if no valid header was found.
    fn decode_header(&mut self) -> usize {
        if !self.buffer.has(48) {
            return 0;
        }

        self.buffer.skip_bytes(0x00);
        if !self.buffer.has(48) {
            return 0;
        }
        let sync = self.buffer.read(11);

        // Attempt to resync if no syncword was found. This sucks balls. The MP2
        // stream contains a syncword just before every frame (11 bits set to
        // 1). However, this syncword is not guaranteed to not occur elsewhere
        // in the stream. So, if we have to resync, we also have to check if the
        // header (samplerate, bitrate) differs from the one we had before. This
        // all may still lead to garbage data being decoded :/

        if sync != FRAME_SYNC && !self.find_frame_sync() {
            return 0;
        }

        let version = self.buffer.read(2);
        let layer = self.buffer.read(2);
        let has_crc = !self.buffer.read_bit();

        if version != MPEG_1 || layer != LAYER_II {
            return 0;
        }

        let bitrate_index = match self.buffer.read(4) {
            index @ 1..=14 => index as usize - 1,
            _ => return 0,
        };

        let samplerate_index = self.buffer.read(2) as usize;
        if samplerate_index == 3 {
            return 0;
        }

        let padding = self.buffer.read(1) as usize;
        self.buffer.skip(1); // f_private
        let mode = self.buffer.read(2);

        // If we already have a header, make sure the samplerate, bitrate and
        // mode are still the same, otherwise we might have missed sync.
        if self.has_header
            && (self.bitrate_index != bitrate_index
                || self.samplerate_index != samplerate_index
                || self.mode != mode)
        {
            return 0;
        }

        self.bitrate_index = bitrate_index;
        self.samplerate_index = samplerate_index;
        self.mode = mode;
        self.has_header = true;

        // Parse the mode_extension, set up the stereo bound
        if mode == MODE_JOINT_STEREO {
            self.bound = (self.buffer.read(2) as usize + 1) << 2;
        } else {
            self.buffer.skip(2);
            self.bound = if mode == MODE_MONO { 0 } else { 32 };
        }

        // Discard the last 4 bits of the header and the CRC value, if present
        self.buffer.skip(4); // copyright(1), original(1), emphasis(2)
        if has_crc {
            self.buffer.skip(16);
        }

        // Compute frame size, check if we have enough data to decode the whole
        // frame.
        let bitrate = BIT_RATE[self.bitrate_index] as usize;
        let samplerate = usize::from(SAMPLE_RATE[self.samplerate_index]);
        let frame_size = (144000 * bitrate / samplerate) + padding;
        frame_size - if has_crc { 6 } else { 4 }
    }

    fn decode_frame(&mut self) {
        // Prepare the quantizer table lookups
        let tab1 = if self.mode == MODE_MONO { 0 } else { 1 };
        let tab2 = QUANT_LUT_STEP_1[tab1][self.bitrate_index];
        let tab3 = QUANT_LUT_STEP_2[usize::from(tab2)][self.samplerate_index];
        let sblimit = usize::from(tab3 & 63);
        let tab3 = usize::from(tab3 >> 6);

        if self.bound > sblimit {
            self.bound = sblimit;
        }

        // Read the allocation information
        for sb in 0..self.bound {
            self.allocation[0][sb] = self.read_allocation(sb, tab3);
            self.allocation[1][sb] = self.read_allocation(sb, tab3);
        }

        for sb in self.bound..sblimit {
            let allocation = self.read_allocation(sb, tab3);
            self.allocation[0][sb] = allocation;
            self.allocation[1][sb] = allocation;
        }

        // Read scale factor selector information
        let channels = if self.mode == MODE_MONO { 1 } else { 2 };
        for sb in 0..sblimit {
            for ch in 0..channels {
                if self.allocation[ch][sb].is_some() {
                    self.scale_factor_info[ch][sb] = self.buffer.read(2) as u8;
                }
            }
            if self.mode == MODE_MONO {
                self.scale_factor_info[1][sb] = self.scale_factor_info[0][sb];
            }
        }

        // Read scale factors
        for sb in 0..sblimit {
            for ch in 0..channels {
                if self.allocation[ch][sb].is_some() {
                    let buffer = &mut self.buffer;
                    let mut read = || buffer.read(6) as i32;
                    let sf = &mut self.scale_factor[ch][sb];
                    match self.scale_factor_info[ch][sb] {
                        0 => {
                            sf[0] = read();
                            sf[1] = read();
                            sf[2] = read();
                        }
                        1 => {
                            sf[0] = read();
                            sf[1] = sf[0];
                            sf[2] = read();
                        }
                        2 => {
                            sf[0] = read();
                            sf[1] = sf[0];
                            sf[2] = sf[0];
                        }
                        _ => {
                            sf[0] = read();
                            sf[1] = read();
                            sf[2] = sf[1];
                        }
                    }
                }
            }
            if self.mode == MODE_MONO {
                self.scale_factor[1][sb] = self.scale_factor[0][sb];
            }
        }

        // Coefficient input and reconstruction
        let mut out_pos = 0;
        for part in 0..3 {
            for _granule in 0..4 {
                // Read the samples
                for sb in 0..self.bound {
                    self.read_samples(0, sb, part);
                    self.read_samples(1, sb, part);
                }
                for sb in self.bound..sblimit {
                    self.read_samples(0, sb, part);
                    self.sample[1][sb] = self.sample[0][sb];
                }
                for sb in sblimit..32 {
                    self.sample[0][sb] = [0; 3];
                    self.sample[1][sb] = [0; 3];
                }

                // Synthesis loop
                for p in 0..3 {
                    // Shifting step
                    self.v_pos = (self.v_pos + 1024 - 64) & 1023;

                    let synthesis = &mut *self.synthesis;
                    for ch in 0..2 {
                        idct36(&self.sample[ch], p, &mut synthesis.v[ch], self.v_pos);

                        // Build U, windowing, calculate output
                        let u = &mut synthesis.u;
                        let d = &synthesis.d;
                        let v = &synthesis.v[ch];
                        *u = [0.0; 32];

                        let mut d_index = 512 - (self.v_pos >> 1);
                        let mut v_index = (self.v_pos % 128) >> 1;
                        while v_index < 1024 {
                            for u in u.iter_mut() {
                                *u += d[d_index] * v[v_index];
                                d_index += 1;
                                v_index += 1;
                            }

                            v_index += 128 - 32;
                            d_index += 64 - 32;
                        }

                        d_index -= 512 - 32;
                        v_index = (128 - 32 + 1024) - v_index;
                        while v_index < 1024 {
                            for u in u.iter_mut() {
                                *u += d[d_index] * v[v_index];
                                d_index += 1;
                                v_index += 1;
                            }

                            v_index += 128 - 32;
                            d_index += 64 - 32;
                        }

                        // Output samples
                        for (j, u) in u.iter().enumerate() {
                            synthesis.samples.interleaved[((out_pos + j) << 1) + ch] =
                                u / -1090519040.0;
                        }
                    } // End of synthesis channel loop
                    out_pos += 32;
                } // End of synthesis sub-block loop
            } // Decoding of the granule finished
        }

        self.buffer.align();
    }

    fn read_allocation(&mut self, sb: usize, tab3: usize) -> Option<&'static QuantizerSpec> {
        let tab4 = QUANT_LUT_STEP_3[tab3][sb];
        let qtab = QUANT_LUT_STEP_4[usize::from(tab4 & 15)]
            [self.buffer.read(u32::from(tab4 >> 4)) as usize];
        match qtab {
            0 => None,
            qtab => Some(&QUANT_TAB[usize::from(qtab) - 1]),
        }
    }

    fn read_samples(&mut self, ch: usize, sb: usize, part: usize) {
        let Some(q) = self.allocation[ch][sb] else {
            // No bits allocated for this subband
            self.sample[ch][sb] = [0; 3];
            return;
        };

        // Resolve scalefactor
        let mut sf = self.scale_factor[ch][sb][part];
        if sf == 63 {
            sf = 0;
        } else {
            let shift = sf / 3;
            sf = (SCALEFACTOR_BASE[(sf % 3) as usize] + ((1 << shift) >> 1)) >> shift;
        }

        // Decode samples
        let mut adj = i32::from(q.levels);
        let sample = &mut self.sample[ch][sb];
        if q.group {
            // Decode grouped samples
            let mut val = self.buffer.read(u32::from(q.bits)) as i32;
            sample[0] = val % adj;
            val /= adj;
            sample[1] = val % adj;
            sample[2] = val / adj;
        } else {
            // Decode direct samples
            sample[0] = self.buffer.read(u32::from(q.bits)) as i32;
            sample[1] = self.buffer.read(u32::from(q.bits)) as i32;
            sample[2] = self.buffer.read(u32::from(q.bits)) as i32;
        }

        // Postmultiply samples
        let scale = 65536 / (adj + 1);
        adj = ((adj + 1) >> 1) - 1;

        for sample in sample {
            let val = (adj - *sample) * scale;
            *sample = (val * (sf >> 12) + ((val * (sf & 4095) + 2048) >> 12)) >> 12;
        }
    }
}

impl fmt::Debug for AudioDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioDecoder")
            .field("info", &self.info())
            .field("time", &self.time())
            .finish_non_exhaustive()
    }
}

// The constants are kept as written in kjmp2 and PL_MPEG.
#[allow(
    unused_assignments,
    clippy::excessive_precision,
    clippy::approx_constant,
    clippy::identity_op
)]
fn idct36(s: &[[i32; 3]; 32], ss: usize, d: &mut [f32; 1024], dp: usize) {
    let (
        mut t01,
        mut t02,
        mut t03,
        mut t04,
        mut t05,
        mut t06,
        mut t07,
        mut t08,
        mut t09,
        mut t10,
        mut t11,
        mut t12,
    ): (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);
    let (
        mut t13,
        mut t14,
        mut t15,
        mut t16,
        mut t17,
        mut t18,
        mut t19,
        mut t20,
        mut t21,
        mut t22,
        mut t23,
        mut t24,
    ): (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);
    let (mut t25, mut t26, mut t27, mut t28, mut t29, mut t30, mut t31, mut t32, mut t33): (
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    );

    t01 = (s[0][ss] + s[31][ss]) as f32;
    t02 = (s[0][ss] - s[31][ss]) as f32 * 0.500602998235;
    t03 = (s[1][ss] + s[30][ss]) as f32;
    t04 = (s[1][ss] - s[30][ss]) as f32 * 0.505470959898;
    t05 = (s[2][ss] + s[29][ss]) as f32;
    t06 = (s[2][ss] - s[29][ss]) as f32 * 0.515447309923;
    t07 = (s[3][ss] + s[28][ss]) as f32;
    t08 = (s[3][ss] - s[28][ss]) as f32 * 0.53104259109;
    t09 = (s[4][ss] + s[27][ss]) as f32;
    t10 = (s[4][ss] - s[27][ss]) as f32 * 0.553103896034;
    t11 = (s[5][ss] + s[26][ss]) as f32;
    t12 = (s[5][ss] - s[26][ss]) as f32 * 0.582934968206;
    t13 = (s[6][ss] + s[25][ss]) as f32;
    t14 = (s[6][ss] - s[25][ss]) as f32 * 0.622504123036;
    t15 = (s[7][ss] + s[24][ss]) as f32;
    t16 = (s[7][ss] - s[24][ss]) as f32 * 0.674808341455;
    t17 = (s[8][ss] + s[23][ss]) as f32;
    t18 = (s[8][ss] - s[23][ss]) as f32 * 0.744536271002;
    t19 = (s[9][ss] + s[22][ss]) as f32;
    t20 = (s[9][ss] - s[22][ss]) as f32 * 0.839349645416;
    t21 = (s[10][ss] + s[21][ss]) as f32;
    t22 = (s[10][ss] - s[21][ss]) as f32 * 0.972568237862;
    t23 = (s[11][ss] + s[20][ss]) as f32;
    t24 = (s[11][ss] - s[20][ss]) as f32 * 1.16943993343;
    t25 = (s[12][ss] + s[19][ss]) as f32;
    t26 = (s[12][ss] - s[19][ss]) as f32 * 1.48416461631;
    t27 = (s[13][ss] + s[18][ss]) as f32;
    t28 = (s[13][ss] - s[18][ss]) as f32 * 2.05778100995;
    t29 = (s[14][ss] + s[17][ss]) as f32;
    t30 = (s[14][ss] - s[17][ss]) as f32 * 3.40760841847;
    t31 = (s[15][ss] + s[16][ss]) as f32;
    t32 = (s[15][ss] - s[16][ss]) as f32 * 10.1900081235;

    t33 = t01 + t31;
    t31 = (t01 - t31) * 0.502419286188;
    t01 = t03 + t29;
    t29 = (t03 - t29) * 0.52249861494;
    t03 = t05 + t27;
    t27 = (t05 - t27) * 0.566944034816;
    t05 = t07 + t25;
    t25 = (t07 - t25) * 0.64682178336;
    t07 = t09 + t23;
    t23 = (t09 - t23) * 0.788154623451;
    t09 = t11 + t21;
    t21 = (t11 - t21) * 1.06067768599;
    t11 = t13 + t19;
    t19 = (t13 - t19) * 1.72244709824;
    t13 = t15 + t17;
    t17 = (t15 - t17) * 5.10114861869;
    t15 = t33 + t13;
    t13 = (t33 - t13) * 0.509795579104;
    t33 = t01 + t11;
    t01 = (t01 - t11) * 0.601344886935;
    t11 = t03 + t09;
    t09 = (t03 - t09) * 0.899976223136;
    t03 = t05 + t07;
    t07 = (t05 - t07) * 2.56291544774;
    t05 = t15 + t03;
    t15 = (t15 - t03) * 0.541196100146;
    t03 = t33 + t11;
    t11 = (t33 - t11) * 1.30656296488;
    t33 = t05 + t03;
    t05 = (t05 - t03) * 0.707106781187;
    t03 = t15 + t11;
    t15 = (t15 - t11) * 0.707106781187;
    t03 += t15;
    t11 = t13 + t07;
    t13 = (t13 - t07) * 0.541196100146;
    t07 = t01 + t09;
    t09 = (t01 - t09) * 1.30656296488;
    t01 = t11 + t07;
    t07 = (t11 - t07) * 0.707106781187;
    t11 = t13 + t09;
    t13 = (t13 - t09) * 0.707106781187;
    t11 += t13;
    t01 += t11;
    t11 += t07;
    t07 += t13;
    t09 = t31 + t17;
    t31 = (t31 - t17) * 0.509795579104;
    t17 = t29 + t19;
    t29 = (t29 - t19) * 0.601344886935;
    t19 = t27 + t21;
    t21 = (t27 - t21) * 0.899976223136;
    t27 = t25 + t23;
    t23 = (t25 - t23) * 2.56291544774;
    t25 = t09 + t27;
    t09 = (t09 - t27) * 0.541196100146;
    t27 = t17 + t19;
    t19 = (t17 - t19) * 1.30656296488;
    t17 = t25 + t27;
    t27 = (t25 - t27) * 0.707106781187;
    t25 = t09 + t19;
    t19 = (t09 - t19) * 0.707106781187;
    t25 += t19;
    t09 = t31 + t23;
    t31 = (t31 - t23) * 0.541196100146;
    t23 = t29 + t21;
    t21 = (t29 - t21) * 1.30656296488;
    t29 = t09 + t23;
    t23 = (t09 - t23) * 0.707106781187;
    t09 = t31 + t21;
    t31 = (t31 - t21) * 0.707106781187;
    t09 += t31;
    t29 += t09;
    t09 += t23;
    t23 += t31;
    t17 += t29;
    t29 += t25;
    t25 += t09;
    t09 += t27;
    t27 += t23;
    t23 += t19;
    t19 += t31;
    t21 = t02 + t32;
    t02 = (t02 - t32) * 0.502419286188;
    t32 = t04 + t30;
    t04 = (t04 - t30) * 0.52249861494;
    t30 = t06 + t28;
    t28 = (t06 - t28) * 0.566944034816;
    t06 = t08 + t26;
    t08 = (t08 - t26) * 0.64682178336;
    t26 = t10 + t24;
    t10 = (t10 - t24) * 0.788154623451;
    t24 = t12 + t22;
    t22 = (t12 - t22) * 1.06067768599;
    t12 = t14 + t20;
    t20 = (t14 - t20) * 1.72244709824;
    t14 = t16 + t18;
    t16 = (t16 - t18) * 5.10114861869;
    t18 = t21 + t14;
    t14 = (t21 - t14) * 0.509795579104;
    t21 = t32 + t12;
    t32 = (t32 - t12) * 0.601344886935;
    t12 = t30 + t24;
    t24 = (t30 - t24) * 0.899976223136;
    t30 = t06 + t26;
    t26 = (t06 - t26) * 2.56291544774;
    t06 = t18 + t30;
    t18 = (t18 - t30) * 0.541196100146;
    t30 = t21 + t12;
    t12 = (t21 - t12) * 1.30656296488;
    t21 = t06 + t30;
    t30 = (t06 - t30) * 0.707106781187;
    t06 = t18 + t12;
    t12 = (t18 - t12) * 0.707106781187;
    t06 += t12;
    t18 = t14 + t26;
    t26 = (t14 - t26) * 0.541196100146;
    t14 = t32 + t24;
    t24 = (t32 - t24) * 1.30656296488;
    t32 = t18 + t14;
    t14 = (t18 - t14) * 0.707106781187;
    t18 = t26 + t24;
    t24 = (t26 - t24) * 0.707106781187;
    t18 += t24;
    t32 += t18;
    t18 += t14;
    t26 = t14 + t24;
    t14 = t02 + t16;
    t02 = (t02 - t16) * 0.509795579104;
    t16 = t04 + t20;
    t04 = (t04 - t20) * 0.601344886935;
    t20 = t28 + t22;
    t22 = (t28 - t22) * 0.899976223136;
    t28 = t08 + t10;
    t10 = (t08 - t10) * 2.56291544774;
    t08 = t14 + t28;
    t14 = (t14 - t28) * 0.541196100146;
    t28 = t16 + t20;
    t20 = (t16 - t20) * 1.30656296488;
    t16 = t08 + t28;
    t28 = (t08 - t28) * 0.707106781187;
    t08 = t14 + t20;
    t20 = (t14 - t20) * 0.707106781187;
    t08 += t20;
    t14 = t02 + t10;
    t02 = (t02 - t10) * 0.541196100146;
    t10 = t04 + t22;
    t22 = (t04 - t22) * 1.30656296488;
    t04 = t14 + t10;
    t10 = (t14 - t10) * 0.707106781187;
    t14 = t02 + t22;
    t02 = (t02 - t22) * 0.707106781187;
    t14 += t02;
    t04 += t14;
    t14 += t10;
    t10 += t02;
    t16 += t04;
    t04 += t08;
    t08 += t14;
    t14 += t28;
    t28 += t10;
    t10 += t20;
    t20 += t02;
    t21 += t16;
    t16 += t32;
    t32 += t04;
    t04 += t06;
    t06 += t08;
    t08 += t18;
    t18 += t14;
    t14 += t30;
    t30 += t28;
    t28 += t26;
    t26 += t10;
    t10 += t12;
    t12 += t20;
    t20 += t24;
    t24 += t02;

    d[dp + 48] = -t33;
    d[dp + 49] = -t21;
    d[dp + 47] = -t21;
    d[dp + 50] = -t17;
    d[dp + 46] = -t17;
    d[dp + 51] = -t16;
    d[dp + 45] = -t16;
    d[dp + 52] = -t01;
    d[dp + 44] = -t01;
    d[dp + 53] = -t32;
    d[dp + 43] = -t32;
    d[dp + 54] = -t29;
    d[dp + 42] = -t29;
    d[dp + 55] = -t04;
    d[dp + 41] = -t04;
    d[dp + 56] = -t03;
    d[dp + 40] = -t03;
    d[dp + 57] = -t06;
    d[dp + 39] = -t06;
    d[dp + 58] = -t25;
    d[dp + 38] = -t25;
    d[dp + 59] = -t08;
    d[dp + 37] = -t08;
    d[dp + 60] = -t11;
    d[dp + 36] = -t11;
    d[dp + 61] = -t18;
    d[dp + 35] = -t18;
    d[dp + 62] = -t09;
    d[dp + 34] = -t09;
    d[dp + 63] = -t14;
    d[dp + 33] = -t14;
    d[dp + 32] = -t05;
    d[dp + 0] = t05;
    d[dp + 31] = -t30;
    d[dp + 1] = t30;
    d[dp + 30] = -t27;
    d[dp + 2] = t27;
    d[dp + 29] = -t28;
    d[dp + 3] = t28;
    d[dp + 28] = -t07;
    d[dp + 4] = t07;
    d[dp + 27] = -t26;
    d[dp + 5] = t26;
    d[dp + 26] = -t23;
    d[dp + 6] = t23;
    d[dp + 25] = -t10;
    d[dp + 7] = t10;
    d[dp + 24] = -t15;
    d[dp + 8] = t15;
    d[dp + 23] = -t12;
    d[dp + 9] = t12;
    d[dp + 22] = -t19;
    d[dp + 10] = t19;
    d[dp + 21] = -t20;
    d[dp + 11] = t20;
    d[dp + 20] = -t13;
    d[dp + 12] = t13;
    d[dp + 19] = -t24;
    d[dp + 13] = t24;
    d[dp + 18] = -t31;
    d[dp + 14] = t31;
    d[dp + 17] = -t02;
    d[dp + 15] = t02;
    d[dp + 16] = 0.0;
}

#[rustfmt::skip]
const SYNTHESIS_WINDOW: [f32; 512] = [
         0.0,     -0.5,     -0.5,     -0.5,     -0.5,     -0.5,
        -0.5,     -1.0,     -1.0,     -1.0,     -1.0,     -1.5,
        -1.5,     -2.0,     -2.0,     -2.5,     -2.5,     -3.0,
        -3.5,     -3.5,     -4.0,     -4.5,     -5.0,     -5.5,
        -6.5,     -7.0,     -8.0,     -8.5,     -9.5,    -10.5,
       -12.0,    -13.0,    -14.5,    -15.5,    -17.5,    -19.0,
       -20.5,    -22.5,    -24.5,    -26.5,    -29.0,    -31.5,
       -34.0,    -36.5,    -39.5,    -42.5,    -45.5,    -48.5,
       -52.0,    -55.5,    -58.5,    -62.5,    -66.0,    -69.5,
       -73.5,    -77.0,    -80.5,    -84.5,    -88.0,    -91.5,
       -95.0,    -98.0,   -101.0,   -104.0,    106.5,    109.0,
       111.0,    112.5,    113.5,    114.0,    114.0,    113.5,
       112.0,    110.5,    107.5,    104.0,    100.0,     94.5,
        88.5,     81.5,     73.0,     63.5,     53.0,     41.5,
        28.5,     14.5,     -1.0,    -18.0,    -36.0,    -55.5,
       -76.5,    -98.5,   -122.0,   -147.0,   -173.5,   -200.5,
      -229.5,   -259.5,   -290.5,   -322.5,   -355.5,   -389.5,
      -424.0,   -459.5,   -495.5,   -532.0,   -568.5,   -605.0,
      -641.5,   -678.0,   -714.0,   -749.0,   -783.5,   -817.0,
      -849.0,   -879.5,   -908.5,   -935.0,   -959.5,   -981.0,
     -1000.5,  -1016.0,  -1028.5,  -1037.5,  -1042.5,  -1043.5,
     -1040.0,  -1031.5,   1018.5,   1000.0,    976.0,    946.5,
       911.0,    869.5,    822.0,    767.5,    707.0,    640.0,
       565.5,    485.0,    397.0,    302.5,    201.0,     92.5,
       -22.5,   -144.0,   -272.5,   -407.0,   -547.5,   -694.0,
      -846.0,  -1003.0,  -1165.0,  -1331.5,  -1502.0,  -1675.5,
     -1852.5,  -2031.5,  -2212.5,  -2394.0,  -2576.5,  -2758.5,
     -2939.5,  -3118.5,  -3294.5,  -3467.5,  -3635.5,  -3798.5,
     -3955.0,  -4104.5,  -4245.5,  -4377.5,  -4499.0,  -4609.5,
     -4708.0,  -4792.5,  -4863.5,  -4919.0,  -4958.0,  -4979.5,
     -4983.0,  -4967.5,  -4931.5,  -4875.0,  -4796.0,  -4694.5,
     -4569.5,  -4420.0,  -4246.0,  -4046.0,  -3820.0,  -3567.0,
      3287.0,   2979.5,   2644.0,   2280.5,   1888.0,   1467.5,
      1018.5,    541.0,     35.0,   -499.0,  -1061.0,  -1650.0,
     -2266.5,  -2909.0,  -3577.0,  -4270.0,  -4987.5,  -5727.5,
     -6490.0,  -7274.0,  -8077.5,  -8899.5,  -9739.0, -10594.5,
    -11464.5, -12347.0, -13241.0, -14144.5, -15056.0, -15973.5,
    -16895.5, -17820.0, -18744.5, -19668.0, -20588.0, -21503.0,
    -22410.5, -23308.5, -24195.0, -25068.5, -25926.5, -26767.0,
    -27589.0, -28389.0, -29166.5, -29919.0, -30644.5, -31342.0,
    -32009.5, -32645.0, -33247.0, -33814.5, -34346.0, -34839.5,
    -35295.0, -35710.0, -36084.5, -36417.5, -36707.5, -36954.0,
    -37156.5, -37315.0, -37428.0, -37496.0,  37519.0,  37496.0,
     37428.0,  37315.0,  37156.5,  36954.0,  36707.5,  36417.5,
     36084.5,  35710.0,  35295.0,  34839.5,  34346.0,  33814.5,
     33247.0,  32645.0,  32009.5,  31342.0,  30644.5,  29919.0,
     29166.5,  28389.0,  27589.0,  26767.0,  25926.5,  25068.5,
     24195.0,  23308.5,  22410.5,  21503.0,  20588.0,  19668.0,
     18744.5,  17820.0,  16895.5,  15973.5,  15056.0,  14144.5,
     13241.0,  12347.0,  11464.5,  10594.5,   9739.0,   8899.5,
      8077.5,   7274.0,   6490.0,   5727.5,   4987.5,   4270.0,
      3577.0,   2909.0,   2266.5,   1650.0,   1061.0,    499.0,
       -35.0,   -541.0,  -1018.5,  -1467.5,  -1888.0,  -2280.5,
     -2644.0,  -2979.5,   3287.0,   3567.0,   3820.0,   4046.0,
      4246.0,   4420.0,   4569.5,   4694.5,   4796.0,   4875.0,
      4931.5,   4967.5,   4983.0,   4979.5,   4958.0,   4919.0,
      4863.5,   4792.5,   4708.0,   4609.5,   4499.0,   4377.5,
      4245.5,   4104.5,   3955.0,   3798.5,   3635.5,   3467.5,
      3294.5,   3118.5,   2939.5,   2758.5,   2576.5,   2394.0,
      2212.5,   2031.5,   1852.5,   1675.5,   1502.0,   1331.5,
      1165.0,   1003.0,    846.0,    694.0,    547.5,    407.0,
       272.5,    144.0,     22.5,    -92.5,   -201.0,   -302.5,
      -397.0,   -485.0,   -565.5,   -640.0,   -707.0,   -767.5,
      -822.0,   -869.5,   -911.0,   -946.5,   -976.0,  -1000.0,
      1018.5,   1031.5,   1040.0,   1043.5,   1042.5,   1037.5,
      1028.5,   1016.0,   1000.5,    981.0,    959.5,    935.0,
       908.5,    879.5,    849.0,    817.0,    783.5,    749.0,
       714.0,    678.0,    641.5,    605.0,    568.5,    532.0,
       495.5,    459.5,    424.0,    389.5,    355.5,    322.5,
       290.5,    259.5,    229.5,    200.5,    173.5,    147.0,
       122.0,     98.5,     76.5,     55.5,     36.0,     18.0,
         1.0,    -14.5,    -28.5,    -41.5,    -53.0,    -63.5,
       -73.0,    -81.5,    -88.5,    -94.5,   -100.0,   -104.0,
      -107.5,   -110.5,   -112.0,   -113.5,   -114.0,   -114.0,
      -113.5,   -112.5,   -111.0,   -109.0,    106.5,    104.0,
       101.0,     98.0,     95.0,     91.5,     88.0,     84.5,
        80.5,     77.0,     73.5,     69.5,     66.0,     62.5,
        58.5,     55.5,     52.0,     48.5,     45.5,     42.5,
        39.5,     36.5,     34.0,     31.5,     29.0,     26.5,
        24.5,     22.5,     20.5,     19.0,     17.5,     15.5,
        14.5,     13.0,     12.0,     10.5,      9.5,      8.5,
         8.0,      7.0,      6.5,      5.5,      5.0,      4.5,
         4.0,      3.5,      3.5,      3.0,      2.5,      2.5,
         2.0,      2.0,      1.5,      1.5,      1.0,      1.0,
         1.0,      1.0,      0.5,      0.5,      0.5,      0.5,
         0.5,      0.5,
];

// Quantizer lookup, step 1: bitrate classes
#[rustfmt::skip]
const QUANT_LUT_STEP_1: [[u8; 16]; 2] = [
    // 32, 48, 56, 64, 80, 96,112,128,160,192,224,256,320,384 <- bitrate
    [ 0,  0,  1,  1,  1,  2,  2,  2,  2,  2,  2,  2,  2,  2, 0, 0], // mono
    // 16, 24, 28, 32, 40, 48, 56, 64, 80, 96,112,128,160,192 <- bitrate / chan
    [ 0,  0,  0,  0,  0,  0,  1,  1,  1,  2,  2,  2,  2,  2, 0, 0], // stereo
];

// Quantizer lookup, step 2: bitrate class, sample rate -> B2 table idx, sblimit
const QUANT_TAB_A: u8 = 27 | 64; // Table 3-B.2a: high-rate, sblimit = 27
const QUANT_TAB_B: u8 = 30 | 64; // Table 3-B.2b: high-rate, sblimit = 30
const QUANT_TAB_C: u8 = 8; // Table 3-B.2c:  low-rate, sblimit =  8
const QUANT_TAB_D: u8 = 12; // Table 3-B.2d:  low-rate, sblimit = 12

#[rustfmt::skip]
const QUANT_LUT_STEP_2: [[u8; 3]; 3] = [
    // 44.1 kHz,    48 kHz,      32 kHz
    [QUANT_TAB_C, QUANT_TAB_C, QUANT_TAB_D], // 32 - 48 kbit/sec/ch
    [QUANT_TAB_A, QUANT_TAB_A, QUANT_TAB_A], // 56 - 80 kbit/sec/ch
    [QUANT_TAB_B, QUANT_TAB_A, QUANT_TAB_B], // 96+     kbit/sec/ch
];

// Quantizer lookup, step 3: B2 table, subband -> nbal, row index
// (upper 4 bits: nbal, lower 4 bits: row index)
#[rustfmt::skip]
const QUANT_LUT_STEP_3: [[u8; 32]; 3] = [
    // Low-rate table (3-B.2c and 3-B.2d)
    [
        0x44, 0x44,
        0x34, 0x34, 0x34, 0x34, 0x34, 0x34, 0x34, 0x34, 0x34, 0x34,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    // High-rate table (3-B.2a and 3-B.2b)
    [
        0x43, 0x43, 0x43,
        0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
        0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0, 0,
    ],
    // MPEG-2 LSR table (B.2 in ISO 13818-3)
    [
        0x45, 0x45, 0x45, 0x45,
        0x34, 0x34, 0x34, 0x34, 0x34, 0x34, 0x34,
        0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24,
        0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24,
        0, 0,
    ],
];

// Quantizer lookup, step 4: table row, allocation[] value -> quant table index
#[rustfmt::skip]
const QUANT_LUT_STEP_4: [[u8; 16]; 6] = [
    [0, 1, 2, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 2,  3, 4, 5, 6, 17, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 2,  3, 4, 5, 6,  7,  8,  9, 10, 11, 12, 13, 14, 17],
    [0, 1, 3,  5, 6, 7, 8,  9, 10, 11, 12, 13, 14, 15, 16, 17],
    [0, 1, 2,  4, 5, 6, 7,  8,  9, 10, 11, 12, 13, 14, 15, 17],
    [0, 1, 2,  3, 4, 5, 6,  7,  8,  9, 10, 11, 12, 13, 14, 15],
];

struct QuantizerSpec {
    levels: u16,
    group: bool,
    bits: u8,
}

const fn quant(levels: u16, group: u8, bits: u8) -> QuantizerSpec {
    QuantizerSpec {
        levels,
        group: group != 0,
        bits,
    }
}

#[rustfmt::skip]
static QUANT_TAB: [QuantizerSpec; 17] = [
    quant(    3, 1,  5), //  1
    quant(    5, 1,  7), //  2
    quant(    7, 0,  3), //  3
    quant(    9, 1, 10), //  4
    quant(   15, 0,  4), //  5
    quant(   31, 0,  5), //  6
    quant(   63, 0,  6), //  7
    quant(  127, 0,  7), //  8
    quant(  255, 0,  8), //  9
    quant(  511, 0,  9), // 10
    quant( 1023, 0, 10), // 11
    quant( 2047, 0, 11), // 12
    quant( 4095, 0, 12), // 13
    quant( 8191, 0, 13), // 14
    quant(16383, 0, 14), // 15
    quant(32767, 0, 15), // 16
    quant(65535, 0, 16), // 17
];
