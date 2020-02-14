use crate::models::GameResult::{Draw, Lose, Win};
use crate::models::Move::{Rock, Paper, Scissors};
use std::fmt::Error;

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_int(int: i32) -> Result<Move, String> {
        match int {
            1 => Ok(Rock),
            2 => Ok(Paper),
            3 => Ok(Scissors),
            _ => Err(format!("incorrect number {}", int))
        }
    }

    pub fn beats(&self, opponent: &Move) -> GameResult {
        match self {
            Rock => match opponent {
                Rock => Draw,
                Paper => Lose,
                Scissors => Win
            },
            Paper => match opponent {
                Rock => Win,
                Paper => Lose,
                Scissors => Lose
            },
            Scissors => match opponent {
                Rock => Lose,
                Paper => Win,
                Scissors => Draw
            },
        }
    }
}

pub enum GameResult {
    Win,
    Lose,
    Draw,
}