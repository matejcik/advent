use std::io::{BufRead, Seek};
use std::time::Instant;
use std::{fs::File, io::BufReader};

use clap::Parser;

use advent2022::{day01, day02, day03, Solver};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    suffix: Option<String>,

    #[arg()]
    days: Vec<u8>,
}

fn run_solver(solver: Solver, heading: String, input: &mut dyn BufRead) -> u128 {
    let start = Instant::now();
    let result = solver(input);
    let elapsed = start.elapsed().as_micros();
    println!("{}: {}", heading, result);
    println!("Elapsed: {} us", elapsed);
    elapsed
}

fn main() {
    let args = Args::parse();

    let mut total_runtime = 0;

    let days = if args.days.is_empty() {
        (1..=DAY_MAX).collect()
    } else {
        args.days
    };

    for day in days {
        let suffix = if let Some(str) = &args.suffix {
            format!("-{}", str)
        } else {
            String::from("")
        };
        let input_filename = format!("input/{:02}{}.txt", day, suffix);
        let mut input = BufReader::new(File::open(input_filename).unwrap());
        input.fill_buf().expect("failed to fill buffer");

        let solvers = get_day(day);
        for (i, func) in solvers.iter().enumerate() {
            total_runtime += run_solver(*func, format!("Day {} part {}", day, i + 1), &mut input);
            input.seek(std::io::SeekFrom::Start(0)).unwrap();
        }
    }

    println!();
    println!("Total runtime: {} us", total_runtime);
}

const DAY_MAX: u8 = 2;

fn get_day(day: u8) -> &'static [Solver] {
    match day {
        1 => day01::SOLVERS,
        2 => day02::SOLVERS,
        3 => day03::SOLVERS,
        _ => panic!("Day {} not implemented", day),
    }
}
