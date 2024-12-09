#![deny(clippy::all)]

mod buffer;

pub use buffer::ByteBufferAdv;

/// An enum to represent the byte order of the ByteBufferAdv object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endian {
    BigEndian,
    LittleEndian,
}