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

    pub fn contains(&self, c: u8) -> bool {
        //assert!(b'a' <= c && c <= b'z');
        self.0 & (1 << (c - b'a')) != 0
    }

    pub fn remove(&mut self, c: u8) {
        //assert!(b'a' <= c && c <= b'z');
        self.0 &= !(1 << (c - b'a'));
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

fn find_distinct_prefix_n<const N: usize>(input: &mut dyn BufRead) -> String {
    let mut data = vec![];
    input.read_to_end(&mut data).unwrap();
    let mut window_start = 0;
    let mut set = AsciiLowerBitSet::new();
    for i in 0..data.len() {
        if !set.contains(data[i]) {
            set.add(data[i])
        } else {
            while data[window_start] != data[i] {
                set.remove(data[window_start]);
                window_start += 1;
            }
            window_start += 1;
        }
        if i - window_start + 1 == N {
            return i.to_string()
        }
    }
    unreachable!()
}

pub const SOLVERS: &[Solver] = &[find_distinct_prefix_n::<4>, find_distinct_prefix_n::<14>];
