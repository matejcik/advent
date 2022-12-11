use std::{collections::HashSet, io::BufRead};

use bstr::io::BufReadExt;

use crate::{parse_num, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Knot(i16, i16);

impl Knot {
    pub fn move_towards(&mut self, other: &Knot) -> bool {
        // assume that other is at most 2 steps in one direction and at most one in the other direction
        let Knot(x1, y1) = self;
        let Knot(x2, y2) = other;
        if (*x1 - *x2).abs() >= 2 || (*y1 - *y2).abs() >= 2 {
            *x1 += (*x2).cmp(x1) as i16;
            *y1 += (*y2).cmp(y1) as i16;
            true
        } else {
            false
        }
    }

    pub fn move_dir(&mut self, dir: u8) {
        let Knot(x, y) = self;
        match dir {
            b'U' => *y -= 1,
            b'D' => *y += 1,
            b'R' => *x += 1,
            b'L' => *x -= 1,
            _ => panic!("invalid direction"),
        }
    }
}

fn part1_tail_positions(mut input: &mut dyn BufRead) -> String {
    let mut positions = HashSet::with_capacity(8000);
    let mut head = Knot(0, 0);
    let mut tail = Knot(0, 0);
    input
        .for_byte_line(|line| {
            let dir = line[0];
            let num = parse_num(&line[2..]);
            for _ in 0..num {
                head.move_dir(dir);
                tail.move_towards(&head);
                positions.insert(tail);
            }
            Ok(true)
        })
        .unwrap();

    positions.len().to_string()
}

fn part2_long_tail(mut input: &mut dyn BufRead) -> String {
    const KNOTS: usize = 10;
    let mut positions = HashSet::with_capacity(8000);
    let mut knots = [Knot(0, 0); KNOTS];
    input
        .for_byte_line(|line| {
            let dir = line[0];
            let num = parse_num(&line[2..]);
            for _ in 0..num {
                knots[0].move_dir(dir);
                for i in 1..KNOTS {
                    let (head, tail) = knots.split_at_mut(i);
                    if !tail[0].move_towards(&head[i - 1]) {
                        break;
                    }
                }
                positions.insert(knots[KNOTS - 1]);
            }
            Ok(true)
        })
        .unwrap();

    positions.len().to_string()
}

pub const SOLVERS: &[Solver] = &[part1_tail_positions, part2_long_tail];
