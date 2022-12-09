#![feature(portable_simd)]

use std::io::BufRead;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

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

pub fn parse_nums(slice: &[u8], result: &mut [u64]) {
    let mut it = slice.iter().copied();
    for res in result.iter_mut() {
        let mut num = 0;
        let mut have_num = false;
        loop {
            match it.next() {
                Some(c) if (b'0'..=b'9').contains(&c) => {
                    have_num = true;
                    num = num * 10 + c as u64 - '0' as u64;
                }
                Some(_) if !have_num => continue,
                _ => {
                    *res = num;
                    break;
                }
            }
        }
    }
}
