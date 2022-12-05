use bstr::io::BufReadExt;
use std::{io::BufRead, simd::Simd};

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
    pub const fn from_id(id: u8) -> Self {
        match id {
            2 => Self::Win,
            1 => Self::Draw,
            0 => Self::Lose,
            _ => unreachable!(),
        }
    }

    pub const fn response_for(self, move_: Move) -> Move {
        match self {
            Self::Win => move_.better(),
            Self::Draw => move_,
            Self::Lose => move_.worse(),
        }
    }

    pub const fn score_for_response(self, move_: Move) -> u32 {
        let response = self.response_for(move_);
        response.score_against(move_)
    }
}

impl Move {
    pub const fn from_id(id: u8) -> Self {
        match id {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub const fn better(self) -> Self {
        Self::from_id((self as u8 + 1) % 3)
    }

    pub const fn worse(self) -> Self {
        Self::from_id((self as u8 + 2) % 3)
    }

    #[inline(always)]
    pub const fn eq(self, other: Self) -> bool {
        self as u32 == other as u32
    }

    pub const fn fight(self, other: Move) -> MatchResult {
        if self.eq(other) {
            MatchResult::Draw
        } else if self.better().eq(other) {
            MatchResult::Lose
        } else {
            MatchResult::Win
        }
    }

    pub const fn score(self, result: MatchResult) -> u32 {
        self as u32 + result as u32 + 1
    }

    pub const fn score_against(self, other: Move) -> u32 {
        self.score(self.fight(other))
    }
}

const OPTIONS_FOR_MOVES: &[u32] = &[
    // A X
    Move::Rock.score_against(Move::Rock),
    // B X
    Move::Rock.score_against(Move::Paper),
    // C X
    Move::Rock.score_against(Move::Scissors),
    // ? X
    0,
    // A Y
    Move::Paper.score_against(Move::Rock),
    // B Y
    Move::Paper.score_against(Move::Paper),
    // C Y
    Move::Paper.score_against(Move::Scissors),
    // ? Y
    0,
    // A Z
    Move::Scissors.score_against(Move::Rock),
    // B Z
    Move::Scissors.score_against(Move::Paper),
    // C Z
    Move::Scissors.score_against(Move::Scissors),
];

const OPTIONS_FOR_RESULTS: &[u32] = &[
    // A X
    MatchResult::Lose.score_for_response(Move::Rock),
    // B X
    MatchResult::Lose.score_for_response(Move::Paper),
    // C X
    MatchResult::Lose.score_for_response(Move::Scissors),
    // ? X
    0,
    // A Y
    MatchResult::Draw.score_for_response(Move::Rock),
    // B Y
    MatchResult::Draw.score_for_response(Move::Paper),
    // C Y
    MatchResult::Draw.score_for_response(Move::Scissors),
    // ? Y
    0,
    // A Z
    MatchResult::Win.score_for_response(Move::Rock),
    // B Z
    MatchResult::Win.score_for_response(Move::Paper),
    // C Z
    MatchResult::Win.score_for_response(Move::Scissors),
];

const SIMD_WIDTH_BITS: usize = 512;
const SIMD_WIDTH: usize = SIMD_WIDTH_BITS / 32;

// Every line is a four-byte entry in the shape of 'A X\n'. If we interpret it as a u32,
// we can subtract LINE_BASE to get a binary entry in the shape of '...00...11' where
// the two bits at the respective A/X position represent numbers 0-2 for the move (or result).
const LINE_BASE: u32 =
    (b'A' as u32) + ((b' ' as u32) << 8) + ((b'X' as u32) << 16) + ((b'\n' as u32) << 24);

const LINE_BASE_SIMD: Simd<u32, SIMD_WIDTH> = Simd::from_array([LINE_BASE; SIMD_WIDTH]);
const RIGHT_SHIFT_SIMD: Simd<u32, SIMD_WIDTH> = Simd::from_array([14; SIMD_WIDTH]);
const BIT_MASK_SIMD: Simd<u32, SIMD_WIDTH> = Simd::from_array([0b1111; SIMD_WIDTH]);

#[inline(always)]
fn simdify_entries(input: &[u8; SIMD_WIDTH_BITS / 8]) -> [u32; SIMD_WIDTH] {
    // assuming that we are on a little-endian architecture because that's how LINE_BASE is declared
    // SAFETY: honestly don't know why this would be unsafe. We're reinterpreting 4xu8 as 1xu32 so what could go wrong?
    let (_, input_u32, _) = unsafe { input.align_to::<u32>() };
    let input_vec: Simd<u32, SIMD_WIDTH> = Simd::from_slice(&input_u32);

    // subtract the line base to get a binary entry in the shape of '...00...11'
    let base_vec = input_vec - LINE_BASE_SIMD;
    // shift the bits to the right to get a binary entry in the shape of '...0011'
    // mask the bits to get a binary entry '0011'
    let bitshifted = ((base_vec >> RIGHT_SHIFT_SIMD) + base_vec) & BIT_MASK_SIMD;
    
    bitshifted.to_array()
}

fn part1_play_strategy(input: &mut dyn BufRead) -> u64 {
    let mut total_score = 0;
    let mut entry = [0u8; SIMD_WIDTH_BITS / 8];
    loop {
        let bytes_read = input.read(&mut entry).unwrap();
        if bytes_read == 0 {
            break;
        }
        let arr = simdify_entries(&entry);
        for i in 0..(bytes_read / 4) {
            total_score += OPTIONS_FOR_MOVES[arr[i] as usize] as u64;
        }
    }
    total_score
}

fn part2_play_to_result(input: &mut dyn BufRead) -> u64 {
    let mut total_score = 0;
    let mut entry = [0u8; SIMD_WIDTH_BITS / 8];
    loop {
        let bytes_read = input.read(&mut entry).unwrap();
        if bytes_read == 0 {
            break;
        }
        let arr = simdify_entries(&entry);
        for i in 0..(bytes_read / 4) {
            total_score += OPTIONS_FOR_RESULTS[arr[i] as usize] as u64;
        }
    }    total_score
}

pub const SOLVERS: &[Solver] = &[part1_play_strategy, part2_play_to_result];
