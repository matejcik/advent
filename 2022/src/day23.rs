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

struct Map {
    map: Tiles<u8>,
    proposals: Tiles<i8>,
    min_bounds: Point,
    max_bounds: Point,
    min_bounds_moving: Point,
    max_bounds_moving: Point,
    step: usize,
}

impl Map {
    pub fn new(mut source: Tiles<u8>) -> Self {
        source.entries.iter_mut().for_each(|c| match c {
            b'#' => *c = 1,
            _ => *c = 0,
        });
        let mut bigger_map =
            Tiles::new(source.width() + EXPAND * 2, source.height() + EXPAND * 2, 0);
        bigger_map.copy(EXPAND, EXPAND, &source);
        let proposals = Tiles::new(bigger_map.width(), bigger_map.height(), -1);

        Self {
            map: bigger_map,
            proposals,
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
                if self.map[p] == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    fn round(&mut self) -> bool {
        for y in self.min_bounds_moving.y - 1..=self.max_bounds_moving.y + 1 {
            for x in self.min_bounds_moving.x - 1..=self.max_bounds_moving.x + 1 {
                let p = Point::from((x, y));
                if self.map[p] == 0 {
                    continue;
                }
                // find neighborhood for this point
                let neighbors: u8 = (self.map[Point::new(p.x - 1, p.y - 1)] << 7)
                    + (self.map[Point::new(p.x, p.y - 1)] << 6)
                    + (self.map[Point::new(p.x + 1, p.y - 1)] << 5)
                    + (self.map[Point::new(p.x - 1, p.y)] << 4)
                    + (self.map[Point::new(p.x + 1, p.y)] << 3)
                    + (self.map[Point::new(p.x - 1, p.y + 1)] << 2)
                    + (self.map[Point::new(p.x, p.y + 1)] << 1)
                    + self.map[Point::new(p.x + 1, p.y + 1)];
                // if there are no neighbors, this point does not move
                if neighbors == 0 {
                    continue;
                }
                for i in 0..4 {
                    // start at the side matching the current step
                    let i = (i + self.step) % 4;
                    let side = SIDES[i];
                    if neighbors & side != 0 {
                        // side is occupied, try another
                        continue;
                    }
                    let np = p + MOVES[i];
                    // propose moving in this direction
                    // if someone already proposed, the move is cancelled
                    // (use 4 instead of 0 so that if multiple points propose the same move,
                    // all are in conflict)
                    self.proposals[np] = if self.proposals[np] > -1 { 4 } else { i as i8 };
                    break;
                }
            }
        }
        let mut moved = false;
        let mut min_bounds_moving = Point::new(i16::MAX, i16::MAX);
        let mut max_bounds_moving = Point::new(0, 0);
        for y in self.min_bounds_moving.y - 2..=self.max_bounds_moving.y + 2 {
            for x in self.min_bounds_moving.x - 2..=self.max_bounds_moving.x + 2 {
                let p = Point::from((x, y));
                let prop_val = self.proposals[p];
                if prop_val < 0 {
                    // no proposal
                    continue;
                }
                self.proposals[p] = -1;
                if prop_val >= 4 {
                    // move cancelled
                    continue;
                }
                // move here from the proposed direction
                let np = p + MOVES[prop_val as usize] * -1;
                self.map[p] = 1;
                self.map[np] = 0;
                moved = true;
                min_bounds_moving = min_bounds_moving.min_bound(p);
                max_bounds_moving = max_bounds_moving.max_bound(p);
            }
        }
        self.min_bounds_moving = min_bounds_moving;
        self.max_bounds_moving = max_bounds_moving;
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
            spaces += 1 - map.map[(x as usize, y as usize)] as usize;
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
