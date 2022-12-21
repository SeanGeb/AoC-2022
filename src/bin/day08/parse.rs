use std::error::Error;
use std::io;

use aoc2022::types::digit::Digit;
use aoc2022::types::grid::Grid;

pub fn parse_input(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<Grid<Digit>, Box<dyn Error>> {
    let mut grid: Vec<Vec<Digit>> = Vec::new();

    for line in lines {
        let mut row: Vec<Digit> = Vec::new();
        for char in line?.chars() {
            row.push(char.try_into()?);
        }
        row.shrink_to_fit();
        grid.push(row);
    }
    grid.shrink_to_fit();

    Ok(grid.try_into()?)
}
