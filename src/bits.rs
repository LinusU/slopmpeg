// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

//! Bit level reading of buffered data, shared by the demuxer and decoders.

use std::io::{self, Read, Seek, SeekFrom};

/// Number of bytes requested from a reader per fill.
const READ_CHUNK_SIZE: usize = 64 * 1024;

/// What happened when a [`Source`] was asked for more data.
pub(crate) enum Fill {
    /// Some bytes were appended.
    More,
    /// No bytes are available right now, but more may be written later.
    Pending,
    /// No more bytes will ever be available.
    End,
}

/// Something a [`BitReader`] can pull more data from when it runs dry.
pub(crate) trait Source {
    /// Appends more bytes to `buf`.
    fn fill(&mut self, buf: &mut Vec<u8>) -> Fill;
}

/// A source that never produces data on its own; data is pushed into the
/// buffer with [`BitReader::write`] instead.
impl Source for () {
    fn fill(&mut self, _buf: &mut Vec<u8>) -> Fill {
        Fill::Pending
    }
}

/// A source backed by an [`io::Read`] implementation.
pub(crate) struct ReadSource<R> {
    reader: R,
    /// Number of bytes read from `reader` since it was handed to us, i.e. the
    /// stream position relative to where it was at that point.
    position: u64,
    /// The total stream length, once known.
    length: Option<u64>,
    /// The first I/O error encountered; it ends the stream until taken.
    error: Option<io::Error>,
}

impl<R> ReadSource<R> {
    pub(crate) fn new(reader: R) -> Self {
        ReadSource {
            reader,
            position: 0,
            length: None,
            error: None,
        }
    }

    pub(crate) fn take_error(&mut self) -> Option<io::Error> {
        self.error.take()
    }
}

impl<R: Read> Source for ReadSource<R> {
    fn fill(&mut self, buf: &mut Vec<u8>) -> Fill {
        if self.error.is_some() {
            return Fill::End;
        }

        let start = buf.len();
        buf.resize(start + READ_CHUNK_SIZE, 0);
        let result = loop {
            match self.reader.read(&mut buf[start..]) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                result => break result,
            }
        };

        match result {
            Ok(0) => {
                buf.truncate(start);
                Fill::End
            }
            Ok(n) => {
                buf.truncate(start + n);
                self.position += n as u64;
                Fill::More
            }
            Err(e) => {
                buf.truncate(start);
                self.error = Some(e);
                Fill::End
            }
        }
    }
}

impl<R: Seek> ReadSource<R> {
    fn seek(&mut self, position: u64) {
        let offset = position as i128 - self.position as i128;
        let result = i64::try_from(offset)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "seek offset out of range"))
            .and_then(|offset| self.reader.seek(SeekFrom::Current(offset)));
        match result {
            Ok(_) => self.position = position,
            Err(e) => self.error = Some(e),
        }
    }

    fn length(&mut self) -> u64 {
        if let Some(length) = self.length {
            return length;
        }

        let result = (|| {
            let current = self.reader.stream_position()?;
            let end = self.reader.seek(SeekFrom::End(0))?;
            self.reader.seek(SeekFrom::Start(current))?;
            Ok(end - (current - self.position))
        })();

        match result {
            Ok(length) => {
                self.length = Some(length);
                length
            }
            Err(e) => {
                self.error = Some(e);
                0
            }
        }
    }
}

/// One entry of a variable length code lookup table. Positive `index` values
/// point to the next pair of entries; zero marks a leaf holding `value`.
#[derive(Clone, Copy)]
pub(crate) struct Vlc<T> {
    pub(crate) index: i16,
    pub(crate) value: T,
}

/// Shorthand for building [`Vlc`] tables.
pub(crate) const fn vlc<T>(index: i16, value: T) -> Vlc<T> {
    Vlc { index, value }
}

/// A byte buffer that is read one bit at a time.
pub(crate) struct BitReader<S> {
    pub(crate) bytes: Vec<u8>,
    pub(crate) bit_index: usize,
    discard_read_bytes: bool,
    /// No more data will be added beyond what is in `bytes`.
    end_of_data: bool,
    /// A read was attempted past the end of the data.
    has_ended: bool,
    pub(crate) source: S,
}

