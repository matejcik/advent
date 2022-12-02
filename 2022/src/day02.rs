use bstr::io::BufReadExt;
use std::io::BufRead;

use crate::Solver;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock = 0,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum MatchResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl MatchResult {
    pub fn from_letter(letter: u8, base: u8) -> Self {
        match letter - base {
            2 => Self::Win,
            1 => Self::Draw,
            0 => Self::Lose,
            _ => panic!("Invalid match result: {}", letter as char),
        }
    }

    pub fn response_for(self, move_: Move) -> Move {
        match self {
            Self::Win => move_.better(),
            Self::Draw => move_,
            Self::Lose => move_.worse(),
        }
    }
}

impl Move {
    const MOVES: &[Move] = &[Move::Rock, Move::Paper, Move::Scissors];

    pub fn from_letter(letter: u8, base: u8) -> Self {
        Self::MOVES[(letter - base) as usize]
    }

    pub fn better(self) -> Self {
        let index = (self as usize + 1) % Self::MOVES.len();
        Self::MOVES[index]
    }

    pub fn worse(self) -> Self {
        let index = (self as usize + Self::MOVES.len() - 1) % Self::MOVES.len();
        Self::MOVES[index]
    }

    pub fn fight(self, other: Move) -> MatchResult {
        if self == other {
            MatchResult::Draw
        } else if self.better() == other {
            MatchResult::Lose
        } else {
            MatchResult::Win
        }
    }

    pub fn score(self, result: MatchResult) -> u64 {
        self as u64 + result as u64 + 1
    }
}

fn part1_play_strategy(mut input: &mut dyn BufRead) -> String {
    let mut total_score = 0;
    input
        .for_byte_line(|line| {
            let elf = Move::from_letter(line[0], b'A');
            let me = Move::from_letter(line[2], b'X');
            total_score += me.score(me.fight(elf));
            Ok(true)
        })
        .unwrap();

    total_score.to_string()
}

fn part2_play_to_result(mut input: &mut dyn BufRead) -> String {
    let mut total_score = 0;
    input
        .for_byte_line(|line| {
            let elf = Move::from_letter(line[0], b'A');
            let my_result = MatchResult::from_letter(line[2], b'X');
            let response = my_result.response_for(elf);
            total_score += response.score(my_result);
            Ok(true)
        })
        .unwrap();

    total_score.to_string()
}

pub const SOLVERS: &[Solver] = &[part1_play_strategy, part2_play_to_result];
