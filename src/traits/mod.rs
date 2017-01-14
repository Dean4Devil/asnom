use common::{Tag, ClassT};
use common::{TagPrimitive};

use byteorder::WriteBytesExt;
use byteorder::BigEndian;
use std::io::Write;

pub trait IntoBER {
    fn into_ber(self, ClassT, u64) -> Tag;
}

impl IntoBER for u32 {
    fn into_ber(self, class: ClassT, tagnr: u64) -> Tag {
        let mut count = 0u8;
        let mut rem = self;
        while {count += 1; rem >>= 8; rem > 0 }{}

        let mut out: Vec<u8> = Vec::with_capacity(count as usize);

        out.write_uint::<BigEndian>(self as u64, count as usize).unwrap();

        Tag::Primitive(TagPrimitive {
            class: class,
            tag_number: tagnr,
            inner: out,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::ClassT;
    use common::{Tag, TagPrimitive};

    #[test]
    fn test_u32() {
        let var = 35148u32;

        let tag = var.into_ber(ClassT::Universal, 2);

        println!("{:?}", tag);
        assert_eq!(tag, Tag::Primitive(TagPrimitive {
            class: ClassT::Universal,
            tag_number: 2,
            inner: vec![0b10001001, 0b01001100]
        }));
    }
}
