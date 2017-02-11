use common::{TagClass, TagStructure};
use structure::{StructureTag, PL};

use std::io;
use std::io::Write;

use byteorder::BigEndian;
use byteorder::WriteBytesExt;

pub fn encode_into(buf: &mut Vec<u8>, tag: StructureTag) -> io::Result<()> {
    let structure = match tag.payload {
        PL::P(_) => TagStructure::Primitive,
        PL::C(_) => TagStructure::Constructed,
    };

    write_type(buf, tag.class, structure, tag.id);
    match tag.payload {
        PL::P(v) => {
            write_length(buf, v.len());
            for byte in v {
                buf.push(byte);
            }
        },
        PL::C(tags) => {
            let mut tmp: Vec<u8> = Vec::new();
            for tag in tags {
                try!(encode_into(&mut tmp, tag));
            }
            write_length(buf, tmp.len());
            for byte in tmp {
                buf.push(byte);
            }
        }
    };

    Ok(())
}

pub fn write_type(mut w: &mut Write, class: TagClass, structure: TagStructure, id: u64) {
    let extended_tag: Option<Vec<u8>>;

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
                let byte = (tag & 0x7F) as u8;

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

    let _ = w.write_u8(type_byte);

    if let Some(mut ext_bytes) = extended_tag
    {
        for _ in 0..ext_bytes.len()-1
        {
            let mut byte = ext_bytes.pop().unwrap();

            // Set the first bit
            byte |= 0x80;

            let _ = w.write_u8(byte);
        }

        let byte = ext_bytes.pop().unwrap();
        let _ = w.write_u8(byte);
    }
}

// Yes I know you could overflow the length in theory. But, do you have 2^64 bytes of memory?
pub fn write_length(mut w: &mut Write, length: usize) {
    // Short form
    if length < 128
    {
        let _ = w.write_u8(length as u8);
    }
    // Long form
    else
    {
        let mut count = 0u8;
        let mut len = length;
        while {count += 1; len >>= 8; len > 0 }{}


        let _ = w.write_u8(count | 0x80);
        let _ = w.write_uint::<BigEndian>(length as u64, count as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::string;

    use std::default::Default;

    use byteorder::{BigEndian, WriteBytesExt};

    use structures::*;
    use common::TagClass::*;

    #[test]
    fn encode_simple_tag() {
        let tag = Tag::Integer(Integer {
            inner: 1616, 
            .. Default::default()
        });

        let mut buf = Vec::<u8>::new();
        super::encode_into(&mut buf, tag.into_structure());

        assert_eq!(buf, vec![0x2, 0x2, 0x06, 0x50]);
    }

    #[test]
    fn encode_constructed_tag()
    {
        let tag = Tag::Sequence(Sequence {
            inner: vec![
                Tag::OctetString(OctetString {
                    inner: String::from("Hello World!").into_bytes(),
                    .. Default::default()
                })
            ],
            .. Default::default()
        });

        let mut buf = Vec::<u8>::new();
        super::encode_into(&mut buf, tag.into_structure());

        assert_eq!(buf, vec![48,14,4,12,72,101,108,108,111,32,87,111,114,108,100,33]);
    }

    #[test]
    fn complex_tag()
    {
        let tag = Tag::Sequence(Sequence {
            inner: vec![
                Tag::Integer(Integer {
                    inner: 1,
                    .. Default::default()
                }),
                Tag::Sequence(Sequence {
                    id: 0,
                    class: Application,
                    inner: vec![
                           Tag::Integer(Integer {
                               inner: 3,
                                .. Default::default()
                           }),
                           Tag::OctetString(OctetString {
                               inner: String::from("cn=root,dc=plabs").into_bytes(),
                                .. Default::default()
                           }),
                           Tag::OctetString(OctetString {
                               id: 0,
                               class: Context,
                               inner: String::from("asdf").into_bytes(),
                           })
                    ]
                })
            ],
            .. Default::default()
        });

        let expected = vec![
            0x30, 0x20,
                0x02, 0x01, 0x01,
                0x60, 0x1B,
                    0x02, 0x01, 0x03,
                    0x04, 0x10, 0x63, 0x6e, 0x3d, 0x72, 0x6f, 0x6f, 0x74, 0x2c, 0x64, 0x63, 0x3d, 0x70, 0x6c, 0x61, 0x62, 0x73,
                    0x80, 0x04, 0x61, 0x73, 0x64, 0x66
        ];

        let mut buf = Vec::<u8>::new();
        super::encode_into(&mut buf, tag.into_structure());

        assert_eq!(buf, expected);
    }
}
