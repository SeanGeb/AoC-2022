use std::cmp::max;

use itertools::Itertools;

/// Returns a boolean indicating if the elements of this iter are in strictly
/// increasing order.
pub fn is_strictly_increasing<T: Clone + PartialOrd>(
    iter: impl Iterator<Item = T>,
) -> bool {
    iter.tuple_windows().all(|(a, b)| a < b)
}

/// Maps an iterator to an iterator of booleans, where each item is true iff
/// the iterator has no larger values before that point, using the value derived
/// by applying extract_fn. Returns an iterator combining a bool to indicate
/// largest-so-far order, and the original value.
pub fn map_is_largest_so_far_f<'a, T, U: Clone + Ord + 'a>(
    iter: impl Iterator<Item = T> + 'a,
    extract_fn: impl Fn(&T) -> U + 'a,
) -> impl Iterator<Item = (bool, T)> + 'a {
    iter.scan((true, None::<U>), move |(prev_ok, largest_so_far), val| {
        let cmp_val = extract_fn(&val);
        Some({
            let (ok, cur_largest) = match &*largest_so_far {
                None => (true, cmp_val),
                Some(prev_largest) => (
                    *prev_largest < cmp_val,
                    max(prev_largest.to_owned(), cmp_val),
                ),
            };
            *prev_ok = ok;
            *largest_so_far = Some(cur_largest);
            (ok, val)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strictly_increasing() {
        assert!(is_strictly_increasing(vec![1, 2, 3].into_iter()));
        assert!(!is_strictly_increasing(vec![1, 1, 1].into_iter()));
        assert!(!is_strictly_increasing(vec![3, 2, 1].into_iter()));
    }

    fn map_is_largest_so_far<'a, T: Clone + Ord + 'a>(
        iter: impl Iterator<Item = T> + 'a,
    ) -> impl Iterator<Item = (bool, T)> + 'a {
        map_is_largest_so_far_f(iter, |x| x.to_owned())
    }

    #[test]
    fn test_map_is_largest_so_far() {
        assert_eq!(
            map_is_largest_so_far(vec![0u64; 0].into_iter()).collect_vec(),
            vec![]
        );
        assert_eq!(
            map_is_largest_so_far(vec![1].into_iter()).collect_vec(),
            vec![(true, 1)]
        );
        assert_eq!(
            map_is_largest_so_far(vec![1, 2].into_iter()).collect_vec(),
            vec![(true, 1), (true, 2)]
        );
        assert_eq!(
            map_is_largest_so_far(vec![1, 1].into_iter()).collect_vec(),
            vec![(true, 1), (false, 1)]
        );
        assert_eq!(
            map_is_largest_so_far(vec![1, 0].into_iter()).collect_vec(),
            vec![(true, 1), (false, 0)]
        );

        assert_eq!(
            map_is_largest_so_far_f(vec![1, 2, 1, 3, 2, 4].into_iter(), |x| *x)
                .collect_vec(),
            vec![
                (true, 1),
                (true, 2),
                (false, 1),
                (true, 3),
                (false, 2),
                (true, 4)
            ]
        );
    }
}
