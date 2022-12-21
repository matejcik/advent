use std::io::{BufRead, Seek};
use std::time::Instant;
use std::{fs::File, io::BufReader};

use clap::Parser;

use advent2022::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, Solver, day21,
};
use prettytable::row;

const BENCH_TRIES_DEFAULT: u128 = 500;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    suffix: Option<String>,

    #[arg(short, long)]
    tries: Option<u128>,

    #[arg()]
    days: Vec<u8>,
}

fn run_solver(solver: Solver, input: &mut (impl BufRead + Seek), tries: u128) -> (String, f64) {
    let start = Instant::now();
    let mut result = None;
    for _ in 0..tries {
        input.seek(std::io::SeekFrom::Start(0)).unwrap();
        match result.as_ref() {
            Some(result) => {
                if *result != solver(input) {
                    panic!("Solver returned different results");
                }
            }
            None => result = Some(solver(input)),
        }
    }
    let elapsed = start.elapsed().as_nanos() as f64 / tries as f64;
    (result.unwrap(), elapsed)
}

fn main() {
    let args = Args::parse();

    let mut total_runtime = 0f64;

    let all_days = args.days.is_empty();

    let days = if all_days {
        (1..=DAY_MAX).collect()
    } else {
        args.days
    };

    let tries = if let Some(tries) = args.tries {
        tries
    } else if all_days {
        BENCH_TRIES_DEFAULT
    } else {
        1
    };

    let mut table = prettytable::Table::new();
    table.add_row(row!["Day", "Part 1", "elapsed", "Part 2", "elapsed"]);

    for day in days {
        let suffix = if let Some(str) = &args.suffix {
            format!("-{}", str)
        } else {
            String::from("")
        };
        let input_filename = format!("input/{:02}{}.txt", day, suffix);
        let mut input = BufReader::new(File::open(input_filename).unwrap());
        input.fill_buf().expect("failed to fill buffer");

        let mut row_vec = vec![day.to_string()];
        let solvers = get_day(day);
        for func in solvers {
            let (result, elapsed) = run_solver(*func, &mut input, tries);
            row_vec.push(result.to_string());
            row_vec.push(format!("{:.02} µs", elapsed / 1000f64));
            total_runtime += elapsed;
        }
        table.add_row(row_vec.into());
    }

    table.printstd();
    println!();
    println!("Total runtime: {:.02} µs", total_runtime / 1000f64);
}

const DAY_MAX: u8 = 21;

fn get_day(day: u8) -> &'static [Solver] {
    match day {
        1 => day01::SOLVERS,
        2 => day02::SOLVERS,
        3 => day03::SOLVERS,
        4 => day04::SOLVERS,
        5 => day05::SOLVERS,
        6 => day06::SOLVERS,
        7 => day07::SOLVERS,
        8 => day08::SOLVERS,
        9 => day09::SOLVERS,
        10 => day10::SOLVERS,
        11 => day11::SOLVERS,
        12 => day12::SOLVERS,
        13 => day13::SOLVERS,
        14 => day14::SOLVERS,
        15 => day15::SOLVERS,
        16 => day16::SOLVERS,
        17 => day17::SOLVERS,
        18 => day18::SOLVERS,
        19 => day19::SOLVERS,
        20 => day20::SOLVERS,
        21 => day21::SOLVERS,
        _ => panic!("Day {} not implemented", day),
    }
}
