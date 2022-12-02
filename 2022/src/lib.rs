use bstr::io::BufReadExt;
use std::io::{BufRead, Seek};

pub mod day01;
pub mod day02;

pub type Solver = fn(&mut dyn BufRead) -> String;

pub trait Resettable: BufReadExt + Seek {
    fn reset(&mut self) {
        self.seek(std::io::SeekFrom::Start(0)).unwrap();
    }
}

impl<T: BufReadExt + Seek> Resettable for T {}

pub fn parse_num(slice: &[u8]) -> u64 {
    let mut num = 0;
    for c in slice {
        num = num * 10 + (*c as u64 - '0' as u64);
    }
    num
}
