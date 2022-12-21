use std::fmt;

use super::grid::FixedWidthDisplay;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Digit(u8);

impl TryFrom<u8> for Digit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=9).contains(&value) {
            Ok(Digit(value))
        } else {
            Err("digit is out of range: must be in 0..=9")
        }
    }
}

impl From<Digit> for u8 {
    fn from(value: Digit) -> Self {
        value.0
    }
}

impl TryFrom<char> for Digit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if ('0'..='9').contains(&value) {
            let d: u8 = ((value as u32) - ('0' as u32)).try_into().unwrap();
            Ok(d.try_into().unwrap())
        } else {
            Err("char given was not in 0..=9")
        }
    }
}

impl From<Digit> for char {
    fn from(value: Digit) -> Self {
        Self::from_u32(('0' as u32) + (value.0 as u32)).unwrap()
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FixedWidthDisplay for Digit {}
