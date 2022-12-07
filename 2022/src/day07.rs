use std::io::BufRead;

use bstr::io::BufReadExt;

use crate::{parse_num, Solver};

const MAX_DIRS: usize = 1000;

const DIRSIZE_LIMIT: u64 = 100_000;

const DISK_SPACE: u64 = 70_000_000;
const SPACE_REQUIRED: u64 = 30_000_000;

fn dirscan(input: &mut dyn BufRead) -> Vec<u64> {
    let mut dirsizes = Vec::with_capacity(MAX_DIRS);
    let mut stack = Vec::with_capacity(100);
    let mut top = 0;
    for line in input.byte_lines().flatten() {
        match &line[..3] {
            // $ cd .. -> un-nest
            b"$ c" if &line == b"$ cd .." => {
                dirsizes.push(top);
                top += stack.pop().unwrap();
            }
            b"$ c" => {
                stack.push(top);
                top = 0;
            }
            b"$ l" => {} // $ ls -> ignore
            b"dir" => {} // ls entry: dir [something] -> ignore
            _ => top += parse_num(&line),
        }
    }
    while let Some(stack_top) = stack.pop() {
        dirsizes.push(top);
        top += stack_top;
    }
    dirsizes
}

fn part1_dirscan(input: &mut dyn BufRead) -> String {
    let dirsizes = dirscan(input);
    let mut total = 0;
    for size in dirsizes {
        if size <= DIRSIZE_LIMIT {
            total += size;
        }
    }
    total.to_string()
}

fn part2_identify_dir_to_delete(input: &mut dyn BufRead) -> String {
    let dirsizes = dirscan(input);
    let total = dirsizes.last().unwrap(); // size of the '/' directory

    let space_left = DISK_SPACE - total;
    let space_required = SPACE_REQUIRED - space_left;

    let mut best_dirsize = u64::MAX;
    for size in dirsizes {
        if size >= space_required {
            best_dirsize = best_dirsize.min(size);
        }
    }
    best_dirsize.to_string()
}

pub const SOLVERS: &[Solver] = &[part1_dirscan, part2_identify_dir_to_delete];