impl<S> BitReader<S> {
    pub(crate) fn new(source: S) -> Self {
        BitReader {
            bytes: Vec::new(),
            bit_index: 0,
            discard_read_bytes: true,
            end_of_data: false,
            has_ended: false,
            source,
        }
    }

    /// Returns whether the read position is at the end and no more data is
    /// expected.
    pub(crate) fn has_ended(&self) -> bool {
        self.has_ended
    }
}

impl<S: Source> BitReader<S> {
    fn available_bits(&self) -> usize {
        (self.bytes.len() << 3).saturating_sub(self.bit_index)
    }

    /// Returns whether at least `count` bits are available, pulling more data
    /// from the source if needed.
    pub(crate) fn has(&mut self, count: usize) -> bool {
        if self.available_bits() >= count {
            return true;
        }

        while !self.end_of_data {
            if self.discard_read_bytes {
                self.discard_read_bytes();
            }
            match self.source.fill(&mut self.bytes) {
                Fill::More => {
                    if self.available_bits() >= count {
                        return true;
                    }
                }
                Fill::Pending => break,
                Fill::End => self.end_of_data = true,
            }
        }

        if self.end_of_data {
            self.has_ended = true;
        }
        false
    }

    /// Reads `count` (at most 32) bits as an unsigned integer. Returns 0 if
    /// not enough data is available.
    pub(crate) fn read(&mut self, count: u32) -> u32 {
        if !self.has(count as usize) {
            return 0;
        }

        let mut value = 0;
        let mut count = count;
        while count > 0 {
            let current_byte = u32::from(self.bytes[self.bit_index >> 3]);

            let remaining = 8 - (self.bit_index & 7) as u32; // Remaining bits in byte
            let read = remaining.min(count); // Bits in this run
            let shift = remaining - read;
            let mask = 0xff >> (8 - read);

            value = (value << read) | ((current_byte & (mask << shift)) >> shift);

            self.bit_index += read as usize;
            count -= read;
        }

        value
    }

    /// Reads a single bit as a boolean.
    pub(crate) fn read_bit(&mut self) -> bool {
        self.read(1) != 0
    }

    /// Advances to the next byte boundary.
    pub(crate) fn align(&mut self) {
        self.bit_index = ((self.bit_index + 7) >> 3) << 3;
    }

    /// Skips `count` bits, if that many are available.
    pub(crate) fn skip(&mut self, count: usize) {
        if self.has(count) {
            self.bit_index += count;
        }
    }

    /// Aligns to the next byte and skips all following bytes equal to `v`.
    /// Returns the number of bytes skipped.
    pub(crate) fn skip_bytes(&mut self, v: u8) -> usize {
        self.align();
        let mut skipped = 0;
        while self.has(8) && self.bytes[self.bit_index >> 3] == v {
            self.bit_index += 8;
            skipped += 1;
        }
        skipped
    }

    /// Advances past the next start code (`00 00 01 xx`) and returns its code
    /// byte, or `None` if no start code was found in the available data.
    pub(crate) fn next_start_code(&mut self) -> Option<u8> {
        self.align();

        while self.has(5 << 3) {
            let byte_index = self.bit_index >> 3;
            if self.bytes[byte_index..byte_index + 3] == [0x00, 0x00, 0x01] {
                self.bit_index = (byte_index + 4) << 3;
                return Some(self.bytes[byte_index + 3]);
            }
            self.bit_index += 8;
        }
        None
    }

    /// Advances past the next start code with the given code byte. Returns
    /// whether it was found.
    pub(crate) fn find_start_code(&mut self, code: u8) -> bool {
        loop {
            match self.next_start_code() {
                Some(current) if current == code => return true,
                Some(_) => {}
                None => return false,
            }
        }
    }

    /// Returns whether a start code with the given code byte follows, without
    /// moving the read position.
    pub(crate) fn has_start_code(&mut self, code: u8) -> bool {
        let previous_bit_index = self.bit_index;
        let previous_discard_read_bytes = self.discard_read_bytes;

        self.discard_read_bytes = false;
        let found = self.find_start_code(code);

        self.bit_index = previous_bit_index;
        self.discard_read_bytes = previous_discard_read_bytes;
        found
    }

