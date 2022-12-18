use std::error::Error;
use std::io;

use aoc2022::utils::file::get_input_lines;

use crate::dt::Dir;
use crate::parse::{parse_lines, Token};

mod dt;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tree = get_tree(get_input_lines("day07")?)?;
    let total_size = tree.update_contents_size();
    println!("{tree}");
    println!("total size: {total_size}");

    println!("part one: {}", tree.sum_dir_size_lte(100_000));

    let delete_target = total_size - 40_000_000;
    eprintln!("try delete {delete_target} bytes");
    println!("part two: {}", tree.smallest_dir_size_gte(delete_target));

    Ok(())
}

fn get_tree(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<dt::Dir, Box<dyn Error>> {
    let mut path: Vec<String> = vec![];
    let mut root = Dir::new();
    let mut cwd = &mut root;

    for cmd in parse_lines(lines) {
        match cmd? {
            Token::Cd(dir) => match dir.as_str() {
                ".." => {
                    path.pop().unwrap();
                    cwd = root.cd(&path);
                },
                dir => {
                    cwd = cwd.cd(&[dir.to_string()]);
                    path.push(dir.to_string());
                },
            },
            Token::Ls => (),
            Token::Dir(_) => (),
            Token::File(size, name) => cwd.add_file(name, size),
        };
    }

    println!("final path was {}", path.join("\\"));

    Ok(root)
}
