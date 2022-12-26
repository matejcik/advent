use std::io::BufRead;

use bit_set::BitSet;

use crate::{
    tiles::{Point, Tiles},
    Solver,
};

struct Blizzards {
    elves: Tiles<bool>,
    new_elves: Tiles<bool>,
    blizzards_up: Vec<BitSet>,
    blizzards_down: Vec<BitSet>,
    blizzards_left: Vec<BitSet>,
    blizzards_right: Vec<BitSet>,
    steps: usize,
}

impl Blizzards {
    pub fn blizzard_bits(&self, p: Point) -> u8 {
        if !self.elves.contains(p) {
            return 0b10000;
        }
        let x = p.x as usize;
        let y = p.y as usize;
        let down = self.blizzards_down[x].contains(
            (y as isize - self.steps as isize).rem_euclid(self.elves.height() as isize) as usize,
        );
        let up = self.blizzards_up[x].contains((y + self.steps) % self.elves.height());
        let right = self.blizzards_right[y].contains(
            (x as isize - self.steps as isize).rem_euclid(self.elves.width() as isize) as usize,
        );
        let left = self.blizzards_left[y].contains((x + self.steps) % self.elves.width());
        (up as u8) << 3 | (down as u8) << 2 | (left as u8) << 1 | right as u8
    }

    pub fn has_blizzard(&self, p: Point) -> bool {
        self.blizzard_bits(p) != 0
    }

    pub fn has_elf(&self, p: Point) -> bool {
        self.elves.get(p).copied().unwrap_or(false)
    }

    pub fn new(input: &Tiles<u8>) -> Self {
        let width = input.width() - 2;
        let height = input.height() - 2;
        let elves = Tiles::new(width, height, false);
        let new_elves = elves.clone();
        let mut blizzards_up = vec![BitSet::with_capacity(height); width];
        let mut blizzards_down = vec![BitSet::with_capacity(height); width];
        let mut blizzards_left = vec![BitSet::with_capacity(width); height];
        let mut blizzards_right = vec![BitSet::with_capacity(width); height];
        for y in 1..input.height() - 1 {
            for x in 1..input.width() - 1 {
                match input[(x, y)] {
                    b'>' => blizzards_right[y - 1].insert(x - 1),
                    b'<' => blizzards_left[y - 1].insert(x - 1),
                    b'^' => blizzards_up[x - 1].insert(y - 1),
                    b'v' => blizzards_down[x - 1].insert(y - 1),
                    _ => false,
                };
            }
        }
        Self {
            elves,
            new_elves,
            blizzards_up,
            blizzards_down,
            blizzards_left,
            blizzards_right,
            steps: 0,
        }
    }

    pub fn step(&mut self, spawn: Point) {
        self.new_elves.entries.fill(false);
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
