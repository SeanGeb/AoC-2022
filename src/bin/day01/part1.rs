use std::cmp;
use std::io;

pub fn process_lines(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<u64, io::Error> {
    let mut max_so_far: u64 = 0;
    let mut sum_this_one: u64 = 0;

    for line in lines {
        let line = line?;

        if line.is_empty() {
            max_so_far = cmp::max(max_so_far, sum_this_one);
            sum_this_one = 0;
        } else {
            let val: u64 = line.parse().expect("Unable to parse num");
            sum_this_one += val;
        }
    }

    max_so_far = cmp::max(max_so_far, sum_this_one);

    Ok(max_so_far)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::utils::file::get_input_lines;

    #[test]
    fn test_process_lines() {
        let lines = get_input_lines("example/day01").unwrap();
        let res = process_lines(lines).unwrap();
        assert_eq!(res, 24000);
    }
}
