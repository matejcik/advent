pub mod intcode;

use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate num_derive;

pub fn load_input(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let buf = BufReader::new(f);
    buf.lines().collect::<Result<Vec<_>, _>>().unwrap()
}
