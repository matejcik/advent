use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

use bstr::io::BufReadExt;
use itertools::Itertools;

use crate::tiles::Point;
use crate::{parse_nums, Solver};

const EXPECTED_DEPTH: usize = 250;
const EXPECTED_WIDTH: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SandDir {
    Down,
    Left,
    Right,
}

impl SandDir {
    fn fall(&self, pos: Point) -> Point {
        pos + match self {
            SandDir::Down => Point::new(0, 1),
            SandDir::Left => Point::new(-1, 1),
            SandDir::Right => Point::new(1, 1),
        }
    }

    fn next(&self) -> Option<Self> {
        match self {
            SandDir::Down => Some(SandDir::Left),
            SandDir::Left => Some(SandDir::Right),
            SandDir::Right => None,
        }
    }
}

fn add_pixels(set: &mut HashSet<Point>, rock_line: &[u8]) -> i16 {
    let mut numbers = [0; 64];
    let n = parse_nums(rock_line, &mut numbers);
    let mut deepest_y = 0;

    for (l, r) in numbers[..n]
        .iter()
        .copied()
        .tuples::<(u64, u64)>()
        .map(|t| Point::from(t))
        .tuple_windows()
    {
        if l.x == r.x {
            let (miny, maxy) = if l.y < r.y { (l.y, r.y) } else { (r.y, l.y) };
            for y in miny..=maxy {
                set.insert(Point::from((l.x, y)));
            }
        } else {
            let (minx, maxx) = if l.x < r.x { (l.x, r.x) } else { (r.x, l.x) };
            for x in minx..=maxx {
                set.insert(Point::from((x, l.y)));
            }
        }
        deepest_y = deepest_y.max(l.y.max(r.y));
    }
    deepest_y
}

fn part1_trace_sand(mut input: &mut dyn BufRead) -> String {
    let mut pixels = HashSet::with_capacity(EXPECTED_DEPTH * EXPECTED_WIDTH / 2);
    let mut deepest_y = 0;
    input.for_byte_line(|line| {
        deepest_y = deepest_y.max(add_pixels(&mut pixels, line));
        Ok(true)
    }).unwrap();

    let mut sand_stack = Vec::with_capacity(deepest_y as usize);
    let mut cur_pos = Point::from((500, 0));
    let mut cur_dir = SandDir::Down;

    let mut settled_grains = 0u32;

    loop {
        let next = cur_dir.fall(cur_pos);
        if !pixels.contains(&next) {
            if next.y > deepest_y {
                break;
            }
            sand_stack.push((cur_pos, cur_dir));
            cur_pos = next;
            cur_dir = SandDir::Down;
            continue;
        }
        // now `next` is trying to fall on an occupied pixel
        if let Some(next_dir) = cur_dir.next() {
            cur_dir = next_dir;
            continue;
        }
        // now there are no more options where to fall -> grain is settling
        settled_grains += 1;
        pixels.insert(cur_pos);
        let (next_pos, next_dir) = sand_stack.pop().unwrap();
        cur_pos = next_pos;
        cur_dir = next_dir;
    }

    settled_grains.to_string()
}

fn part2_bfs_fill(mut input: &mut dyn BufRead) -> String {
    let mut pixels = HashSet::with_capacity(EXPECTED_DEPTH * EXPECTED_WIDTH / 2);
    let mut deepest_y = 0;
    input.for_byte_line(|line| {
        deepest_y = deepest_y.max(add_pixels(&mut pixels, line));
        Ok(true)
    }).unwrap();

    let limit = deepest_y + 1;
    let mut queue = VecDeque::with_capacity(deepest_y as usize * 2);
    queue.push_back(Point::new(500, 0));
    let mut settled_grains = 1u32;

    while let Some(cur_pos) = queue.pop_front() {
        if cur_pos.y >= limit {
            continue;
        }
        for dir in [SandDir::Down, SandDir::Left, SandDir::Right] {
            let next = dir.fall(cur_pos);
            if !pixels.contains(&next) {
                queue.push_back(next);
                pixels.insert(next);
                settled_grains += 1;
            }
        }
    }
    settled_grains.to_string()
}


pub const SOLVERS: &[Solver] = &[part1_trace_sand, part2_bfs_fill];