    /// Returns whether the next `bit_count` bits are not all zero, without
    /// moving the read position.
    pub(crate) fn peek_non_zero(&mut self, bit_count: u32) -> bool {
        if !self.has(bit_count as usize) {
            return false;
        }

        let val = self.read(bit_count);
        self.bit_index -= bit_count as usize;
        val != 0
    }

    /// Reads one variable length code using the given lookup table.
    pub(crate) fn read_vlc<T: Copy>(&mut self, table: &[Vlc<T>]) -> T {
        let mut index = 0;
        loop {
            let state = table[index + self.read(1) as usize];
            if state.index <= 0 {
                return state.value;
            }
            index = state.index as usize;
        }
    }

    /// Moves all unread data to the start of the buffer, dropping what has
    /// already been read.
    pub(crate) fn discard_read_bytes(&mut self) {
        let byte_pos = (self.bit_index >> 3).min(self.bytes.len());
        if byte_pos == self.bytes.len() {
            self.bit_index = 0;
            self.bytes.clear();
        } else if byte_pos > 0 {
            self.bytes.drain(..byte_pos);
            self.bit_index -= byte_pos << 3;
        }
    }
}

impl BitReader<()> {
    /// Appends data to the buffer.
    pub(crate) fn write(&mut self, data: &[u8]) {
        if self.discard_read_bytes {
            self.discard_read_bytes();
        }
        self.bytes.extend_from_slice(data);
        self.end_of_data = false;
        self.has_ended = false;
    }

    /// Marks the current data as all there will be, until the next write.
    pub(crate) fn signal_end(&mut self) {
        self.end_of_data = true;
    }

    /// Drops all data and clears the end of data state.
    pub(crate) fn clear(&mut self) {
        self.bytes.clear();
        self.bit_index = 0;
        self.end_of_data = false;
        self.has_ended = false;
    }
}

impl<R: Read> BitReader<ReadSource<R>> {
    /// The stream position of the read pointer, in bytes.
    pub(crate) fn tell(&self) -> u64 {
        self.source.position - self.bytes.len() as u64 + (self.bit_index >> 3) as u64
    }
}

impl<R: Read + Seek> BitReader<ReadSource<R>> {
    /// Moves the read pointer to the given stream position, in bytes.
    pub(crate) fn seek(&mut self, position: u64) {
        self.has_ended = false;
        self.end_of_data = false;
        self.source.seek(position);
        self.bit_index = 0;
        self.bytes.clear();
    }

    /// The total length of the stream, in bytes.
    pub(crate) fn length(&mut self) -> u64 {
        self.source.length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pushed(data: &[u8]) -> BitReader<()> {
        let mut reader = BitReader::new(());
        reader.write(data);
        reader
    }

    #[test]
    fn reads_bits_across_bytes() {
        let mut reader = pushed(&[0b1010_1100, 0b0101_0011]);
        assert_eq!(reader.read(3), 0b101);
        assert_eq!(reader.read(7), 0b011_0001);
        assert_eq!(reader.read(6), 0b01_0011);
        assert_eq!(reader.read(1), 0);
        assert!(!reader.has_ended());
        reader.signal_end();
        assert_eq!(reader.read(1), 0);
        assert!(reader.has_ended());
    }

    #[test]
    fn finds_start_codes() {
        let mut reader = pushed(&[0xff, 0x00, 0x00, 0x01, 0xb3, 0x00, 0x00, 0x01, 0x00, 0xaa]);
        assert!(reader.has_start_code(0xb3));
        assert_eq!(reader.bit_index, 0);
        assert_eq!(reader.next_start_code(), Some(0xb3));
        assert_eq!(reader.next_start_code(), Some(0x00));
        // Scanning needs five bytes, but only one remains
        assert_eq!(reader.next_start_code(), None);
    }

    #[test]
    fn pulls_from_reader_source() {
        let data: Vec<u8> = (0..=255).cycle().take(200_000).collect();
        let mut reader = BitReader::new(ReadSource::new(io::Cursor::new(data)));
        assert_eq!(reader.read(8), 0);
        reader.skip(8 * 150_000);
        assert_eq!(reader.tell(), 150_001);
        assert_eq!(reader.read(8), (150_001 % 256) as u32);
        reader.seek(10);
        assert_eq!(reader.read(8), 10);
        assert_eq!(reader.length(), 200_000);
    }
}
