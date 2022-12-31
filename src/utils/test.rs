use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::panic;

pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(
    f: F,
) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

/// Compares two vecs as multisets, asserting both contain the same elements (in
/// any order, but equal number).
pub fn assert_vec_eq_multiset<T: Debug + Eq + Hash>(a: Vec<T>, b: Vec<T>) {
    if a.len() != b.len() {
        panic!(
            "vecs are different lengths: {} vs {}\nlhs = {a:?}\nrhs = {b:?}",
            a.len(),
            b.len()
        );
    }

    let mut a_mset: HashMap<&T, NonZeroUsize> = HashMap::new();
    for a_item in a.iter() {
        a_mset
            .entry(a_item)
            .and_modify(|x| *x = x.checked_add(1).unwrap())
            .or_insert(1.try_into().unwrap());
    }

    for b_item in b.iter() {
        match a_mset.entry(b_item) {
            Entry::Occupied(mut e) => {
                // Decrement e, removing it if this would change it to zero.
                let v: usize = (*e.get()).into();
                if v == 1 {
                    e.remove();
                } else {
                    *e.get_mut() = (v - 1).try_into().unwrap()
                }
            },
            Entry::Vacant(e) => panic!(
                "rhs contained value {:?} not in lhs\nlhs = {a:?}\nrhs = {b:?}",
                e.key()
            ),
        }
    }

    // Theoretically, this is redundant: either we've seen everything in a_mset
    // in b, or otherwise failed, so a_mset should be empty.
    assert!(a_mset.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_eq_set() {
        assert_vec_eq_multiset::<u8>(vec![1, 2, 3], vec![3, 2, 1]);
        assert_vec_eq_multiset::<u8>(vec![1, 2, 3], vec![1, 2, 3]);
        assert_vec_eq_multiset::<u8>(vec![1, 2, 2], vec![2, 1, 2]);

        assert!(catch_unwind_silent(|| assert_vec_eq_multiset::<u8>(
            vec![1, 2, 3],
            vec![3, 2]
        ))
        .is_err());
        assert!(catch_unwind_silent(|| assert_vec_eq_multiset::<u8>(
            vec![3, 2],
            vec![1, 2, 3]
        ))
        .is_err());
        assert!(catch_unwind_silent(|| assert_vec_eq_multiset::<u8>(
            vec![],
            vec![1, 2, 3]
        ))
        .is_err());
        assert!(catch_unwind_silent(|| assert_vec_eq_multiset::<u8>(
            vec![2, 2],
            vec![2, 2, 2]
        ))
        .is_err());
        assert!(catch_unwind_silent(|| assert_vec_eq_multiset::<u8>(
            vec![2, 3, 3],
            vec![2, 2, 3]
        ))
        .is_err());
    }
}
