use crate::dt;
use std::io;

fn move_type_to_intended_result(m: &dt::MoveType) -> dt::ResultType {
    match m {
        dt::MoveType::Rock => dt::ResultType::Loss,
        dt::MoveType::Paper => dt::ResultType::Draw,
        dt::MoveType::Scissors => dt::ResultType::Win,
    }
}

pub fn resolve_moves(
    moves: impl Iterator<Item = Result<dt::Move, io::Error>>,
) -> impl Iterator<Item = Result<dt::Move, io::Error>> {
    moves.map(|m| -> Result<dt::Move, io::Error> {
        let m = m?;

        let intended_result = move_type_to_intended_result(&m.we_played);
        let we_played = match intended_result {
            dt::ResultType::Win => match m.opp_played {
                dt::MoveType::Rock => dt::MoveType::Paper,
                dt::MoveType::Paper => dt::MoveType::Scissors,
                dt::MoveType::Scissors => dt::MoveType::Rock,
            },
            dt::ResultType::Draw => m.opp_played,
            dt::ResultType::Loss => match m.opp_played {
                dt::MoveType::Rock => dt::MoveType::Scissors,
                dt::MoveType::Paper => dt::MoveType::Rock,
                dt::MoveType::Scissors => dt::MoveType::Paper,
            },
        };

        Ok(dt::Move {
            opp_played: m.opp_played,
            we_played,
        })
    })
}
