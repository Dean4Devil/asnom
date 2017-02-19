use structure;

use super::{ASNTag, Tag};
use common::TagClass;

// Explitic tags don't implement Default because that just wouldn't make sense.
#[derive(Clone, Debug, PartialEq)]
pub struct ExplicitTag {
    pub id: u64,
    pub class: TagClass,
    pub inner: Box<Tag>,
}

impl ASNTag for ExplicitTag {
    fn into_structure(self) -> structure::StructureTag {
        structure::StructureTag {
            id: self.id,
            class: self.class,
            payload: structure::PL::C(vec![self.inner.into_structure()]),
        }
    }
}
