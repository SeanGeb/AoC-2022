use std::fmt::{Display, Write};
use std::iter::Peekable;

#[derive(Debug)]
pub enum MaybeVec {
    One(u32),
    Vec(Vec<MaybeVec>),
}

impl Display for MaybeVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One(n) => write!(f, "{n}"),
            Self::Vec(v) => {
                f.write_char('[')?;
                for (i, e) in v.iter().enumerate() {
                    e.fmt(f)?;
                    if i < v.len() - 1 {
                        f.write_str(", ")?;
                    }
                }
                f.write_char(']')
            },
        }
    }
}

impl PartialEq for MaybeVec {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::One(l), Self::One(r)) => l == r,
            (Self::Vec(l), Self::Vec(r)) => l == r,
            (Self::One(l), Self::Vec(r)) => {
                let l = vec![Self::One(*l)];
                &l == r
            },
            (Self::Vec(l), Self::One(r)) => {
                let r = vec![Self::One(*r)];
                l == &r
            },
        }
    }
}

impl Eq for MaybeVec {}

impl PartialOrd for MaybeVec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaybeVec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::One(a), Self::One(b)) => a.cmp(b),
            (Self::Vec(a), Self::Vec(b)) => a.cmp(b),
            (Self::One(a), Self::Vec(b)) => {
                let a: Vec<MaybeVec> = vec![Self::One(*a)];
                a.cmp(&b)
            },
            (Self::Vec(a), Self::One(b)) => {
                let b: Vec<MaybeVec> = vec![Self::One(*b)];
                a.cmp(&b)
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Open,
    Close,
    Number(u32),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => f.write_char('['),
            Self::Close => f.write_char(']'),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

pub fn parse(input: &str) -> MaybeVec {
    TokenIter::new(input.chars()).parse()
}

struct TokenIter<T: Iterator<Item = char>>(Peekable<T>);

impl<T: Iterator<Item = char>> TokenIter<T> {
    fn new(iter: T) -> Self {
        Self(iter.peekable())
    }

    fn parse(mut self) -> MaybeVec {
        match self.next().expect("unexpected end of input") {
            Token::Open => {
                let r = MaybeVec::Vec(self.consume_vec());
                if let Some(t) = self.next() {
                    panic!("unexpected token {t} at end of stream");
                }
                r
            },
            t => panic!("expected [ at start of stream, got {t}"),
        }
    }

    fn consume_vec(&mut self) -> Vec<MaybeVec> {
        // Parses from self until a closing bracket. Assumes the opening bracket
        // has been consumed.
        let mut res: Vec<MaybeVec> = Vec::new();
        loop {
            let next = match self.next().expect("unexpected end of input") {
                Token::Close => return res,
                Token::Open => MaybeVec::Vec(self.consume_vec()),
                Token::Number(n) => MaybeVec::One(n),
            };
            res.push(next);
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for TokenIter<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.0.next()? {
            '[' => Self::Item::Open,
            ']' => {
                // Also consume a trailing ','.
                if self.0.peek() == Some(&',') {
                    self.0.next().unwrap();
                }
                Self::Item::Close
            },
            c @ '0'..='9' => {
                let mut n = (c as u32) - ('0' as u32);
                loop {
                    let next_digit = match self.0.peek() {
                        None => break,
                        Some(c) => match c {
                            '0'..='9' => {
                                (self.0.next().unwrap() as u32) - ('0' as u32)
                            },
                            ',' => {
                                self.0.next();
                                break;
                            },
                            ']' => break,
                            c => panic!(
                                "unexpected char '{c}' (expected digit or ,)"
                            ),
                        },
                    };
                    n = n
                        .checked_mul(10)
                        .expect("overflow (mul)")
                        .checked_add(next_digit)
                        .expect("overflow (add)");
                }
                Self::Item::Number(n)
            },
            c => panic!("unexpected char '{c}' (expected bracket or number)"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(
            parse("[1,2,3,[123,5,[]]]"),
            MaybeVec::Vec(vec![
                MaybeVec::One(1),
                MaybeVec::One(2),
                MaybeVec::One(3),
                MaybeVec::Vec(vec![
                    MaybeVec::One(123),
                    MaybeVec::One(5),
                    MaybeVec::Vec(vec![]),
                ]),
            ])
        );
    }

    #[test]
    fn test_cmp() {
        assert!(parse("[1]") < parse("[2]"));
        assert!(parse("[1,2]") < parse("[2,2]"));
        assert!(parse("[1]") < parse("[1,0]"));
        assert!(parse("[1]") == parse("[[1]]"));
        assert!(parse("[1,2]") == parse("[[1],2]"));
        assert!(parse("[1,2]") < parse("[[1],3]"));
        assert!(parse("[1,3]") > parse("[[1],2]"));

        assert!(parse("[1,1,3,1,1]") < parse("[1,1,5,1,1]"));
        assert!(parse("[[1],[2,3,4]]") < parse("[[1],4]"));
        assert!(parse("[9]") > parse("[[8,7,6]]"));
        assert!(parse("[[4,4],4,4]") < parse("[[4,4],4,4,4]"));
        assert!(parse("[7,7,7,7]") > parse("[7,7,7]"));
        assert!(parse("[]") < parse("[3]"));
        assert!(parse("[[[]]]") > parse("[[]]"));
        assert!(
            parse("[1,[2,[3,[4,[5,6,7]]]],8,9]")
                > parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        );
    }
}
