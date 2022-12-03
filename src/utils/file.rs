use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::str;

/// get_input_lines takes a file path under `data`, minus extension, and returns
/// an iterator to retrieve one line at a time.
pub fn get_input_lines(file: &str) -> Result<Lines<BufReader<File>>, io::Error> {
    let data = File::open(format!("data/{file}.txt"))?;

    Ok(BufReader::new(data).lines())
}
