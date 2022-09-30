use quickcheck_macros::quickcheck;

use reldb::equiv::equiv;
use reldb::set::{Empty, HashSet, IntoSet, Set};

#[quickcheck]
fn dummy(a: std::collections::HashSet<u8>) -> bool {
    let x = &a;
    let y = &HashSet::from([1u8]);

    equiv(x.into_set(), y)
}

#[quickcheck]
fn hash_set_is_the_same_after_coertion(a: std::collections::HashSet<u8>) -> bool {
    a.clone() == a.into_set().into_iter().collect()
}

#[quickcheck]
fn empty_set_is_right_neutral_to_union(a: HashSet<u8>) -> bool {
    equiv((&a).union(Empty::new()), &a)
}

#[quickcheck]
fn empty_set_is_left_neutral_to_union(a: HashSet<u8>) -> bool {
    equiv(Empty::new().union(&a), &a)
}

#[quickcheck]
fn union_is_commutative(a: HashSet<u8>, b: HashSet<u8>) -> bool {
    equiv((&a).union(&b), (&b).union(&a))
}

#[quickcheck]
fn union_is_associative(a: HashSet<u8>, b: HashSet<u8>, c: HashSet<u8>) -> bool {
    equiv((&a).union(&b).union(&c), (&a).union((&b).union(&c)))
}

#[quickcheck]
fn difference_anihilates_itself(a: HashSet<u8>) -> bool {
    equiv((&a).difference(&a), Empty::new())
}

#[quickcheck]
fn union_as_difference(a: HashSet<u8>, b: HashSet<u8>) -> bool {
    equiv((&a).union(&b), (&a).union((&b).difference(&a)))
}

#[quickcheck]
fn intersection_is_associative(a: HashSet<u8>, b: HashSet<u8>, c: HashSet<u8>) -> bool {
    equiv(
        (&a).intersection(&b).intersection(&c),
        (&a).intersection((&b).intersection(&c)),
    )
}

#[quickcheck]
fn empty_set_right_anihilates_intersection(a: HashSet<u8>) -> bool {
    equiv((&a).intersection(Empty::new()), Empty::new())
}

#[quickcheck]
fn empty_set_left_anihilates_intersection(a: HashSet<u8>) -> bool {
    equiv(Empty::new().intersection(&a), Empty::new())
}
