pub mod difference;
pub mod empty;
pub mod hash_set;
pub mod ref_set;
pub mod union;

use std::collections::BTreeSet;

pub use empty::Empty;
pub use hash_set::HashSet;

use difference::Difference;
use ref_set::RefSet;
use union::Union;

use crate::equiv::Observable;

pub type Intersection<A, B> = Difference<A, Difference<A, B>>;

pub trait Set: Sized + Clone {
    type Item: Eq;
    fn contains(&self, value: &Self::Item) -> bool;

    fn empty() -> Empty<Self::Item> {
        Empty::new()
    }

    fn union<U>(self, other: U) -> Union<Self, U::SetType>
    where
        U: IntoSet<Item = Self::Item>,
    {
        Union::new(self, other.into_set())
    }

    fn difference<U>(self, other: U) -> Difference<Self, U::SetType>
    where
        U: IntoSet<Item = Self::Item>,
    {
        Difference::new(self, other.into_set())
    }

    fn intersection<U>(self, other: U) -> Intersection<Self, U::SetType>
    where
        U: IntoSet<Item = Self::Item>,
    {
        self.clone().difference(self.difference(other))
    }

    fn as_ref<'a>(&'a self) -> RefSet<'a, Self::Item, Self> {
        RefSet::new(self)
    }
}

impl<T, S> Observable<BTreeSet<T>> for S
where
    S: Set<Item = T> + IntoIterator<Item = T>,
    T: Ord,
{
    fn observe(self) -> BTreeSet<T> {
        self.into_iter().collect()
    }
}

impl<'a, T, S> Set for &'a S
where
    T: 'a + Eq,
    S: Set<Item = T>,
{
    type Item = &'a T;

    fn contains(&self, value: &Self::Item) -> bool {
        (*self).contains(value)
    }
}

pub trait IntoSet {
    type Item;
    type SetType: Set<Item = Self::Item>;

    fn into_set(self) -> Self::SetType;
}

impl<T: Eq, S: Set<Item = T>> IntoSet for S {
    type Item = T;
    type SetType = Self;

    fn into_set(self) -> Self::SetType {
        self
    }
}

impl<T: Eq> IntoSet for [T; 0] {
    type Item = T;
    type SetType = Empty<T>;

    fn into_set(self) -> Self::SetType {
        Empty::new()
    }
}
