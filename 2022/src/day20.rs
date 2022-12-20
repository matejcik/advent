use std::{collections::VecDeque, io::BufRead};

use bstr::io::BufReadExt;

use crate::{parse_num, Solver};

fn parse_num_negative(line: &[u8]) -> i64 {
    if line[0] == b'-' {
        -(parse_num(&line[1..]) as i64)
    } else {
        parse_num(line) as i64
    }
}

fn do_mix(data: &mut VecDeque<(usize, i64)>, start_finger: usize) -> usize {
    let len = data.len();
    let mut finger = start_finger;
    let mut cur = 0;
    while cur < len {
        while data[finger].0 != cur {
            finger = (finger + 1) % len
        }
        let (idx, n) = data[finger];
        assert_eq!(idx, cur);
        let dest_index = (finger as i64 + n).rem_euclid(len as i64 - 1) as usize;
        data.remove(finger);
        data.insert(dest_index, (idx, n));
        cur += 1;
    }
    finger
}

fn part1_mix_once(input: &mut dyn BufRead) -> String {
    let mut data = input
        .byte_lines()
        .flatten()
        .map(|line| parse_num_negative(&line) as i64)
        .enumerate()
        .collect::<VecDeque<_>>();

    do_mix(&mut data, 0);

    let zero_idx = data.iter().position(|(_, n)| *n == 0).unwrap();
    data.iter()
        .map(|(_, n)| *n)
        .cycle()
        .skip(zero_idx)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i64>()
        .to_string()
}

fn part2_mix_ten_times(input: &mut dyn BufRead) -> String {
    const DECRYPTION_KEY: i64 = 811589153;
    let mut data = input
        .byte_lines()
        .flatten()
        .map(|line| parse_num_negative(&line) as i64 * DECRYPTION_KEY)
        .enumerate()
        .collect::<VecDeque<_>>();

    let mut finger = 0;
    for _ in 0..10 {
        finger = do_mix(&mut data, finger);
    }

    let zero_idx = data.iter().position(|(_, n)| *n == 0).unwrap();
    data.iter()
        .map(|(_, n)| *n)
        .cycle()
        .skip(zero_idx)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i64>()
        .to_string()
}

pub const SOLVERS: &[Solver] = &[part1_mix_once, part2_mix_ten_times];
