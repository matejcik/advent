use std::{fs::File, io::BufReader};
use std::time::Instant;

use clap::Parser;

use advent2022::day01;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    suffix: Option<String>,

    #[arg()]
    days: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut input = BufReader::new(File::open("inputs/01.txt").unwrap());
    let now = Instant::now();
    let (part1, part2) = day01::solve(&mut input);
    let elapsed_us = now.elapsed().as_micros();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {} us", elapsed_us);
}
