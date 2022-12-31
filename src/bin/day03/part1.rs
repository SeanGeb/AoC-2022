use aoc2022::utils::error;
use itertools::Itertools;

use crate::common;
use std::collections::HashSet;
use std::io;

pub fn solve(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<u64, io::Error> {
    // Summary for part1: take inputs, split in half, and find the common letter.
    lines
        .map(|line| -> Result<u64, io::Error> {
            let line = line?;
            assert_eq!(line.len(), line.bytes().len());
            assert_eq!(line.len() % 2, 0);

            let (cmp1, cmp2) = line.split_at(line.len() / 2);
            assert_eq!(cmp1.len(), cmp2.len());

            let set1: HashSet<u8> = HashSet::from_iter(cmp1.bytes());
            let set2: HashSet<u8> = HashSet::from_iter(cmp2.bytes());

            let common_item = set1
                .intersection(&set2)
                .exactly_one()
                .map_err(error::invalid_data_err_from)?;

            Ok(common::score(common_item))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc2022::utils::file::get_input_lines;

    use super::*;

    #[test]
    fn test_solve() {
        let lines = get_input_lines("example/day03").unwrap();
        assert_eq!(solve(lines).unwrap(), 157);
    }
}
