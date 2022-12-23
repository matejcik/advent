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

fn round(map: &mut Tiles<u8>, proposals: &mut Tiles<i8>, step: usize) -> bool {
    for (idx, _) in map.entries.iter().enumerate().filter(|(_, &c)| c == 1) {
        let p = map.coords_for(idx);
        assert!(p.x > 0 && p.y > 0);
        // find neighborhood for this point
        let neighbors: u8 = (map[Point::new(p.x - 1, p.y - 1)] << 7)
            + (map[Point::new(p.x, p.y - 1)] << 6)
            + (map[Point::new(p.x + 1, p.y - 1)] << 5)
            + (map[Point::new(p.x - 1, p.y)] << 4)
            + (map[Point::new(p.x + 1, p.y)] << 3)
            + (map[Point::new(p.x - 1, p.y + 1)] << 2)
            + (map[Point::new(p.x, p.y + 1)] << 1)
            + map[Point::new(p.x + 1, p.y + 1)];
        // if there are no neighbors, this point does not move
        if neighbors == 0 {
            continue;
        }
        for i in 0..4 {
            // start at the side matching the current step
            let i = (i + step) % 4;
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
            proposals[np] = if proposals[np] > -1 { 4 } else { i as i8 };
            break;
        }
    }
    let mut moved = false;
    for (idx, prop) in proposals
        .entries
        .iter_mut()
        .enumerate()
        .filter(|(_, c)| **c > -1)
    {
        let prop_val = *prop as usize;
        *prop = -1;
        let p = map.coords_for(idx);
        if prop_val >= 4 {
            // move cancelled
            continue;
        }
        // move here from the proposed direction
        let np = p + MOVES[prop_val] * -1;
        map[p] = 1;
        map[np] = 0;
        moved = true;
    }
    moved
}

pub fn part1_ten_rounds(input: &mut dyn BufRead) -> String {
    let mut map = Tiles::load(input, INPUT_SIZE);
    map.entries.iter_mut().for_each(|c| match c {
        b'#' => *c = 1,
        _ => *c = 0,
    });
    let mut bigger_map = Tiles::new(map.width() + EXPAND * 2, map.height() + EXPAND * 2, 0);
    bigger_map.copy(EXPAND, EXPAND, &map);
    let mut proposals = Tiles::new(bigger_map.width(), bigger_map.height(), -1);
    assert_eq!(bigger_map.entries.len(), proposals.entries.len());

    for i in 0..10 {
        round(&mut bigger_map, &mut proposals, i);
    }

    let mut min_bound = Point::new(i16::MAX, i16::MAX);
    let mut max_bound = Point::new(0, 0);
    for (p, &c) in bigger_map.entries.iter().enumerate() {
        if c == 1 {
            let p = bigger_map.coords_for(p);
            min_bound = min_bound.min_bound(p);
            max_bound = max_bound.max_bound(p);
        }
    }

    let mut spaces = 0;
    for y in min_bound.y..=max_bound.y {
        for x in min_bound.x..=max_bound.x {
            spaces += 1 - bigger_map[(x as usize, y as usize)] as usize;
        }
    }

    spaces.to_string()
}

pub fn part2_move_until_done(input: &mut dyn BufRead) -> String {
    let mut map = Tiles::load(input, INPUT_SIZE);
    map.entries.iter_mut().for_each(|c| match c {
        b'#' => *c = 1,
        _ => *c = 0,
    });
    let mut bigger_map = Tiles::new(map.width() + EXPAND * 2, map.height() + EXPAND * 2, 0);
    bigger_map.copy(EXPAND, EXPAND, &map);
    let mut proposals = Tiles::new(bigger_map.width(), bigger_map.height(), -1);

    let mut counter = 0;
    loop {
        if !round(&mut bigger_map, &mut proposals, counter) {
            break;
        } else {
            counter += 1;
        }
    }
    (counter + 1).to_string()
}

pub const SOLVERS: &[Solver] = &[part1_ten_rounds, part2_move_until_done];
