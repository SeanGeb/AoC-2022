use std::error::Error;
use std::io;
use std::str;

use lazy_static::lazy_static;
use regex::Regex;

use aoc2022::utils::error::{parse_error, ParseError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Cd(String),
    Ls,
    Dir(String),
    File(u64, String),
}

impl str::FromStr for Token {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_CD: Regex = Regex::new(r"^\$ cd (.*)$").unwrap();
            static ref RE_LS: Regex = Regex::new(r"^\$ ls$").unwrap();
            static ref RE_DIR: Regex = Regex::new(r"^dir (.*)$").unwrap();
            static ref RE_FILE: Regex = Regex::new(r"^(\d+) (.*)$").unwrap();
        }

        if let Some(caps) = RE_CD.captures_iter(s).next() {
            return Ok(Token::Cd(caps.get(1).unwrap().as_str().to_string()));
        }

        if RE_LS.is_match(s) {
            return Ok(Token::Ls);
        }

        if let Some(caps) = RE_DIR.captures_iter(s).next() {
            return Ok(Token::Dir(caps.get(1).unwrap().as_str().to_string()));
        }

        if let Some(caps) = RE_FILE.captures_iter(s).next() {
            let size: u64 = str::parse(caps.get(1).unwrap().into()).unwrap();
            let name = caps.get(2).unwrap().as_str();
            return Ok(Token::File(size, name.to_string()));
        }

        Err(parse_error(format!("No match for {s}").as_str()))
    }
}

pub fn parse_lines(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> impl Iterator<Item = Result<Token, Box<dyn Error>>> {
    lines.map(|v| match v {
        Ok(line) => match str::parse::<Token>(&line) {
            Ok(token) => Ok(token),
            Err(e) => Err(e.into()),
        },
        Err(e) => Err(e.into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            str::parse::<Token>("$ cd /").unwrap(),
            Token::Cd("/".to_string())
        );
        assert_eq!(str::parse::<Token>("$ ls").unwrap(), Token::Ls);
        assert_eq!(
            str::parse::<Token>("dir a").unwrap(),
            Token::Dir("a".to_string())
        );
        assert_eq!(
            str::parse::<Token>("1234 bar").unwrap(),
            Token::File(1234, "bar".to_string())
        );
    }
}
