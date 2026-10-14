// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! Decoded video frames.

use std::fmt;
use std::time::Duration;

use crate::seconds;

/// A decoded video frame in YCbCr 4:2:0 format.
///
/// The three planes are stored at a size rounded up to whole macroblocks
/// (16×16 pixels), which may be larger than the display size of the frame.
///
/// The planes can be converted to RGB on the CPU with [`Frame::write_pixels`],
/// or on the GPU with the following BT.601 matrix:
///
/// ```glsl
/// mat4 bt601 = mat4(
///     1.16438,  0.00000,  1.59603, -0.87079,
///     1.16438, -0.39176, -0.81297,  0.52959,
///     1.16438,  2.01723,  0.00000, -1.08139,
///     0, 0, 0, 1
/// );
/// gl_FragColor = vec4(y, cb, cr, 1.0) * bt601;
/// ```
#[derive(Clone)]
pub struct Frame {
    pub(crate) time: f64,
    width: usize,
    height: usize,
    luma_width: usize,
    luma_height: usize,
    chroma_width: usize,
    chroma_height: usize,
    /// The Y, Cr and Cb planes back to back, followed by some padding so that
    /// motion compensation never reads out of bounds.
    pub(crate) data: Vec<u8>,
}

impl Frame {
    pub(crate) fn new(width: usize, height: usize, mb_width: usize, mb_height: usize) -> Self {
        let luma_width = mb_width << 4;
        let luma_height = mb_height << 4;
        let chroma_width = mb_width << 3;
        let chroma_height = mb_height << 3;
        let size = luma_width * luma_height + 2 * chroma_width * chroma_height + chroma_width + 1;
        Frame {
            time: 0.0,
            width,
            height,
            luma_width,
            luma_height,
            chroma_width,
            chroma_height,
            data: vec![0; size],
        }
    }

    pub(crate) fn cr_offset(&self) -> usize {
        self.luma_width * self.luma_height
    }

    pub(crate) fn cb_offset(&self) -> usize {
        self.cr_offset() + self.chroma_width * self.chroma_height
    }

    /// The presentation time of this frame.
    pub fn time(&self) -> Duration {
        seconds(self.time)
    }

    /// The display width of this frame.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The display height of this frame.
    pub fn height(&self) -> usize {
        self.height
    }

    /// The luma (Y) plane.
    pub fn y(&self) -> Plane<'_> {
        Plane {
            width: self.luma_width,
            height: self.luma_height,
            data: &self.data[..self.cr_offset()],
        }
    }

    /// The red difference chroma (Cr) plane, at half the luma resolution.
    pub fn cr(&self) -> Plane<'_> {
        Plane {
            width: self.chroma_width,
            height: self.chroma_height,
            data: &self.data[self.cr_offset()..self.cb_offset()],
        }
    }

    /// The blue difference chroma (Cb) plane, at half the luma resolution.
    pub fn cb(&self) -> Plane<'_> {
        let start = self.cb_offset();
        Plane {
            width: self.chroma_width,
            height: self.chroma_height,
            data: &self.data[start..start + self.chroma_width * self.chroma_height],
        }
    }

    /// Converts the frame to packed RGB pixels and returns them, with rows
    /// of exactly `width() * format.bytes_per_pixel()` bytes.
    ///
    /// Alpha components are set to 255.
    #[must_use]
    pub fn to_pixels(&self, format: PixelFormat) -> Vec<u8> {
        let stride = self.width * format.bytes_per_pixel();
        let mut pixels = vec![0xff; stride * self.height];
        self.write_pixels(format, &mut pixels, stride);
        pixels
    }

    /// Converts the frame to RGB pixels in `dest`, where each row starts
    /// `stride` bytes after the previous one. Alpha components in `dest` are
    /// left untouched.
    ///
    /// Conversion follows the BT.601 standard and works on 2×2 pixel blocks;
    /// for odd display sizes the last column or row is left untouched.
    ///
    /// # Panics
    ///
    /// Panics if `stride` is smaller than `width() * format.bytes_per_pixel()`
    /// or `dest` is too small to hold `height()` rows.
    pub fn write_pixels(&self, format: PixelFormat, dest: &mut [u8], stride: usize) {
        let row_size = self.width * format.bytes_per_pixel();
        assert!(
            stride >= row_size,
            "stride {stride} is smaller than a row ({row_size} bytes)"
        );
        let needed = if self.height == 0 {
            0
        } else {
            stride * (self.height - 1) + row_size
        };
        assert!(
            dest.len() >= needed,
            "destination holds {} bytes, but {needed} are needed",
            dest.len()
        );

        match format {
            PixelFormat::Rgb => self.convert::<3, 0, 1, 2>(dest, stride),
            PixelFormat::Bgr => self.convert::<3, 2, 1, 0>(dest, stride),
            PixelFormat::Rgba => self.convert::<4, 0, 1, 2>(dest, stride),
            PixelFormat::Bgra => self.convert::<4, 2, 1, 0>(dest, stride),
            PixelFormat::Argb => self.convert::<4, 1, 2, 3>(dest, stride),
            PixelFormat::Abgr => self.convert::<4, 3, 2, 1>(dest, stride),
        }
    }

    // YCbCr conversion following the BT.601 standard:
    // https://infogalactic.com/info/YCbCr#ITU-R_BT.601_conversion
    fn convert<const BYTES_PER_PIXEL: usize, const RI: usize, const GI: usize, const BI: usize>(
        &self,
        dest: &mut [u8],
        stride: usize,
    ) {
        let luma = self.y().data;
        let cr_plane = self.cr().data;
        let cb_plane = self.cb().data;

        let cols = self.width >> 1;
        let rows = self.height >> 1;
        let yw = self.luma_width;
        let cw = self.chroma_width;
        for row in 0..rows {
            for col in 0..cols {
                let c_index = row * cw + col;
                let y_index = row * 2 * yw + col * 2;
                let d_index = row * 2 * stride + col * 2 * BYTES_PER_PIXEL;

                let cr = i32::from(cr_plane[c_index]) - 128;
                let cb = i32::from(cb_plane[c_index]) - 128;
                let r = (cr * 104597) >> 16;
                let g = (cb * 25674 + cr * 53278) >> 16;
                let b = (cb * 132201) >> 16;

                let mut put_pixel = |y_offset: usize, dest_offset: usize| {
                    let y = ((i32::from(luma[y_index + y_offset]) - 16) * 76309) >> 16;
                    let pixel = &mut dest[d_index + dest_offset..];
                    pixel[RI] = clamp(y + r);
                    pixel[GI] = clamp(y - g);
                    pixel[BI] = clamp(y + b);
                };
                put_pixel(0, 0);
                put_pixel(1, BYTES_PER_PIXEL);
                put_pixel(yw, stride);
                put_pixel(yw + 1, stride + BYTES_PER_PIXEL);
            }
        }
    }
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
            .field("time", &self.time())
            .field("width", &self.width)
            .field("height", &self.height)
            .finish_non_exhaustive()
    }
}

