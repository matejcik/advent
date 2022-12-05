use std::io::BufRead;

use bstr::io::BufReadExt;

use crate::{parse_nums, Solver};

pub const STACKS_MAX: usize = 9;

struct Stacks([Vec<u8>; STACKS_MAX]);

impl Stacks {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn feed(&mut self, line: &[u8]) {
        assert!(line.len() + 1 <= STACKS_MAX * 4);
        let stacks_on_line = (line.len() + 1) / 4;
        for i in 0..stacks_on_line {
            let char = line[i * 4 + 1];
            if char != b' ' {
                self.0[i].push(char);
            }
        }
    }

    pub fn prepare(&mut self) {
        for stack in self.0.iter_mut() {
            stack.reverse();
        }
    }

    pub fn move_items_lifo(&mut self, count: usize, from: usize, to: usize) {
        let len = self.0[from].len();
        assert!(count <= len);
        for idx in 0..count {
            self.0[to].push(self.0[from][len - idx - 1]);
        }
        self.0[from].truncate(len - count);
    }

    pub fn move_items_fifo(&mut self, count: usize, from: usize, to: usize) {
        let pivot = from.min(to) + 1;
        let (left, right) = self.0.split_at_mut(pivot);
        let (from, to) = if from < to {
            (&mut left[from], &mut right[to - pivot])
        } else {
            (&mut right[from - pivot], &mut left[to])
        };
        let iter = from.drain((from.len() - count)..);
        to.extend(iter);
    }

    pub fn tops(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().flat_map(|stack| stack.last().copied())
    }
}

fn part1_move_stacks(mut input: &mut dyn BufRead) -> String {
    let mut stacks = Stacks::new();
    input
        .for_byte_line(|line| {
            Ok(if line.is_empty() {
                false
            } else {
                stacks.feed(line);
                true
            })
        })
        .unwrap();
    stacks.prepare();

    let mut numbers = [0u64; 3];
    input
        .for_byte_line(|line| {
            parse_nums(line, &mut numbers);
            let [count, from, to] = numbers;
            stacks.move_items_lifo(count as usize, (from - 1) as usize, (to - 1) as usize);
            Ok(true)
        })
        .unwrap();

    stacks.tops().map(|c| c as char).collect()
}

fn part2_mover9001(mut input: &mut dyn BufRead) -> String {
    let mut stacks = Stacks::new();
    input
        .for_byte_line(|line| {
            Ok(if line.is_empty() {
                false
            } else {
                stacks.feed(line);
                true
            })
        })
        .unwrap();
    stacks.prepare();

    let mut numbers = [0u64; 3];
    input
        .for_byte_line(|line| {
            parse_nums(line, &mut numbers);
            let [count, from, to] = numbers;
            stacks.move_items_fifo(count as usize, (from - 1) as usize, (to - 1) as usize);
            Ok(true)
        })
        .unwrap();

    stacks.tops().map(|c| c as char).collect()
}

pub const SOLVERS: &[Solver] = &[part1_move_stacks, part2_mover9001];
