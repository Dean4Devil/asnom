use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use common::AsBER;
use universal::{AsUniversalBER, UniversalNr};

impl AsBER for u8 {
    fn encode_into(&self, out: &mut Vec<u8>) {
        encode_limited(*self as u64, out);
    }

    fn decode(bytes: &Vec<u8>) -> Option<Self> {
        assert!(bytes.len() > 0);
        Some(bytes[0])
    }
}

impl AsUniversalBER for u8 {
    fn id(&self) -> UniversalNr {
        UniversalNr::INTEGER
    }
}

impl AsBER for u16 {
    fn encode_into(&self, out: &mut Vec<u8>) {
        encode_limited(*self as u64, out);
    }

    fn decode(bytes: &Vec<u8>) -> Option<Self> {
        bytes.as_slice().read_u16::<BigEndian>().ok()
    }
}

impl AsBER for u32 {
    fn encode_into(&self, out: &mut Vec<u8>) {
        encode_limited(*self as u64, out);
    }

    fn decode(bytes: &Vec<u8>) -> Option<Self> {
        bytes.as_slice().read_u32::<BigEndian>().ok()
    }
}

impl AsBER for u64 {
    fn encode_into(&self, out: &mut Vec<u8>) {
        encode_limited(*self, out);
    }

    fn decode(bytes: &Vec<u8>) -> Option<Self> {
        bytes.as_slice().read_u64::<BigEndian>().ok()
    }
}

impl AsBER for i64 {
    fn encode_into(&self, out: &mut Vec<u8>) {
        encode_signed_limited(*self, out);
    }

    fn decode(bytes: &Vec<u8>) -> Option<Self> {
        bytes.as_slice().read_i64::<BigEndian>().ok()
    }
}

fn encode_limited(input: u64, out: &mut Vec<u8>) {
    let mut count = 0u8;
    let mut rem: u64 = input;
    while {count += 1; rem >>= 8; rem > 0 }{}

    out.write_uint::<BigEndian>(input, count as usize).unwrap();
}

fn encode_signed_limited(input: i64, out: &mut Vec<u8>) {
    let mut count = 0u8;
    let mut rem: i64 = if input >= 0 { input } else { input * -1 };
    while {count += 1; rem >>= 8; rem > 0 }{}

    out.write_int::<BigEndian>(input, count as usize).unwrap();

}
