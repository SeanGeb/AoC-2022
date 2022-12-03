use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str;

/// get_input_lines takes a file path under `data`, minus extension, and returns
/// an iterator to retrieve one line at a time.
pub fn get_input_lines(
    file: &str,
) -> Result<impl Iterator<Item = Result<String, io::Error>>, io::Error> {
    let data = File::open(format!("data/{file}.txt"))?;

    Ok(BufReader::new(data).lines())
}
