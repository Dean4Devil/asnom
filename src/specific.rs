use traits::{BERPayload, BERTag};
use common::{TagStructure, TagClass};

use parse::{parse_type_header, parse_length};

use write::{write_type, write_length};

pub struct SpecificTag<T> {
    class: TagClass,
    id: u64,
    structure: TagStructure,
    inner: T,
}

impl<T: BERPayload> SpecificTag<T> {
    pub fn wrap(class: TagClass,
                id: u64,
                structure: TagStructure,
                inner: T) -> Self {
        SpecificTag {
            class: class,
            id: id,
            structure: structure,
            inner: inner
        }
    }
}

impl<T: BERPayload> BERTag for SpecificTag<T> {
    fn decode(bytes: &[u8]) -> Option<Self> {
        None
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        write_type(out, self.class, self.structure, self.id);
        write_length(out, self.inner.len());
        self.inner.encode_into(out);
    }
}

impl<T: BERPayload> BERPayload for Vec<SpecificTag<T>> {
    fn decode(bytes: &[u8]) -> Option<Self> {
        let out: Vec<SpecificTag<T>> = Vec::new();
        None
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        for tag in self {
            tag.encode_into(out);
        }
    }

    fn len(&self) -> u64 {
        let mut len = 0u64;

        for tag in self {
            len += tag.inner.len();
        }

        len
    }
}

impl BERPayload for Vec<u8> {
    fn decode(bytes: &[u8]) -> Option<Self> {
        Some(bytes.to_vec())
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        for byte in self {
            out.push(*byte);
        }
    }

    fn len(&self) -> u64 {
        self.len() as u64
    }
}
