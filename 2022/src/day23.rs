use std::io::BufRead;

use crate::{
    tiles::{Point, Tiles},
    Solver,
};

const INPUT_SIZE: usize = 100 * 100;
const EXPAND: usize = 70;

const N_SIDE: u8 = 0b1110_0000;
const W_SIDE: u8 = 0b1001_0100;
const S_SIDE: u8 = 0b0000_0111;
const E_SIDE: u8 = 0b0010_1001;

const SIDES: [u8; 4] = [N_SIDE, S_SIDE, W_SIDE, E_SIDE];
const MOVES: [Point; 4] = [
    Point::new(0, -1),
    Point::new(0, 1),
    Point::new(-1, 0),
    Point::new(1, 0),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry {
    current: u8,
    next: u8,
}

impl Entry {
    fn new(current: u8, next: u8) -> Self {
        Self { current, next }
    }
}

struct Map {
    map: Tiles<Entry>,
    min_bounds: Point,
    max_bounds: Point,
    min_bounds_moving: Point,
    max_bounds_moving: Point,
    step: usize,
}

impl Map {
    pub fn new(source: Tiles<u8>) -> Self {
        let modified = source
            .entries
            .iter()
            .map(|c| match c {
                b'#' => Entry::new(1, 0),
                _ => Entry::new(0, 0),
            })
            .collect::<Vec<_>>();
        let map = Tiles {
            entries: modified,
            entry_len: source.entry_len,
            line_width: source.line_width,
        };
        let mut bigger_map = Tiles::new(
            map.width() + EXPAND * 2,
            map.height() + EXPAND * 2,
            Entry::new(0, 0),
        );
        bigger_map.copy(EXPAND, EXPAND, &map);

        Self {
            map: bigger_map,
            min_bounds: Point::from((EXPAND, EXPAND)),
            max_bounds: Point::from((EXPAND + source.width(), EXPAND + source.height())),
            min_bounds_moving: Point::from((EXPAND, EXPAND)),
            max_bounds_moving: Point::from((EXPAND + source.width(), EXPAND + source.height())),
            step: 0,
        }
    }

    fn print(&self) {
        for y in self.min_bounds.y - 1..=self.max_bounds.y + 1 {
            for x in self.min_bounds.x - 1..=self.max_bounds.x + 1 {
                let p = Point::from((x, y));
                if self.map[p].current == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    fn round(&mut self) -> bool {
        let mut moved = false;
        let mut min_bounds_moving = Point::new(i16::MAX, i16::MAX);
        let mut max_bounds_moving = Point::new(0, 0);

        for y in self.min_bounds_moving.y - 1..=self.max_bounds_moving.y + 1 {
            for x in self.min_bounds_moving.x - 1..=self.max_bounds_moving.x + 1 {
                let cur_point = Point::from((x, y));
                if self.map[cur_point].current == 0 {
                    continue;
                }
                // find neighborhood for this point
                let neighbors: u8 =
                    (self.map[Point::new(cur_point.x - 1, cur_point.y - 1)].current << 7)
                        + (self.map[Point::new(cur_point.x, cur_point.y - 1)].current << 6)
                        + (self.map[Point::new(cur_point.x + 1, cur_point.y - 1)].current << 5)
                        + (self.map[Point::new(cur_point.x - 1, cur_point.y)].current << 4)
                        + (self.map[Point::new(cur_point.x + 1, cur_point.y)].current << 3)
                        + (self.map[Point::new(cur_point.x - 1, cur_point.y + 1)].current << 2)
                        + (self.map[Point::new(cur_point.x, cur_point.y + 1)].current << 1)
                        + self.map[Point::new(cur_point.x + 1, cur_point.y + 1)].current;
                // by default, stay in place
                self.map[cur_point].next = 1;
                // if there are no neighbors, this point does not move
                if neighbors == 0 {
                    continue;
                }
                // update bounds because this is a candidate for move
                moved = true;
                min_bounds_moving = min_bounds_moving.min_bound(cur_point);
                max_bounds_moving = max_bounds_moving.max_bound(cur_point);
                for i in 0..4 {
                    // start at the side matching the current step
                    let i = (i + self.step) % 4;
                    let side = SIDES[i];
                    if neighbors & side != 0 {
                        // side is occupied, try another
                        continue;
                    }
                    let next_point = cur_point + MOVES[i];
                    if self.map[next_point].next == 1 {
                        // if someone is already moving here, this point does not move and
                        // the other point is pushed back
                        self.map[cur_point].next = 1;
                        self.map[next_point].next = 0;
                        self.map[next_point + MOVES[i]].next = 1;
                    } else {
                        // move in this direction
                        self.map[cur_point].next = 0;
                        self.map[next_point].next = 1;
                    }
                    break;
                }
            }
        }
        self.min_bounds_moving = min_bounds_moving;
        self.max_bounds_moving = max_bounds_moving;

        for y in self.min_bounds_moving.y - 1..=self.max_bounds_moving.y + 1 {
            for x in self.min_bounds_moving.x - 1..=self.max_bounds_moving.x + 1 {
                let p = Point::from((x, y));
                self.map[p].current = self.map[p].next;
            }
        }

        self.min_bounds = self.min_bounds.min_bound(self.min_bounds_moving);
        self.max_bounds = self.max_bounds.max_bound(self.max_bounds_moving);
        self.step += 1;
        moved
    }
}

pub fn part1_ten_rounds(input: &mut dyn BufRead) -> String {
    let input_map = Tiles::load(input, INPUT_SIZE);
    let mut map = Map::new(input_map);

    for _ in 0..10 {
        map.round();
    }

    let mut spaces = 0;
    for y in map.min_bounds.y..=map.max_bounds.y {
        for x in map.min_bounds.x..=map.max_bounds.x {
            spaces += 1 - map.map[(x as usize, y as usize)].current as usize;
        }
    }

    spaces.to_string()
}

pub fn part2_move_until_done(input: &mut dyn BufRead) -> String {
    let input_map = Tiles::load(input, INPUT_SIZE);
    let mut map = Map::new(input_map);

    while map.round() {}
    map.step.to_string()
}

pub const SOLVERS: &[Solver] = &[part1_ten_rounds, part2_move_until_done];
