use std::cmp::max;
use std::error::Error;

use aoc2022::types::digit::Digit;
use aoc2022::types::grid::Grid;
use aoc2022::utils::file::get_input_lines;
use aoc2022::utils::iter;

mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let grid = parse::parse_input(get_input_lines("day08")?)?;

    // Part 1: create a grid of bools for the visible trees, iterating in all
    // four directions (← → ↑ ↓), setting visibility whenever we can see a tree
    // from the edge before encountering a taller tree.
    let mut visibility_grid = Grid::new(false, grid.width(), grid.height());
    update_visibility_grid(&mut visibility_grid, grid.iter_rows());
    update_visibility_grid(&mut visibility_grid, grid.iter_rev_rows());
    update_visibility_grid(&mut visibility_grid, grid.iter_cols());
    update_visibility_grid(&mut visibility_grid, grid.iter_rev_cols());

    // Then count the number of visible trees.
    println!(
        "Part one: {}",
        visibility_grid
            .enumerate()
            .filter_map(|(_, ok)| if *ok { Some(()) } else { None })
            .count()
    );

    // Part 2: for each tree, count the number of trees before finding one of
    // equal or greater size, and multiply the numbers from each direction to
    // get the visibility score. Find the largest such score.
    let mut largest_score = 0;
    for ((x, y), from_height) in grid.enumerate() {
        // Upwards
        let mut score_up = 0;
        for y_2 in (0..y).rev() {
            score_up += 1;
            if grid.get(x, y_2).unwrap() >= from_height {
                break;
            }
        }

        // Downwards
        let mut score_down = 0;
        for y_2 in (y + 1)..grid.height() {
            score_down += 1;
            if grid.get(x, y_2).unwrap() >= from_height {
                break;
            }
        }

        // Leftwards
        let mut score_left = 0;
        for x_2 in (0..x).rev() {
            score_left += 1;
            if grid.get(x_2, y).unwrap() >= from_height {
                break;
            }
        }

        // Rightwards
        let mut score_right = 0;
        for x_2 in (x + 1)..grid.width() {
            score_right += 1;
            if grid.get(x_2, y).unwrap() >= from_height {
                break;
            }
        }

        largest_score = max(
            largest_score,
            score_right * score_left * score_up * score_down,
        );
    }
    println!("Part two: {largest_score}");

    Ok(())
}

fn update_visibility_grid<'a>(
    vis_map: &mut Grid<bool>,
    grid_iter: impl Iterator<
        Item = impl Iterator<Item = ((usize, usize), &'a Digit)>,
    >,
) {
    grid_iter.for_each(|row_iter| {
        iter::map_is_largest_so_far_f(row_iter, |((_, _), h)| *h)
            .filter(|(ok, _)| *ok)
            .for_each(|(_, ((x, y), _))| {
                *(vis_map.get_mut(x, y).unwrap()) = true
            })
    })
}
