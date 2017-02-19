use common::TagClass;
use super::ASNTag;
use universal;
use structure;

use std::default;

use byteorder::{BigEndian, WriteBytesExt};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Integer {
    pub id: u64,
    pub class: TagClass,
    pub inner: i64,
}

impl ASNTag for Integer {
    fn into_structure(self) -> structure::StructureTag {
        let mut count = 0u8;
        let mut rem: i64 = if self.inner >= 0 { self.inner } else { self.inner * -1 };
        while {count += 1; rem >>= 8; rem > 0 }{}

        let mut out: Vec<u8> = Vec::with_capacity(count as usize);

        out.write_int::<BigEndian>(self.inner, count as usize).unwrap();

        structure::StructureTag {
            id: self.id,
            class: self.class,
            payload: structure::PL::P(out),
        }
    }
}

impl default::Default for Integer {
    fn default() -> Integer {
        Integer {
            id: universal::Types::Integer as u64,
            class: TagClass::Universal,
            inner: 0i64,
        }
    }
}
