// use quickcheck_macros::quickcheck;

use reldb::set::{Empty, Set};

#[test]
fn empty_set_contains_nothing() {
    let empty = Empty::new();

    for i in u8::MIN..u8::MAX {
        assert!(!empty.contains(&i))
    }
}

#[test]
fn empty_set_is_an_empty_collection() {
    assert_eq!(Empty::<u8>::new().into_iter().next(), None)
}
