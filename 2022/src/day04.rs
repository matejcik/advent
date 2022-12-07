use std::io::BufRead;

use bstr::io::BufReadExt;

use crate::{parse_nums, Solver};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Interval(u64, u64);

impl Interval {
    pub fn new(min: u64, max: u64) -> Self {
        Self(min, max)
    }

    pub fn contains_or_contained(&self, other: Interval) -> bool {
        (self.0 <= other.0 && self.1 >= other.1) || (self.0 >= other.0 && self.1 <= other.1)
    }

    pub fn overlaps(&self, other: Interval) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

fn part1_count_total_overlaps(mut input: &mut dyn BufRead) -> String {
    let mut total = 0;
    let mut numbers = [0; 4];
    input
        .for_byte_line(|line| {
            parse_nums(line, &mut numbers);
            let l = Interval::new(numbers[0], numbers[1]);
            let r = Interval::new(numbers[2], numbers[3]);
            total += l.contains_or_contained(r) as u64;
            Ok(true)
        })
        .unwrap();
    total.to_string()
}

fn part2_count_partial_overlaps(mut input: &mut dyn BufRead) -> String {
    let mut total = 0;
    let mut numbers = [0; 4];
    input
        .for_byte_line(|line| {
            parse_nums(line, &mut numbers);
            let l = Interval::new(numbers[0], numbers[1]);
            let r = Interval::new(numbers[2], numbers[3]);
            total += l.overlaps(r) as u64;
            Ok(true)
        })
        .unwrap();
    total.to_string()
}

pub const SOLVERS: &[Solver] = &[part1_count_total_overlaps, part2_count_partial_overlaps];
