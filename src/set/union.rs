use std::{hash::Hash, ops::Add};

use crate::set::Set;

///
/// Union
///

#[derive(Debug, Clone)]
pub struct Union<A, B> {
    a: A,
    b: B,
}

impl<A, B> Union<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Union { a, b }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Set

impl<A, B, T: Eq> Set for Union<A, B>
where
    A: Set<Item = T>,
    B: Set<Item = T>,
{
    type Item = T;

    fn contains(&self, value: &T) -> bool {
        self.a.contains(value) || self.b.contains(value)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Iterator

impl<A, B, T: Eq + Hash + Clone> IntoIterator for Union<A, B>
where
    A: IntoIterator<Item = T>,
    B: IntoIterator<Item = T>,
{
    type Item = T;

    type IntoIter = UnionIter<A::IntoIter, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        UnionIter::new(self.a.into_iter(), self.b.into_iter())
    }
}

pub struct UnionIter<AI: Iterator, BI> {
    // Cannot have another type parameter for item because compiler fails
    // to due to some unlimited recursion
    cache: std::collections::HashSet<AI::Item>,
    switch: UnionSwitch,
    a_iter: Option<AI>,
    b_iter: Option<BI>,
}
enum UnionSwitch {
    A,
    B,
}

impl UnionSwitch {
    fn toggle(&self) -> Self {
        match self {
            UnionSwitch::A => UnionSwitch::B,
            UnionSwitch::B => UnionSwitch::A,
        }
    }
}

impl<AI, BI, T> UnionIter<AI, BI>
where
    AI: Iterator<Item = T>,
    BI: Iterator<Item = T>,
{
    pub fn new(a: AI, b: BI) -> Self {
        Self {
            cache: std::collections::HashSet::new(),
            switch: UnionSwitch::A,
            a_iter: Some(a),
            b_iter: Some(b),
        }
    }
}

impl<AI, BI, T> Iterator for UnionIter<AI, BI>
where
    T: Eq + Hash + Clone,
    AI: Iterator<Item = T>,
    BI: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (&mut self.a_iter, &mut self.b_iter) {
            (Some(ai), Some(bi)) => match &self.switch {
                UnionSwitch::A => match ai.next() {
                    Some(next) => {
                        if self.cache.insert(next.clone()) {
                            self.switch.toggle();
                            Some(next)
                        } else {
                            self.next()
                        }
                    }
                    None => {
                        self.a_iter = None;
                        self.next()
                    }
                },
                UnionSwitch::B => match bi.next() {
                    Some(next) => {
                        if self.cache.insert(next.clone()) {
                            self.switch.toggle();
                            Some(next)
                        } else {
                            self.next()
                        }
                    }
                    None => {
                        self.b_iter = None;
                        self.next()
                    }
                },
            },
            (None, Some(bi)) => bi.next(),
            (Some(ai), None) => ai.next(),
            (None, None) => None,
        }
    }
}
