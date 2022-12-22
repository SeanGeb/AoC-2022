/// Variadic macro applying std::cmp::min to all its arguments.
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}

/// Variadic macro applying std::cmp::max to all its arguments.
#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}

#[allow(unused_imports)]
pub(crate) use {max, min};

#[cfg(test)]
mod tests {
    #[test]
    fn test_min_max() {
        assert_eq!(max!(1), 1);
        assert_eq!(max!(1, 2, 3), 3);
        assert_eq!(max!(3, 2, 1), 3);
        assert_eq!(min!(2), 2);
        assert_eq!(min!(1, 2), 1);
        assert_eq!(min!(2, 1, 2), 1);
    }
}
