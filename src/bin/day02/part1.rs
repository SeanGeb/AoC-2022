use crate::dt;
use std::io;

pub fn score_all_moves(
    moves: impl Iterator<Item = Result<dt::Move, io::Error>>,
) -> Result<u64, io::Error> {
    moves
        .map(|m| -> Result<u64, io::Error> { Ok(dt::score_move(&m?)) })
        .sum()
}
