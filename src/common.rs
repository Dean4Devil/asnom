#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Primitive(TagPrimitive),
    Constructed(TagConstructed),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TagPrimitive {
    pub class: ClassT,
    pub tag_number: u64,
    pub inner: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TagConstructed {
    pub class: ClassT,
    pub tag_number: u64,
    pub inner: Vec<Tag>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeHeader {
    pub class: ClassT,
    pub pc: PCT,
    pub tagnr: u64,
}

impl TypeHeader {
    pub fn from_tag(tag: &Tag) -> TypeHeader{
        match tag {
            &Tag::Primitive(ref i) => TypeHeader {
                class: i.class,
                pc: PCT::Primitive,
                tagnr: i.tag_number,
            },
            &Tag::Constructed(ref i) => TypeHeader {
                class: i.class,
                pc: PCT::Constructed,
                tagnr: i.tag_number,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClassT {
    Universal,
    Application,
    ContextSpecific,
    Private,
}

impl ClassT {
    pub fn from_u8(n: u8) -> Option<ClassT> {
        match n {
            0 => Some(ClassT::Universal),
            1 => Some(ClassT::Application),
            2 => Some(ClassT::ContextSpecific),
            3 => Some(ClassT::Private),
            _ => None,
        }
    }
    pub fn to_u8(c: ClassT) -> u8 {
        match c {
            ClassT::Universal => 0,
            ClassT::Application => 1,
            ClassT::ContextSpecific => 2,
            ClassT::Private => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PCT {
    Primitive,
    Constructed,
}

impl PCT {
    pub fn from_u8(n: u8) -> Option<PCT> {
        match n {
            0 => Some(PCT::Primitive),
            1 => Some(PCT::Constructed),
            _ => None,
        }
    }
    pub fn to_u8(s: PCT) -> u8 {
        match s {
            PCT::Primitive => 0,
            PCT::Constructed => 1,
        }
    }
}
