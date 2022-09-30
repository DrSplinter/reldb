use std::marker::PhantomData;

use crate::set::Set;

#[derive(Debug, Copy, PartialEq, Eq)]
pub struct Empty<T>(PhantomData<T>);

impl<T> Empty<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn as_ref(&self) -> Empty<&T> {
        Empty::new()
    }
}

impl<T> Clone for Empty<T> {
    fn clone(&self) -> Self {
        Empty::new()
    }
}

impl<T: Eq> Set for Empty<T> {
    type Item = T;

    fn contains(&self, _value: &T) -> bool {
        false
    }
}

impl<T> Iterator for Empty<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
