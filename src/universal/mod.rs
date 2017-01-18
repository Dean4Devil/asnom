use std::ops::{Deref, DerefMut};

use traits::{BERPayload, BERTag};

//mod number;

pub trait UniversalPayload: BERPayload {
    fn id(&self) -> UniversalNr;
}

pub struct UniversalTag<T> {
    inner: T
}

impl<T: UniversalPayload> UniversalTag<T> {
    pub fn wrap(inner: T) -> Self {
        UniversalTag { inner: inner }
    }
}

impl<T: UniversalPayload> BERTag for UniversalTag<T> {
    fn decode(bytes: &[u8]) -> Option<Self> {
        T::decode(bytes).map(|x| UniversalTag { inner: x })
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        out.push(0xFF);
        out.push(0xFF);
        self.inner.encode_into(out);
    }
}

impl<T> Deref for UniversalTag<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for UniversalTag<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub enum UniversalNr {
    INTEGER
}