/// Clamps `n` to the range of a byte.
pub(crate) fn clamp(n: i32) -> u8 {
    n.clamp(0, 255) as u8
}

/// A single plane of a [`Frame`].
#[derive(Clone, Copy)]
pub struct Plane<'a> {
    width: usize,
    height: usize,
    data: &'a [u8],
}

impl<'a> Plane<'a> {
    /// The width of the plane, which is also the number of bytes per row.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height of the plane.
    pub fn height(&self) -> usize {
        self.height
    }

    /// The samples of the plane, `width() * height()` bytes row by row.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

impl fmt::Debug for Plane<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Plane")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish_non_exhaustive()
    }
}

/// The layout of an RGB pixel, for [`Frame::write_pixels`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// Red, green, blue; 3 bytes per pixel.
    Rgb,
    /// Blue, green, red; 3 bytes per pixel.
    Bgr,
    /// Red, green, blue, alpha; 4 bytes per pixel.
    Rgba,
    /// Blue, green, red, alpha; 4 bytes per pixel.
    Bgra,
    /// Alpha, red, green, blue; 4 bytes per pixel.
    Argb,
    /// Alpha, blue, green, red; 4 bytes per pixel.
    Abgr,
}

impl PixelFormat {
    /// The number of bytes per pixel.
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            PixelFormat::Rgb | PixelFormat::Bgr => 3,
            PixelFormat::Rgba | PixelFormat::Bgra | PixelFormat::Argb | PixelFormat::Abgr => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_grey_to_rgb() {
        let mut frame = Frame::new(4, 2, 1, 1);
        let cr = frame.cr_offset();
        frame.data[..cr].fill(126);
        frame.data[cr..].fill(128);

        let rgba = frame.to_pixels(PixelFormat::Rgba);
        assert_eq!(rgba.len(), 4 * 2 * 4);
        assert!(rgba.chunks(4).all(|pixel| pixel == [128, 128, 128, 255]));

        let mut argb = [7; 4 * 4 * 2];
        frame.write_pixels(PixelFormat::Argb, &mut argb, 16);
        assert!(argb.chunks(4).all(|pixel| pixel == [7, 128, 128, 128]));
    }

    #[test]
    #[should_panic(expected = "stride")]
    fn rejects_short_stride() {
        let frame = Frame::new(4, 2, 1, 1);
        frame.write_pixels(PixelFormat::Rgb, &mut [0; 100], 11);
    }
}
