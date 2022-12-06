use std::io::BufRead;

use crate::Solver;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct AsciiLowerBitSet (u32);

impl AsciiLowerBitSet {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add(&mut self, c: u8) {
        //assert!(b'a' <= c && c <= b'z');
        self.0 |= 1 << (c - b'a');
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

fn find_distinct_prefix_n<const N: usize>(input: &mut dyn BufRead) -> String {
    let mut data = vec![];
    input.read_to_end(&mut data).unwrap();
    for i in N..data.len() {
        let mut set = AsciiLowerBitSet::new();
        for j in 1..=N {
            set.add(data[i - j]);
        }
        if set.len() == N {
            return i.to_string();
        }
    }
    unreachable!()
}

pub const SOLVERS: &[Solver] = &[find_distinct_prefix_n::<4>, find_distinct_prefix_n::<14>];
