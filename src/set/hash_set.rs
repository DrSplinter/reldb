use std::hash::Hash;

use quickcheck::{Arbitrary, Gen};

use crate::set::{IntoSet, Set};

#[derive(Debug, Clone)]
pub struct HashSet<T> {
    base: std::collections::HashSet<T>,
}

impl<T: Eq + Hash> HashSet<T> {
    fn new(hash_set: std::collections::HashSet<T>) -> Self {
        Self { base: hash_set }
    }
}

#[derive(Debug, Clone)]
pub struct HashSetRef<'a, T> {
    base: &'a std::collections::HashSet<T>,
}

impl<'a, T: Eq + Hash> HashSetRef<'a, T> {
    fn new(hash_set: &'a std::collections::HashSet<T>) -> Self {
        Self { base: hash_set }
    }
}

impl<T> Set for HashSet<T>
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    fn contains(&self, value: &T) -> bool {
        self.base.contains(value)
    }
}

impl<'a, T> Set for HashSetRef<'a, T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a T;

    fn contains(&self, value: &Self::Item) -> bool {
        self.base.contains(value)
    }
}

impl<T, S> From<S> for HashSet<T>
where
    T: Eq + Hash + Clone,
    S: Into<std::collections::HashSet<T>>,
{
    fn from(val: S) -> Self {
        HashSet::new(val.into())
    }
}

impl<T> IntoSet for std::collections::HashSet<T>
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    type SetType = HashSet<T>;

    fn into_set(self) -> Self::SetType {
        HashSet::new(self)
    }
}

impl<'a, T> IntoSet for &'a std::collections::HashSet<T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a T;

    type SetType = HashSetRef<'a, T>;

    fn into_set(self) -> Self::SetType {
        HashSetRef::new(self)
    }
}

impl<T> IntoIterator for HashSet<T> {
    type Item = T;

    type IntoIter = std::collections::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a HashSet<T> {
    type Item = &'a T;

    type IntoIter = std::collections::hash_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.iter()
    }
}

impl<'a, T> IntoIterator for HashSetRef<'a, T> {
    type Item = &'a T;

    type IntoIter = std::collections::hash_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.iter()
    }
}

impl<T: Arbitrary + Eq + Hash> Arbitrary for HashSet<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        std::collections::HashSet::arbitrary(g).into_set()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.base.shrink().map(IntoSet::into_set))
    }
}
