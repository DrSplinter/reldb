use std::{hash::Hash, marker::PhantomData};

use crate::set::Set;

#[derive(Debug, Clone)]
pub struct Difference<A, B> {
    a: A,
    b: B,
}

impl<A, B> Difference<A, B> {
    pub fn new(a: A, b: B) -> Difference<A, B> {
        Difference { a, b }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Set

impl<A, B, T: Eq> Set for Difference<A, B>
where
    A: Set<Item = T>,
    B: Set<Item = T>,
{
    type Item = T;

    fn contains(&self, value: &Self::Item) -> bool {
        !self.b.contains(value) && self.a.contains(value)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Iterator

impl<A, B, T: Eq + Hash> IntoIterator for Difference<A, B>
where
    A: IntoIterator<Item = T>,
    B: IntoIterator<Item = T>,
{
    type Item = T;

    type IntoIter = DifferenceIter<A::IntoIter, B::IntoIter, T>;

    fn into_iter(self) -> Self::IntoIter {
        DifferenceIter::new(self.a.into_iter(), self.b.into_iter())
    }
}

pub struct DifferenceIter<A, B, T> {
    a: A,
    cache: std::collections::HashSet<T>,
    data: PhantomData<B>,
}

impl<A, B, T: Eq + Hash> DifferenceIter<A, B, T>
where
    A: IntoIterator<Item = T>,
    B: IntoIterator<Item = T>,
{
    fn new(a: A, b: B) -> Self {
        Self {
            a,
            cache: b.into_iter().collect(),
            data: PhantomData,
        }
    }
}

impl<A, B, T: Eq + Hash> Iterator for DifferenceIter<A, B, T>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(a) = self.a.next() {
            if !self.cache.contains(&a) {
                return Some(a);
            }
        }

        None
    }
}
