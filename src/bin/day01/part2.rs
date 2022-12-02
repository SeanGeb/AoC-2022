use std::cmp;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn process_lines(lines: Lines<BufReader<File>>) -> Result<u64, Box<dyn Error>> {
    Ok(find_top_n(lines, 3)?.iter().sum())
}

/// find_top_n retrieves the largest n lines of numbers, performing the required
/// summation/resetting along the way.
///
/// TODO:
/// * return an iterator over the BinaryHeap using `pop`.
/// * take an iterator of ints instead of a Lines<...>.
fn find_top_n(lines: Lines<BufReader<File>>, n: usize) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut heap = BinaryHeap::with_capacity(n);
    let mut sum_this_one: u64 = 0;

    let mut add_val = |line: String| -> Result<(), std::num::ParseIntError> {
        if line.is_empty() {
            heap.push(cmp::Reverse(sum_this_one));
            sum_this_one = 0;

            if heap.len() > 3 {
                heap.pop();
            }
        } else {
            sum_this_one += line.parse::<u64>()?;
        }

        Ok(())
    };

    for line in lines {
        add_val(line?)?;
    }

    add_val("".into())?;

    // Change our Vec<Reverse<u64>> --> Vec<u64>.
    let res: Vec<u64> = heap.into_sorted_vec().iter().map(|v| v.0).collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file;

    #[test]
    fn test_find_top_3() {
        let lines = file::get_input_lines("example/day01").unwrap();
        let res = find_top_n(lines, 3).unwrap();
        assert_eq!(res, [24000, 11000, 10000]);
    }

    #[test]
    fn test_process_lines() {
        let lines = file::get_input_lines("example/day01").unwrap();
        let res = process_lines(lines).unwrap();
        assert_eq!(res, 45000);
    }
}
