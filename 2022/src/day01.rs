use bstr::io::BufReadExt;
use std::io::BufRead;

use crate::{parse_num, Solver};

fn part1_find_max_joules(mut input: &mut dyn BufRead) -> String {
    let mut max_joules = 0;
    let mut current_joules = 0;
    input
        .for_byte_line(|line| {
            match line {
                b"" => {
                    max_joules = max_joules.max(current_joules);
                    current_joules = 0;
                }
                str => {
                    let num = parse_num(str);
                    current_joules += num;
                }
            }
            Ok(true)
        })
        .unwrap();
    max_joules.max(current_joules).to_string()
}

struct Top3([u64; 3]);

impl Top3 {
    pub fn new() -> Self {
        Self([0, 0, 0])
    }

    pub fn insert(&mut self, new_elf: u64) {
        let mut slider = new_elf;
        for i in 0..self.0.len() {
            let new_slider = self.0[i].min(slider);
            self.0[i] = self.0[i].max(slider);
            slider = new_slider;
        }
    }

    pub fn total(&self) -> u64 {
        self.0.iter().sum()
    }
}

fn part2_find_top_3(mut input: &mut dyn BufRead) -> String {
    let mut top3 = Top3::new();

    let mut current_elf = 0;
    input
        .for_byte_line(|line| {
            match line {
                b"" => {
                    top3.insert(current_elf);
                    current_elf = 0;
                }
                str => {
                    let num = parse_num(str);
                    current_elf += num;
                }
            }
            Ok(true)
        })
        .unwrap();
    top3.insert(current_elf);

    top3.total().to_string()
}

pub const SOLVERS: &[Solver] = &[part1_find_max_joules, part2_find_top_3];
