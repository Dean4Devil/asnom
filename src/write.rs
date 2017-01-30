use common::{TagClass, TagStructure};
use structure::{StructureTag, PL};

use std::io;
use std::io::Write;

use byteorder::BigEndian;
use byteorder::ByteOrder;
use byteorder::WriteBytesExt;

pub fn encode_into(buf: &mut Vec<u8>, tag: StructureTag) -> io::Result<()> {
    let structure = match tag.payload {
        PL::P(_) => TagStructure::Primitive,
        PL::C(_) => TagStructure::Constructed,
    };

    write_type(buf, tag.class, structure, tag.id);
    match tag.payload {
        PL::P(v) => {
            for byte in v {
                buf.push(byte);
            }
        },
        PL::C(tags) => {
            for tag in tags {
                try!(encode_into(buf, tag));
            }
        }
    };

    Ok(())
}

pub fn write_type(mut w: &mut Write, class: TagClass, structure: TagStructure, id: u64) {
    let mut extended_tag: Option<Vec<u8>> = None;

    let type_byte = {
        // First two bits: Class
        (class as u8) << 6 |
        // Bit 6: Primitive/Constructed
        (structure as u8) << 5 |
        // Bit 5-1: Tag Number
        if id > 30
        {
            let mut tagbytes: Vec<u8> = Vec::new();

            let mut tag = id;
            while tag > 0
            {
                // Only take the 7 lower bits.
                let mut byte = (tag & 0x7F) as u8;

                tag >>= 7;

                tagbytes.push(byte);
            }

            extended_tag = Some(tagbytes);

            // This means we need to set the 5 tag bits to 11111, so 31 or 0x1F
            0x1F
        }
        else
        {
            extended_tag = None;
            id as u8
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
pub fn write_length(mut w: &mut Write, mut length: u64) {
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

#[cfg(atest)]
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
