use std::default;
use universal;
use structure;

use super::ASNTag;
use common::TagClass;

#[derive(Clone, Debug)]
pub struct Null {
    id: u64,
    class: TagClass,
    inner: (),
}

impl ASNTag for Null {
    fn into_structure(self) -> structure::StructureTag {
        structure::StructureTag {
            id: self.id,
            class: self.class,
            payload: structure::PL::P(Vec::new()),
        }
    }
}

impl default::Default for Null {
    fn default() -> Self {
        Null {
            id: universal::Types::Null as u64,
            class: TagClass::Universal,
            inner: (),
        }
    }
}
