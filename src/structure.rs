use common::TagClass;

#[derive(Clone, PartialEq, Debug)]
pub struct StructureTag {
    pub class: TagClass,
    pub id: u64,
    pub payload: PL
}

#[derive(Clone, PartialEq, Debug)]
pub enum PL {
    P(Vec<u8>),
    C(Vec<StructureTag>),
}

struct GenericConstructed {
    inner: Vec<StructureTag>
}

struct GenericPrimitive {
    inner: Vec<u8>
}


