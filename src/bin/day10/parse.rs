use std::error::Error;
use std::io;

use crate::dt::Instruction;

pub fn parse_lines(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> impl Iterator<Item = Result<Instruction, Box<dyn Error>>> {
    lines.map(|line| Ok(line?.as_str().try_into()?))
}
