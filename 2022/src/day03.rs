use std::io::BufRead;

use bstr::io::BufReadExt;

use crate::Solver;

fn item_prio(letter: u8) -> u8 {
    match letter {
        b'a'..=b'z' => letter - b'a' + 1,
        b'A'..=b'Z' => letter - b'A' + 27,
        _ => unreachable!(),
    }
}

fn make_set(items: &[u8]) -> u64 {
    let mut set = 0;
    for item in items {
        set |= 1 << *item - 64
    }
    set
}

fn bit_to_item(bitset: u64) -> u8 {
    let mut bitset = bitset;
    let mut item = 0;
    while bitset > 1 {
        bitset >>= 1;
        item += 1;
    }
    item + 64
}

fn part1_item_in_both_priorities(mut input: &mut dyn BufRead) -> String {
    let mut total = 0;
    input
        .for_byte_line(|line| {
            assert!(line.len() % 2 == 0);
            let (left, right) = line.split_at(line.len() / 2);
            let left_set = make_set(left);
            let right_set = make_set(right);

            let item = bit_to_item(left_set & right_set);
            total += item_prio(item) as u64;
            Ok(true)
        })
        .unwrap();
    total.to_string()
}

fn part2_item_in_groups_of_3(input: &mut dyn BufRead) -> String {
    let mut total = 0;
    let mut collector = u64::MAX;
    for (i, line) in input.byte_lines().enumerate() {
        let line = line.unwrap();
        collector &= make_set(line.as_slice());
        if i % 3 == 2 {
            let item = bit_to_item(collector);
            total += item_prio(item) as u64;
            collector = u64::MAX;
        }
    }
    total.to_string()
}

pub const SOLVERS: &[Solver] = &[part1_item_in_both_priorities, part2_item_in_groups_of_3];
