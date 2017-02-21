use std::default;
use universal;
use structure;

use super::{ASNTag, Tag};
use common::TagClass;

#[derive(Clone, Debug, PartialEq)]
pub struct Sequence {
    pub id: u64,
    pub class: TagClass,
    pub inner: Vec<Tag>,
}

impl ASNTag for Sequence {
    fn into_structure(self) -> structure::StructureTag {
        structure::StructureTag {
            id: self.id,
            class: self.class,
            payload: structure::PL::C(self.inner.into_iter().map(|x| x.into_structure()).collect()),
        }
    }
}

impl default::Default for Sequence {
    fn default() -> Self {
        Sequence {
            id: universal::Types::Sequence as u64,
            class: TagClass::Universal,
            inner: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceOf<T> {
    pub id: u64,
    pub class: TagClass,
    pub inner: Vec<T>,
}

impl<T: ASNTag + Sized> SequenceOf<T> {
    fn into_structure(self) -> structure::StructureTag {
        structure::StructureTag {
            id: self.id,
            class: self.class,
            payload: structure::PL::C(self.inner.into_iter().map(|x| x.into_structure()).collect()),
        }
    }
}

impl<T: ASNTag + Sized> default::Default for SequenceOf<T> {
    fn default() -> Self {
        SequenceOf::<T> {
            id: universal::Types::Sequence as u64,
            class: TagClass::Universal,
            inner: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetOf<T> {
    pub id: u64,
    pub class: TagClass,
    pub inner: Vec<T>,
}

impl<T: ASNTag + Sized> ASNTag for SetOf<T> {
    fn into_structure(self) -> structure::StructureTag {
        structure::StructureTag {
            id: self.id,
            class: self.class,
            payload: structure::PL::C(self.inner.into_iter().map(|x| x.into_structure()).collect()),
        }
    }
}


impl<T: ASNTag + Sized> default::Default for SetOf<T> {
    fn default() -> Self {
        SetOf {
            id: universal::Types::Set as u64,
            class: TagClass::Universal,
            inner: Vec::new(),
        }
    }
}
