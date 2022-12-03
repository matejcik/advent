use std::io::BufRead;

pub mod day01;
pub mod day02;
pub mod day03;

pub type Solver = fn(&mut dyn BufRead) -> u64;

pub fn parse_num(slice: &[u8]) -> u64 {
    let mut num = 0;
    for c in slice {
        num = num * 10 + (*c as u64 - '0' as u64);
    }
    num
}
