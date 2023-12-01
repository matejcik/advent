use std::io::BufRead;

use bit_set::BitSet;
use bit_vec::BitVec;

use crate::{
    tiles::{Point, Tiles},
    Solver,
};

struct Blizzards {
    width: usize,
    height: usize,
    elves: Vec<BitVec>,
    blizzards_up: Vec<BitVec>,
    blizzards_down: Vec<BitVec>,
    blizzards_left: Vec<BitVec>,
    blizzards_right: Vec<BitVec>,
    steps: usize,
}

impl Blizzards {
    pub fn has_elf(&self, p: Point) -> bool {
        self.elves
            .get(p.y as usize)
            .map(|v| v.get(p.x as usize).unwrap_or(false))
            .unwrap_or(false)
    }

    pub fn new(input: &Tiles<u8>) -> Self {
        let width = input.width() - 2;
        let height = input.height() - 2;
        let elves = vec![BitVec::with_capacity(width); height];
        let new_elves = elves.clone();
        let mut blizzards_up = vec![BitVec::with_capacity(height); width];
        let mut blizzards_down = vec![BitVec::with_capacity(height); width];
        let mut blizzards_left = vec![BitVec::with_capacity(width); height];
        let mut blizzards_right = vec![BitVec::with_capacity(width); height];
        for y in 1..input.height() - 1 {
            for x in 1..input.width() - 1 {
                match input[(x, y)] {
                    b'>' => blizzards_right[y - 1].set(x - 1, true),
                    b'<' => blizzards_left[y - 1].set(x - 1, true),
                    b'^' => blizzards_up[x - 1].set(y - 1, true),
                    b'v' => blizzards_down[x - 1].set(y - 1, true),
                    _ => (),
                };
            }
        }
        Self {
            elves,
            width,
            height,
            blizzards_up,
            blizzards_down,
            blizzards_left,
            blizzards_right,
            steps: 0,
        }
    }

    pub fn step(&mut self, spawn: Point) {
        new_elves = vec![BitVec::with_capacity(self.width); self.height];
        self.steps += 1;
        for (idx, _) in self.elves.entries.iter().enumerate().filter(|(_, &v)| v) {
            let p = self.elves.coords_for(idx);
            if !self.has_blizzard(p) {
                self.new_elves[p] = true;
            }
            for np in p.neighbors() {
                if !self.has_blizzard(np) {
                    self.new_elves[np] = true;
                }
            }
        }
        if !self.has_blizzard(spawn) {
            self.new_elves[spawn] = true;
        }
        std::mem::swap(&mut self.elves, &mut self.new_elves);
    }

    pub fn clear_elves(&mut self) {
        self.elves.entries.fill(false);
    }

    #[allow(unused)]
    pub fn print(&self) {
        for y in 0..self.elves.height() {
            for x in 0..self.elves.width() {
                let p = Point::from((x, y));
                let bits = self.blizzard_bits(p);
                let ch = if self.has_elf(p) {
                    'E'
                } else {
                    match bits {
                        0 => '.',
                        0b1000 => '^',
                        0b0100 => 'v',
                        0b0010 => '<',
                        0b0001 => '>',
                        x if x.count_ones() == 2 => '2',
                        x if x.count_ones() == 3 => '3',
                        x if x.count_ones() == 4 => '4',
                        _ => '?',
                    }
                };
                print!("{}", ch);
            }
            println!();
        }
        println!("---");
    }
}

const MAP_SIZE: usize = 140 * 30;

fn convert_map(input: &mut dyn BufRead) -> (Blizzards, Point, Point) {
    let input = Tiles::load(input, MAP_SIZE);
    let start_idx = input
        .rows_steppers()
        .next()
        .unwrap()
        .iter()
        .find(|c| input.entries[*c] == b'.')
        .unwrap();
    let start = input.coords_for(start_idx) - Point::new(1, 0);
    let end_idx = input
        .rows_steppers()
        .last()
        .unwrap()
        .iter()
        .find(|c| input.entries[*c] == b'.')
        .unwrap();
    let end = input.coords_for(end_idx) - Point::new(1, 2);

    let map = Blizzards::new(&input);

    (map, start, end)
}

// fn step(spawn: Point, map: &Tiles<u8>, newmap: &mut Tiles<u8>) {
//     for y in 0..map.height() {
//         for x in 0..map.width() {
//             let p = Point::from((x, y));
//             for (i, dir) in Point::CARDINAL_DIRECTIONS.iter().enumerate() {
//                 let mask = 1 << i;
//                 if map[p] & mask != 0 {
//                     let np = (Point::from((x, y)) + *dir) % map.size();
//                     newmap[np] |= mask;
//                 }
//             }
//         }
//     }

//     for y in 0..map.height() {
//         for x in 0..map.width() {
//             let p = Point::from((x, y));
//             if map[p] & ELF_BIT == 0 {
//                 continue;
//             }
//             if matches!(newmap.get(p), Some(0)) {
//                 newmap[p] = ELF_BIT;
//             }
//             for dir in Point::CARDINAL_DIRECTIONS.iter() {
//                 let np = Point::from((x, y)) + *dir;
//                 if matches!(newmap.get(np), Some(0)) {
//                     newmap[np] = ELF_BIT;
//                 }
//             }
//         }
//     }
//     if newmap[spawn] == 0 {
//         newmap[spawn] = ELF_BIT;
//     }
// }

fn part1_quantum_elves(input: &mut dyn BufRead) -> String {
    let (map, start, end) = convert_map(input);
    let destinations = &[end];
    routefinder(map, start, destinations).to_string()
}

fn part2_forgetful_elves(input: &mut dyn BufRead) -> String {
    let (map, start, end) = convert_map(input);
    let destinations = &[end, start, end];
    routefinder(map, start, destinations).to_string()
}

fn routefinder(mut map: Blizzards, mut spawn: Point, destinations: &[Point]) -> usize {
    let mut destinations = destinations.iter().copied().peekable();

    loop {
        map.step(spawn);
        // println!("after step {}:", map.steps);
        // map.print();
        let dest = destinations.peek().unwrap();
        if map.has_elf(*dest) {
            spawn = destinations.next().unwrap();
            // clear elves from the map
            map.clear_elves();
            // count one additional step to get to the actual destination outside the map
            map.step(Point::new(-5, -5));
            // if no next destination, we're done
            if destinations.peek().is_none() {
                break;
            }
        }
    }

    map.steps
}

pub const SOLVERS: &[Solver] = &[part1_quantum_elves, part2_forgetful_elves];
