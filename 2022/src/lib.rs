#![feature(portable_simd)]
#![feature(int_roundings)]
#![feature(generic_arg_infer)]
use std::io::BufRead;

pub mod bitset;
pub mod tiles;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub type Solver = fn(&mut dyn BufRead) -> String;

pub fn parse_num(slice: &[u8]) -> u64 {
    let mut num = 0;
    for c in slice {
        match c {
            b'0'..=b'9' => num = num * 10 + (*c as u64 - '0' as u64),
            _ => break,
        }
    }
    num
}

pub fn parse_nums(slice: &[u8], result: &mut [u64]) -> usize {
    let mut it = slice.iter().copied();
    let mut nums = 0;
    'outer: for res in result.iter_mut() {
        let mut num = 0;
        let mut have_num = false;
        loop {
            match it.next() {
                Some(c) if (b'0'..=b'9').contains(&c) => {
                    have_num = true;
                    num = num * 10 + c as u64 - '0' as u64;
                    continue;
                }
                Some(_) if !have_num => continue,
                x => {
                    *res = num;
                    nums += 1;
                    if x.is_none() {
                        break 'outer;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    nums
}
