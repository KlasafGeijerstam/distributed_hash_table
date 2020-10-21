pub use pdu_io_derive::*;

pub trait ToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_bytes(input: &mut &[u8]) -> Self;
}

pub trait ParsePdu: Sized {
    fn try_parse(buffer: &[u8]) -> Option<(Self, usize)>;
}

pub trait SizeOf {
    fn size_of() -> usize;
}

impl ToBytes for u8 {
    fn to_bytes(self) -> Vec<u8> {
        vec![self.to_be_bytes()[0]]
    }
}

impl FromBytes for u8 {
    fn from_bytes(input: &mut &[u8]) -> Self {
        util::read_be_u8(input)
    }
}

impl ToBytes for u16 {
    fn to_bytes(self) -> Vec<u8> {
        let mut o = Vec::new();
        o.extend_from_slice(&self.to_be_bytes());
        o
    }
}

impl FromBytes for u16 {
    fn from_bytes(input: &mut &[u8]) -> Self {
        util::read_be_u16(input)
    }
}

impl ToBytes for u32 {
    fn to_bytes(self) -> Vec<u8> {
        let mut o = Vec::new();
        o.extend_from_slice(&self.to_be_bytes());
        o
    }
}

impl FromBytes for u32 {
    fn from_bytes(input: &mut &[u8]) -> Self {
        util::read_be_u32(input)
    }
}

impl SizeOf for u8 {
    fn size_of() -> usize {
        std::mem::size_of::<u8>()
    }
}

impl SizeOf for u16 {
    fn size_of() -> usize {
        std::mem::size_of::<u16>()
    }
}

impl SizeOf for u32 {
    fn size_of() -> usize {
        std::mem::size_of::<u32>()
    }
}

pub mod util {
    use std::convert::TryInto;

    pub fn read_be_u8(input: &mut &[u8]) -> u8 {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<u8>());
        *input = rest;
        u8::from_be_bytes(int_bytes.try_into().unwrap())
    }

    pub fn read_be_u32(input: &mut &[u8]) -> u32 {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
        *input = rest;
        u32::from_be_bytes(int_bytes.try_into().unwrap())
    }

    pub fn read_be_u16(input: &mut &[u8]) -> u16 {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
        *input = rest;
        u16::from_be_bytes(int_bytes.try_into().unwrap())
    }
}
