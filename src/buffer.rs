use crate::Endian;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::{
    fmt::Debug,
    io::{Error, ErrorKind, Read, Result, Write},
};

// Version 0.1.1: File structure updated, remaking the ByteBuffer crate to be more usable.

pub struct ByteBufferAdv {
    data: Vec<u8>,
    wpos: usize,
    rpos: usize,
    wbit: usize,
    rbit: usize,
    endian: Endian,
}

impl From<&[u8]> for ByteBufferAdv {
    fn from(value: &[u8]) -> Self {
        ByteBufferAdv::from_bytes(value)
    }
}

impl From<Vec<u8>> for ByteBufferAdv {
    fn from(value: Vec<u8>) -> Self {
        ByteBufferAdv::from_vec(value)
    }
}

impl From<ByteBufferAdv> for Vec<u8> {
    fn from(value: ByteBufferAdv) -> Self {
        value.into_vec()
    }
}

impl Default for ByteBufferAdv {
    fn default() -> Self {
        Self::new()
    }
}

impl Read for ByteBufferAdv {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.flush_bits();
        let read_len = std::cmp::min(self.data.len() - self.rpos, buf.len());
        let range = self.rpos..self.rpos + read_len;
        for (i, val) in self.data[range].iter().enumerate() {
            buf[i] = *val;
        }
        self.rpos += read_len;
        Ok(read_len)
    }
}

impl Write for ByteBufferAdv {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_bytes(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Debug for ByteBufferAdv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rpos = if self.rbit > 0 {
            self.rpos + 1
        } else {
            self.rpos
        };

        let read_len = self.data.len() - rpos;
        let mut remaining_data = vec![0; read_len];
        let range = rpos..rpos + read_len;
        for (i, val) in self.data[range].iter().enumerate() {
            remaining_data[i] = *val;
        }

        write!(
            f,
            "ByteBuffer {{ remaining_data: {:?}, total_data: {:?}, wpos: {:?}, rpos: {:?}, endian: {:?} }}",
            remaining_data, self.data, self.wpos, self.rpos, self.endian
        )
    }
}

impl ByteBufferAdv {
    /// Construct a new, empty ByteBufferAdv
    pub fn new() -> ByteBufferAdv {
        ByteBufferAdv {
            data: vec![],
            wpos: 0,
            rpos: 0,
            wbit: 0,
            rbit: 0,
            endian: Endian::BigEndian,
        }
    }

    /// Construct a new ByteBufferAdv filled with the data array
    pub fn from_bytes(bytes: &[u8]) -> ByteBufferAdv {
        let mut buf = ByteBufferAdv::new();
        buf.write_bytes(bytes);
        buf
    }

    /// Constructs a new ByteBufferAdv from an existing vector. This
    /// function takes ownership of the vector
    pub fn from_vec(vec: Vec<u8>) -> ByteBufferAdv {
        let len = vec.len();
        ByteBufferAdv {
            data: vec,
            wpos: len,
            rpos: 0,
            wbit: 0,
            rbit: 0,
            endian: Endian::BigEndian,
        }
    }

    /// Return raw byte buffer as Vec<u8>
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    /// Return the buffer size
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear the buffer and reinitialize the reading and writing cursors
    pub fn clear(&mut self) {
        self.data.clear();
        self.reset_cursors();
        self.reset_bits_cursors();
    }

    /// Reinitialize the read and writing cursor
    pub fn reset_cursors(&mut self) {
        self.wpos = 0;
        self.rpos = 0;
    }

    /// Reinitialize the bit reading and bit writing cursor
    pub fn reset_bits_cursors(&mut self) {
        self.wbit = 0;
        self.rbit = 0;
    }

    /// Change the buffer size to size
    ///
    /// _Note_: You cannot shrink a buffer with this method
    pub fn resize(&mut self, size: usize) {
        let diff = size - self.data.len();
        if diff > 0 {
            self.data.extend(std::iter::repeat(0).take(diff));
        }
    }

