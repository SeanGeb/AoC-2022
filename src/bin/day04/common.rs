use std::error::Error;

use aoc2022::utils::error::{parse_error, ParseError};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str;

pub struct Range {
    from: u64,
    to: u64,
}

impl Range {
    pub fn new(from: u64, to: u64) -> Result<Range, ParseError> {
        if from <= to {
            Ok(Range { from, to })
        } else {
            Err(parse_error(format!("from > to ({from} > {to})").as_str()))
        }
    }

    pub fn overlaps_with(&self, r2: &Range) -> bool {
        if self.from <= r2.from {
            // |--- r1
            //          |--- r2
            // therefore need r1 to end after r2 starts
            self.to >= r2.from
        } else {
            //      |--- r1
            // |--- r2
            // therefore need r2 to end after r1 starts
            r2.to >= self.from
        }
    }

    pub fn contains(&self, r2: &Range) -> bool {
        self.from <= r2.from && self.to >= r2.to
    }
}

pub fn parse_line(line: &str) -> Result<(Range, Range), Box<dyn Error>> {
    lazy_static! {
        static ref PARSER: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    }

    let vals = match PARSER.captures(line) {
        Some(v) => v,
        None => return Err(parse_error(format!("failed to match line {line}").as_str()).into()),
    };

    assert_eq!(vals.len(), 5, "bad num captures in line {line}");
    match vals
        .iter()
        .skip(1)
        .map(|x| x.unwrap().as_str().parse::<u64>())
        .collect_tuple()
    {
        Some((v1, v2, v3, v4)) => Ok((Range::new(v1?, v2?)?, Range::new(v3?, v4?)?)),
        None => Err(parse_error("???").into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_overlaps_with() {
        // r1 == r2
        assert!(Range::new(1, 5)
            .unwrap()
            .overlaps_with(&Range::new(1, 5).unwrap()));

        // r1 encloses r2
        assert!(Range::new(1, 5)
            .unwrap()
            .overlaps_with(&Range::new(2, 4).unwrap()));
        // r2 right-shifted overlapping r1
        assert!(Range::new(1, 5)
            .unwrap()
            .overlaps_with(&Range::new(2, 6).unwrap()));
        // r2 left-shifted overlapping r1
        assert!(Range::new(1, 5)
            .unwrap()
            .overlaps_with(&Range::new(2, 4).unwrap()));

        // r1 encloses r2
        assert!(Range::new(0, 6)
            .unwrap()
            .overlaps_with(&Range::new(1, 5).unwrap()));
        // r1 right-shifted overlapping r2
        assert!(Range::new(2, 6)
            .unwrap()
            .overlaps_with(&Range::new(1, 5).unwrap()));
        // r1 left-shifted overlapping r2
        assert!(Range::new(0, 4)
            .unwrap()
            .overlaps_with(&Range::new(0, 6).unwrap()));

        // r1 right-shifted touching r2
        assert!(Range::new(5, 7)
            .unwrap()
            .overlaps_with(&Range::new(1, 5).unwrap()));
        // r1 left-shifted touching r2
        assert!(Range::new(0, 5)
            .unwrap()
            .overlaps_with(&Range::new(5, 10).unwrap()));

        // non-overlaps
        // r1 far left of r2
        assert!(!Range::new(1, 5)
            .unwrap()
            .overlaps_with(&Range::new(6, 10).unwrap()));
        // r1 far right of r1
        assert!(!Range::new(6, 10)
            .unwrap()
            .overlaps_with(&Range::new(1, 5).unwrap()));
    }

    #[test]
    fn test_range_contains() {
        // r1 == r2
        assert!(Range::new(1, 5)
            .unwrap()
            .contains(&Range::new(1, 5).unwrap()));
        // |-- r1 --|
        // |-- r2 -|
        assert!(Range::new(1, 5)
            .unwrap()
            .contains(&Range::new(1, 4).unwrap()));
        // |-- r1 --|
        //  |- r2 --|
        assert!(Range::new(1, 5)
            .unwrap()
            .contains(&Range::new(2, 5).unwrap()));
        // |-- r1 --|
        //  |- r2 -|
        assert!(Range::new(1, 5)
            .unwrap()
            .contains(&Range::new(2, 4).unwrap()));
        //  |- r1 --|
        // |-- r2 -|
        assert!(!Range::new(1, 5)
            .unwrap()
            .contains(&Range::new(0, 4).unwrap()));
        // |-- r1 -|
        // |-- r2 --|
        assert!(!Range::new(1, 4)
            .unwrap()
            .contains(&Range::new(1, 5).unwrap()));
        //  |- r1 -|
        // |-- r2 --|
        assert!(!Range::new(2, 4)
            .unwrap()
            .contains(&Range::new(1, 5).unwrap()));
    }
}
