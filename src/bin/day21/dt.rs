use std::error::Error;
use std::fmt::{Debug, Display, Write};
use std::str::FromStr;

use aoc2022::utils::file::get_input_lines;

/// An Op is one of the five binary operators each monkey supports.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Op {
    /// Given the operands, apply this operator to those operands and return
    /// the result.
    pub fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs.checked_add(rhs),
            Self::Sub => lhs.checked_sub(rhs),
            Self::Mul => lhs.checked_mul(rhs),
            Self::Div => lhs.checked_div(rhs),
            Self::Eq => {
                assert_eq!(lhs, rhs);
                Some(lhs)
            },
        }
        .unwrap()
    }

    /// Given the right-hand operand and the value to which the operation must
    /// resolve to, determine the value to which the left-hand subtree must
    /// resolve to.
    pub fn find_lhs(&self, rhs: i64, val: i64) -> i64 {
        match self {
            Op::Add => val.checked_sub(rhs).unwrap(), // ? + r = v
            Op::Sub => val.checked_add(rhs).unwrap(), // ? - r = v
            Op::Mul => val.checked_div(rhs).unwrap(), // ? * r = v
            Op::Div => val.checked_mul(rhs).unwrap(), // ? / r = v
            Op::Eq => rhs,
        }
    }

    /// As find_lhs, but determines the right-hand operand.
    pub fn find_rhs(&self, lhs: i64, val: i64) -> i64 {
        match self {
            Op::Add => val.checked_sub(lhs).unwrap(), // l + ? = v
            Op::Sub => lhs.checked_sub(val).unwrap(), // l - ? = v // NB: order
            Op::Mul => val.checked_div(lhs).unwrap(), // l * ? = v
            Op::Div => lhs.checked_div(val).unwrap(), // l / ? = v // NB: order
            Op::Eq => lhs,
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            s => Err(format!("unable to parse op {s:?}")),
        }
    }
}

/// An identifier for a given monkey e.g. "root" or "dbpl".
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Ident([u8; 4]);

pub const IDENT_ROOT: Ident = Ident(*b"root");
pub const IDENT_HUMN: Ident = Ident(*b"humn");

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            f.write_char(c as char)?
        }
        Ok(())
    }
}

impl FromStr for Ident {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| -> u8 {
                    assert!(c.is_ascii(), "non-ASCII char {c}");
                    c.try_into().unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }
}

/// An expression consisting of a binary operator and the monkeys providing the
/// operands.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Expr {
    pub lhs: Ident,
    pub op: Op,
    pub rhs: Ident,
}

/// An Expr with an associated monkey name i.e. a representation of a monkey
/// with an operation to perform.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NamedExpr {
    pub name: Ident,
    pub expr: Expr,
}

/// An enum used during the solve that can contain an expression, value, or
/// the single unknown value to determine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolvableExpr {
    Expr(Expr),
    Val(i64),
    Unknown,
}

/// A ResolvableExpr with an associated name, just like NamedExpr.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct NamedResolvableExpr {
    pub name: Ident,
    pub expr: ResolvableExpr,
}

impl FromStr for NamedResolvableExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();

        if parts.is_empty() {
            return Err(format!("{s:?} cannot be split"));
        }

        let name: Ident =
            parts[0].chars().take(4).collect::<String>().parse()?;

        match parts.len() {
            2 => Ok(Self {
                name,
                expr: ResolvableExpr::Val(
                    parts[1]
                        .parse()
                        .map_err(|e| format!("unable to parse {s:?}: {e}"))?,
                ),
            }),
            4 => Ok(Self {
                name,
                expr: ResolvableExpr::Expr(Expr {
                    lhs: parts[1].parse()?,
                    op: parts[2].parse()?,
                    rhs: parts[3].parse()?,
                }),
            }),
            n => Err(format!("unable to parse {s:?}: returns {n} parts")),
        }
    }
}

pub fn parse_lines(
    file: &str,
) -> Result<Vec<NamedResolvableExpr>, Box<dyn Error>> {
    let lines = get_input_lines(file)?;
    let mut r: Vec<NamedResolvableExpr> = Vec::new();
    for line in lines {
        r.push(line?.parse()?);
    }
    Ok(r)
}
