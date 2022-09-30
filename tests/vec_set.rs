// use std::iter::FromIterator;

// use proptest::prelude::*;
// use reldb::set::{Set, VecSet};

// trait ToVec: Iterator + Sized {
//     fn to_vec<T: FromIterator<Self::Item>>(self) -> Vec<Self::Item> {
//         self.collect()
//     }
// }

// impl<T: Iterator> ToVec for T {}

// fn vec_set() -> impl Strategy<Value = VecSet<u8>> {
//     any::<Vec<u8>>().prop_map(VecSet::from)
// }

// #[quickcheck_macros::quickcheck]
// fn q_empty_set_is_neutral_to_union(s: VecSet<u8>) {
//     let e = VecSet::new();

//     assert_eq!(s.union(&e).to_vec(), s.to_vec());
//     assert_eq!(e.union(&s).to_vec(), s.to_vec());
// }

// #[quickcheck_macros::quickcheck]
// fn union_is_associative(a: VecSet<u8>, b: VecSet<u8>, c: VecSet<u8>) {
//     assert_eq!(
//         a.union(&b).union(&c).to_vec(),
//         a.union(&(b.union(&c))).to_vec()
//     );
// }

// proptest! {
//     #[test]
//     fn empty_set_is_neutral_to_union(s in vec_set()) {
//         let e = VecSet::new();
//         assert_eq!(s.union(&e).to_vec(), s.to_vec());
//         assert_eq!(e.union(&s).to_vec(), s.to_vec());
//     }
// }
