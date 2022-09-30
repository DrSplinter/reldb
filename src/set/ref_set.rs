use std::marker::PhantomData;

use crate::set::Set;

#[derive(Debug, Clone)]
pub struct RefSet<'a, T, A> {
    set: &'a A,
    data: PhantomData<T>,
}

impl<'a, T, A> RefSet<'a, T, A>
where
    T: 'a + Eq,
    A: Set<Item = T>,
{
    pub fn new(set: &'a A) -> Self {
        RefSet {
            set,
            data: PhantomData,
        }
    }
}

impl<'a, T, A> Set for RefSet<'a, T, A>
where
    T: 'a + Eq + Clone,
    A: Set<Item = T>,
{
    type Item = T;

    fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }
}
