use common::Tag;
use common::TagPrimitive;
use common::TagConstructed;
use common::TypeHeader;
use common::ClassT;
use common::PCT;

use nom;
use nom::InputLength;

named!(class_bits<(&[u8], usize), ClassT>,
    map_opt!(
        take_bits!(u8, 2),
        ClassT::from_u8
    )
);

named!(pc_bit<(&[u8], usize), PCT>,
    map_opt!(
        take_bits!(u8, 1),
        PCT::from_u8
    )
);

named!(tagnr_bits<(&[u8], usize), u64>,
    take_bits!(u64, 5)
);

named!(parse_type_header<TypeHeader>, bits!(
    do_parse!(
        class: class_bits >>
        pc: pc_bit >>
        tagnr: tagnr_bits >>
        ( TypeHeader { class: class, pc: pc, tagnr: tagnr } )
   )
));

named!(parse_length<u64>,
    alt!(
        bits!(
            do_parse!(
                // Short length form
                tag_bits!(u8, 1, 0u8) >>
                len: take_bits!(u64, 7) >>
                (len)
            )
        )
    |
        length_value!(
            bits!(
                do_parse!(
                    /* // TODO: Fix nom to be able to do this.
                     *return_error!(nom::ErrorKind::Custom(1),
                     *    not!(tag_bits!(u8, 8, 255u8))
                     *) >>
                     */
                    // Long length form
                    tag_bits!(u8, 1, 1u8) >>
                    len: take_bits!(u8, 7) >>
                    (len)
                )
            ),
            parse_uint
        )
    )
);



pub fn parse_uint(i: &[u8]) -> nom::IResult<&[u8], u64> {
    match i.len() {
        1 => nom::be_u8(i).map(|x| x as u64),
        2 => nom::be_u16(i).map(|x| x as u64),
        3 => nom::be_u32(i).map(|x| x as u64),
        8 => nom::be_u64(i),
        _ => unimplemented!()
    }
}

pub fn parse_tag(i: &[u8]) -> nom::IResult<&[u8], Tag> {
    let (i, (hdr,len)) = try_parse!(i, do_parse!(
        hdr: parse_type_header >>
        len: parse_length >>
        ((hdr, len))
    ));

    match hdr.pc {
        PCT::Primitive => {
            let (i, content) = try_parse!(i, length_bytes!(value!(len)));
            let t = TagPrimitive {
                class: hdr.class,
                tag_number: hdr.tagnr,
                inner: content.to_vec(),
            };
            nom::IResult::Done(i, Tag::Primitive(t))
        }
        PCT::Constructed => {
            let mut content: &[u8];
            let pres = try_parse!(i, length_bytes!(value!(len)));

            let i = pres.0;
            content = pres.1;

            let mut tv: Vec<Tag> = Vec::new();
            while content.input_len() > 0 {
                let pres = try_parse!(content, call!(parse_tag));
                content = pres.0;
                let res: Tag = pres.1;
                tv.push(res);
            }
            let t = TagConstructed {
                class: hdr.class,
                tag_number: hdr.tagnr,
                inner: tv,
            };
            nom::IResult::Done(i, Tag::Constructed(t))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;
    use common::{TypeHeader, ClassT, PCT, Tag, TagPrimitive, TagConstructed};

    #[test]
    fn test_primitive() {
        let bytes: Vec<u8> = vec![2, 2, 255, 127];
        let result_tag: Tag = Tag::Primitive(TagPrimitive {
            class: ClassT::Universal,
            tag_number: 2u64,
            inner: vec![255, 127],
        });
        let rest_tag: Vec<u8> = vec![];

        let tag = parse_tag(&bytes[..]);

        assert_eq!(tag,
                   IResult::Done(&rest_tag[..], result_tag));
    }

    #[test]
    fn test_constructed() {
        let bytes: Vec<u8> = vec![48,14,12,12,72,101,108,108,111,32,87,111,114,108,100,33];
        let result_tag: Tag = Tag::Constructed(TagConstructed {
            class: ClassT::Universal,
            tag_number: 16u64,
            inner: vec![Tag::Primitive(TagPrimitive {
                class: ClassT::Universal,
                tag_number: 12,
                inner: vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]
            })],
        });
        let rest_tag: Vec<u8> = vec![];

        let tag = parse_tag(&bytes[..]);

        assert_eq!(tag,
                   IResult::Done(&rest_tag[..], result_tag));
    }

    // FIXME
    //#[test]
    fn invalid_length() {
        let bytes: Vec<u8> = vec![48, 255];
        let result_type = TypeHeader {
            class: ClassT::Universal,
            pc: PCT::Constructed,
            tagnr: 16u64,
        };
        let rest_type: Vec<u8> = vec![255];
        assert_eq!(super::parse_type_header(&bytes[..]),
                   IResult::Done(&rest_type[..], result_type));

        let result_len= 255u64;
        let rest_len: Vec<u8> = vec![];
        assert_eq!(super::parse_length(&rest_type[..]),
                   IResult::Done(&rest_len[..], result_len));

    }

    #[test]
    fn test_long_length() {
        let bytes: Vec<u8> = vec![
            0x30, 0x82, 0x01, 0x01,
            0x80, 0x0C, 0x4A, 0x75,
            0x73, 0x74, 0x41, 0x4C,
            0x6F, 0x6E, 0x67, 0x54,
            0x61, 0x67, 0x81, 0x81,
            0xF0, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67, 0x4A, 0x75, 0x73,
            0x74, 0x41, 0x4C, 0x6F,
            0x6E, 0x67, 0x54, 0x61,
            0x67,
        ];
        let result_type = TypeHeader {
            class: ClassT::Universal,
            pc: PCT::Constructed,
            tagnr: 16u64,
        };
        let rest_type: Vec<u8> = bytes[1..].to_vec();
        assert_eq!(super::parse_type_header(&bytes[..]),
                   IResult::Done(&rest_type[..], result_type));

        let result_len= 257u64;
        let rest_len: Vec<u8> = rest_type[3..].to_vec();
        assert_eq!(super::parse_length(&rest_type[..]),
                   IResult::Done(&rest_len[..], result_len));
    }
}
