use std::io;

use aoc2022::utils::{error::invalid_data_err, file::get_input_lines};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MoveType {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn parse_movetype(c: &char) -> Result<MoveType, io::Error> {
    match c {
        'A' | 'X' => Ok(MoveType::Rock),
        'B' | 'Y' => Ok(MoveType::Paper),
        'C' | 'Z' => Ok(MoveType::Scissors),
        _ => Err(invalid_data_err(&format!("unrecognised char {}", c))),
    }
}

pub fn parse_move(m: &String) -> Result<Move, io::Error> {
    if m.len() != 3 {
        return Err(invalid_data_err(&format!(
            "input was wrong length: got {}",
            m
        )));
    }

    let chars: Vec<char> = m.chars().collect();
    if chars.len() != 3 {
        return Err(invalid_data_err(&format!("not three chars in {}", m)));
    }

    Ok(Move {
        opp_played: parse_movetype(&chars[0])?,
        we_played: parse_movetype(&chars[2])?,
    })
}

pub fn parse_moves_from_file(
    from: &str,
) -> Result<impl Iterator<Item = Result<Move, io::Error>>, io::Error> {
    let lines = get_input_lines(from)?;

    Ok(lines.map(|l| -> Result<Move, io::Error> { parse_move(&l?) }))
}

pub enum ResultType {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn get_result(m: &Move) -> ResultType {
    let diff_value = (m.we_played as i8) - (m.opp_played as i8);
    match diff_value.rem_euclid(3) {
        0 => ResultType::Draw,
        1 => ResultType::Win,
        2 => ResultType::Loss,
        _ => panic!("diff_value out of range"),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Move {
    pub opp_played: MoveType,
    pub we_played: MoveType,
}

pub fn score_move(m: &Move) -> u64 {
    let score_played = m.we_played as u64;
    let score_win = get_result(m) as u64;
    score_played + score_win
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moves() {
        let mut moves = parse_moves_from_file("example/day02").unwrap();

        assert_eq!(
            moves.next().unwrap().unwrap(),
            Move {
                opp_played: MoveType::Rock,
                we_played: MoveType::Paper,
            }
        );

        assert_eq!(
            moves.next().unwrap().unwrap(),
            Move {
                opp_played: MoveType::Paper,
                we_played: MoveType::Rock,
            }
        );

        assert_eq!(
            moves.next().unwrap().unwrap(),
            Move {
                opp_played: MoveType::Scissors,
                we_played: MoveType::Scissors,
            }
        );

        assert!(moves.next().is_none());
    }

    #[test]
    fn test_score_moves() {
        let mut moves = parse_moves_from_file("example/day02").unwrap();
        assert_eq!(score_move(&moves.next().unwrap().unwrap()), 8);
        assert_eq!(score_move(&moves.next().unwrap().unwrap()), 1);
        assert_eq!(score_move(&moves.next().unwrap().unwrap()), 6);
        assert!(&moves.next().is_none());
    }

    #[test]
    fn test_score_all_moves() {
        let moves = parse_moves_from_file("example/day02").unwrap();
        let res: Result<u64, io::Error> = moves
            .map(|m| -> Result<u64, io::Error> { Ok(score_move(&m?)) })
            .sum();
        assert_eq!(res.unwrap(), 15);
    }
}
