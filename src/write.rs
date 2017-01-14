use common::Tag;
use common::{TypeHeader, ClassT, PCT};
use common::{TagConstructed, TagPrimitive};

use std::io::Write;

use byteorder::BigEndian;
use byteorder::ByteOrder;
use byteorder::WriteBytesExt;

pub fn encode(tag: &Tag) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    encode_into(&mut out, tag);
    return out;
}

pub fn encode_into(mut w: &mut Write, input: &Tag) {
    match input {
        &Tag::Primitive(ref tag) => {
            write_type(w, TypeHeader::from_tag(&input));
            write_length(w, tag.inner.len() as u64);
            w.write_all(&tag.inner);
        },
        &Tag::Constructed(ref tag) => {
            let mut out: Vec<u8> = Vec::new();
            for subtag in &tag.inner {
                encode_into(&mut out, &subtag);
            }
            write_type(w, TypeHeader::from_tag(&input));
            write_length(w, out.len() as u64);
            w.write_all(&out);
        },
    }
}


fn write_type(mut w: &mut Write, tagtype: TypeHeader) {
    let mut extended_tag: Option<Vec<u8>> = None;

    let type_byte = {
        // First two bits: Class
        (ClassT::to_u8(tagtype.class)) << 6 |
        // Bit 6: Primitive/Constructed
        (PCT::to_u8(tagtype.pc)) << 5 |
        // Bit 5-1: Tag Number
        if tagtype.tagnr > 30
        {
            let mut tagbytes: Vec<u8> = Vec::new();

            let mut tag = tagtype.tagnr;
            while tag > 0
            {
                let mut byte = (tag & 0x7F) as u8;

                tag >>= 7;

                tagbytes.push(byte);
            }

            extended_tag = Some(tagbytes);

            // This means we need to set the 5 tag bits to 11111, so 31 or 0x1F
            31
        }
        else
        {
            extended_tag = None;
            tagtype.tagnr as u8
        }
    }; // let type_byte

    w.write_u8(type_byte);

    let mut written = 1;

    if let Some(mut ext_bytes) = extended_tag
    {
        for _ in 0..ext_bytes.len()-1
        {
            let mut byte = ext_bytes.pop().unwrap();

            // Set the first bit
            byte |= 0x80;

            w.write_u8(byte);
        }

        let byte = ext_bytes.pop().unwrap();
        w.write_u8(byte);
    }
}

// Yes I know you could overflow the length in theory. But, do you have 2^64 bytes of memory?
fn write_length(mut w: &mut Write, mut length: u64) {
    // Short form
    if length < 128
    {
        w.write_u8(length as u8);
    }
    // Long form
    else
    {
        let mut count = 0u8;
        let mut len = length;
        while {count += 1; len >>= 8; len > 0 }{}


        w.write_u8(count | 0x80);
        w.write_uint::<BigEndian>(length, count as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use byteorder::{BigEndian, WriteBytesExt};

    use common::{Tag, TagPrimitive, TagConstructed};
    use common::{TypeHeader, ClassT};

    #[test]
    fn encode_simple_tag() {
        let mut payload: Vec<u8> = Vec::new();
        payload.write_i16::<BigEndian>(1616);

        let tag = Tag::Primitive(TagPrimitive {
            class: ClassT::Universal,
            tag_number: 2u64,
            inner: payload,
        });

        let mut buf = Vec::<u8>::new();
        super::encode_into(&mut buf, &tag);

        assert_eq!(buf, vec![0x2, 0x2, 0x06, 0x50]);
    }

    #[test]
    fn encode_constructed_tag()
    {
        let childtag = Tag::Primitive(TagPrimitive {
            class: ClassT::Universal,
            tag_number: 12u64,
            inner: "Hello World!".to_string().into_bytes(),
        });

        let tag = Tag::Constructed(TagConstructed {
            class: ClassT::Universal,
            tag_number: 16u64,
            inner: vec![childtag],
        });

        let mut buf = Vec::<u8>::new();
        super::encode_into(&mut buf, &tag);

        assert_eq!(buf, vec![48,14,12,12,72,101,108,108,111,32,87,111,114,108,100,33]);
    }
}