    /// Set the byte order of the buffer
    ///
    /// _Note_: By default, the buffer uses big endian order
    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    /// Returns the current byte order of the buffer
    pub fn endian(&self) -> Endian {
        self.endian
    }
}

impl ByteBufferAdv {
    // Writing Operations

    // Integer

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.flush_bits();

        let size = bytes.len() + self.wpos;

        if size > self.data.len() {
            self.resize(size);
        };

        for b in bytes {
            self.data[self.wpos] = *b;
            self.wpos += 1;
        }
    }

    // Unsigned

    pub fn write_u8(&mut self, val: u8) {
        self.write_bytes(&[val]);
    }

    pub fn write_u16(&mut self, val: u16) {
        let mut buf = [0; 2];

        match self.endian {
            Endian::BigEndian => BigEndian::write_u16(&mut buf, val),
            Endian::LittleEndian => LittleEndian::write_u16(&mut buf, val),
        };

        self.write_bytes(&buf);
    }

    pub fn write_u32(&mut self, val: u32) {
        let mut buf = [0; 4];

        match self.endian {
            Endian::BigEndian => BigEndian::write_u32(&mut buf, val),
            Endian::LittleEndian => LittleEndian::write_u32(&mut buf, val),
        };

        self.write_bytes(&buf);
    }

    pub fn write_u64(&mut self, val: u64) {
        let mut buf = [0; 8];

        match self.endian {
            Endian::BigEndian => BigEndian::write_u64(&mut buf, val),
            Endian::LittleEndian => LittleEndian::write_u64(&mut buf, val),
        };

        self.write_bytes(&buf);
    }

    pub fn write_u128(&mut self, val: u128) {
        let mut buf = [0; 16];

        match self.endian {
            Endian::BigEndian => BigEndian::write_u128(&mut buf, val),
            Endian::LittleEndian => LittleEndian::write_u128(&mut buf, val),
        };

        self.write_bytes(&buf);
    }

    // Signed

    pub fn write_i8(&mut self, val: i8) {
        self.write_u8(val as u8);
    }

    pub fn write_i16(&mut self, val: i16) {
        self.write_u16(val as u16);
    }

    pub fn write_i32(&mut self, val: i32) {
        self.write_u32(val as u32);
    }

    pub fn write_i64(&mut self, val: i64) {
        self.write_u64(val as u64);
    }

    pub fn write_i128(&mut self, val: i128) {
        self.write_u128(val as u128);
    }

    // Float

    pub fn write_f32(&mut self, val: f32) {
        let mut buf = [0; 4];

        match self.endian {
            Endian::BigEndian => BigEndian::write_f32(&mut buf, val),
            Endian::LittleEndian => LittleEndian::write_f32(&mut buf, val),
        };

        self.write_bytes(&buf);
    }

    pub fn write_f64(&mut self, val: f64) {
        let mut buf = [0; 8];

        match self.endian {
            Endian::BigEndian => BigEndian::write_f64(&mut buf, val),
            Endian::LittleEndian => LittleEndian::write_f64(&mut buf, val),
        };

        self.write_bytes(&buf);
    }

    // Extra

    pub fn write_string(&mut self, val: &str) {
        self.write_u32(val.len() as u32);
        self.write_bytes(val.as_bytes());
    }
}

impl ByteBufferAdv {
    // Reading Operations

    pub fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        self.flush_bits();
        if self.rpos + size > self.data.len() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Could not read enough bytes from buffer"
            ))
        }
        let range = self.rpos..self.rpos+size;
        let mut res = Vec::<u8>::new();
        res.write_all(&self.data[range])?;
        self.rpos += size;
        Ok(res)
    }   
}

impl ByteBufferAdv {
    // Flushing Operations
    pub fn flush_bits(&mut self) {
        if self.rbit > 0 {
            self.flush_rbits();
        }
        if self.wbit > 0 {
            self.flush_wbits();
        }
    }

    fn flush_rbits(&mut self) {
        self.rpos += 1;
        self.rbit = 0
    }

    fn flush_wbits(&mut self) {
        self.wpos += 1;
        self.wbit = 0
    }
}
