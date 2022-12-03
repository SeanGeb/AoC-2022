use std::cmp;
use std::collections::BinaryHeap;
use std::io;

use aoc2022::utils::error::{invalid_data_err, invalid_data_err_from};

pub fn process_lines(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<u64, io::Error> {
    Ok(find_top_n::<3>(lines)?.iter().sum())
}

/// find_top_n retrieves the largest n lines of numbers, performing the required
/// summation/resetting along the way.
fn find_top_n<const N: usize>(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<[u64; N], io::Error> {
    let mut heap = BinaryHeap::with_capacity(N + 1);
    let mut sum_this_one: u64 = 0;

    let mut add_val = |line: String| -> Result<(), io::Error> {
        if line.is_empty() {
            heap.push(cmp::Reverse(sum_this_one));
            sum_this_one = 0;

            if heap.len() > N {
                heap.pop();
            }
        } else {
            sum_this_one += line.parse::<u64>().map_err(invalid_data_err_from)?;
        }

        Ok(())
    };

    for line in lines {
        add_val(line?)?
    }

    add_val("".into())?;

    // Raise error if too few values were provided.
    if heap.len() != N {
        return Err(invalid_data_err("wrong number of heap items"));
    }

    // Empty the heap into an array.
    let mut res = [0u64; N];
    for res_i in res.iter_mut() {
        *res_i = heap.pop().unwrap().0;
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::utils::file::get_input_lines;

    #[test]
    fn test_find_top_3() {
        let lines = get_input_lines("example/day01").unwrap();
        let res = find_top_n::<3>(lines).unwrap();
        assert_eq!(res, [10000, 11000, 24000]);
    }

    #[test]
    fn test_process_lines() {
        let lines = get_input_lines("example/day01").unwrap();
        let res = process_lines(lines).unwrap();
        assert_eq!(res, 45000);
    }
}
