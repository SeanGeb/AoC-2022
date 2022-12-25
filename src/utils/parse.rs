use std::iter::Peekable;
use std::str::Chars;

/// A Parser is a convenience object for parsing a string input. It panics when
/// it cannot parse as required by a given call.
pub struct Parser<'a>(Peekable<Chars<'a>>);

/// Produces a next_num function for the given type and unsigned number type.
macro_rules! consume_unsigned {
    ($fun: ident, $Ty: ty) => {
        /// Consumes a
        #[doc = stringify!($Ty)]
        /// from the underlying iterator.
        pub fn $fun(&mut self) -> $Ty {
            let mut peeked = *self.0.peek().expect("expected number (input ended early)");

            if !('0'..='9').contains(&peeked) {
                panic!("expected a number")
            }

            let mut acc: $Ty = 0;
            while ('0'..='9').contains(&peeked) {
                let digit: u8 = ((self.0.next().expect("peeked item went away") as u32)
                    - ('0' as u32))
                    .try_into()
                    .unwrap();

                acc = acc
                    .checked_mul(10)
                    .expect("overflowed (mul)")
                    .checked_add(digit.into())
                    .expect("overflowed (add)");

                peeked = match self.0.peek() {
                    None => break,
                    Some(p) => *p,
                };
            }

            acc
        }
    };
}

/// As consume_unsigned, but for an unsigned integer.
macro_rules! consume_signed {
    ($fun: ident, $Ty: ty) => {
        /// Consumes a
        #[doc = stringify!($Ty)]
        /// from the underlying iterator.
        pub fn $fun(&mut self) -> $Ty {
            let mut peeked = *self.0.peek().expect("expected number (input ended early)");

            if !('0'..='9').contains(&peeked) && peeked != '-' {
                panic!("expected a number");
            }

            let is_negative = peeked == '-';
            if is_negative {
                assert_eq!(peeked, self.0.next().expect("peeked item went away"));
                peeked = *self
                    .0
                    .peek()
                    .expect("expected a number (input ended early)");
            }

            assert!(
                ('0'..='9').contains(&peeked),
                "consumed a '-' not followed by a digit"
            );

            let mut acc: $Ty = 0;
            while ('0'..='9').contains(&peeked) {
                let digit: u8 = ((self.0.next().expect("peeked item went away") as u32)
                    - ('0' as u32))
                    .try_into()
                    .unwrap();

                acc = acc
                    .checked_mul(10)
                    .expect("overflowed (mul)")
                    .checked_add_unsigned(digit.into())
                    .expect("overflowed (add)");

                peeked = match self.0.peek() {
                    None => break,
                    Some(p) => *p,
                };
            }

            if is_negative {
                acc = acc.checked_neg().expect("overflowed (neg)");
            }

            acc
        }
    };
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        Parser(value.chars().peekable())
    }
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    /// Collects the remaining unconsumed elements of the internal iterator to a
    /// String, and returns that String.
    pub fn to_string(self) -> String {
        self.0.collect()
    }

    /// Asserts that the entire input string has been consumed.
    pub fn empty(mut self) {
        assert!(
            self.is_empty(),
            "expected empty, found {:?}",
            self.to_string()
        );
    }

    pub fn is_empty(&mut self) -> bool {
        self.0.peek().is_none()
    }

    /// Consumes whitespace from the input, returning the number of whitespace
    /// characters returned.
    pub fn whitespace(&mut self) -> usize {
        for consumed in 0.. {
            let c = match self.0.peek() {
                None => return consumed,
                Some(c) => *c,
            };

            if !c.is_whitespace() {
                return consumed;
            }

            assert_eq!(
                self.0.next().expect("peeked value went away"),
                c,
                "peeked value changed"
            );
        }
        unreachable!()
    }

    /// Consumes the given prefix.
    pub fn str(&mut self, prefix: &str) {
        for c in prefix.chars() {
            assert_eq!(
                self.0
                    .next()
                    .expect("failed to match prefix (input ended early)"),
                c,
                "didn't match prefix provided"
            );
        }
    }

    consume_unsigned!(u8, u8);
    consume_unsigned!(u16, u16);
    consume_unsigned!(u32, u32);
    consume_unsigned!(u64, u64);
    consume_unsigned!(u128, u128);
    consume_unsigned!(usize, usize);
    consume_signed!(i8, i8);
    consume_signed!(i16, i16);
    consume_signed!(i32, i32);
    consume_signed!(i64, i64);
    consume_signed!(i128, i128);
    consume_signed!(isize, isize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_unsigned() {
        let mut iter: Parser = "12345   1234".into();
        assert_eq!(iter.u32(), 12345);
        assert_eq!(iter.whitespace(), 3);
        assert_eq!(iter.u32(), 1234);
        iter.empty();
    }

    #[test]
    fn test_next_signed() {
        let mut iter: Parser = "-12345 12345 -1234 1234".into();
        assert_eq!(iter.i32(), -12345);
        assert_eq!(iter.whitespace(), 1);
        assert_eq!(iter.i32(), 12345);
        assert_eq!(iter.whitespace(), 1);
        assert_eq!(iter.i32(), -1234);
        assert_eq!(iter.whitespace(), 1);
        assert_eq!(iter.i32(), 1234);
        iter.empty();
    }

    #[test]
    fn test_prefix() {
        let mut iter: Parser = "abcdefgh".into();
        iter.str("abc");
        iter.str("def");
        assert_eq!(iter.to_string().as_str(), "gh");
    }

    #[test]
    fn test_empty() {
        let mut iter: Parser = "abc".into();
    }
}
