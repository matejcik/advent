use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    io::BufRead,
};

use crate::{
    tiles::{Point, Tiles},
    Solver,
};

const MAP_SIZE: usize = 140 * 30;

fn convert_map(input: &mut dyn BufRead) -> (Tiles<u8>, Point, Point) {
    let input = Tiles::load(input, MAP_SIZE);
    let start_idx = input
        .rows_steppers()
        .next()
        .unwrap()
        .iter()
        .find(|c| input.entries[*c] == b'.')
        .unwrap();
    let start = input.coords_for(start_idx) - Point::new(1, 1);
    let end_idx = input
        .rows_steppers()
        .last()
        .unwrap()
        .iter()
        .find(|c| input.entries[*c] == b'.')
        .unwrap();
    let end = input.coords_for(end_idx) - Point::new(1, 1);
    let entries = input
        .entries
        .iter()
        .map(|e| match e {
            b'v' => 0b0001,
            b'>' => 0b0010,
            b'^' => 0b0100,
            b'<' => 0b1000,
            _ => 0,
        })
        .collect();

    let input_converted = Tiles::from_vec(entries, input.entry_len, 1);
    let size = input_converted.size();
    let map = input_converted.region(1, 1, size.x as usize - 1, size.y as usize - 1);

    (map, start, end)
}

fn printmap(map: &Tiles<u8>, elves: &HashSet<Point>) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let entry = map[(x, y)];
            let ch = match entry {
                0 => {
                    if elves.contains(&Point::from((x, y))) {
                        'E'
                    } else {
                        '.'
                    }
                }
                0b0001 => 'v',
                0b0010 => '>',
                0b0100 => '^',
                0b1000 => '<',
                x if x.count_ones() == 2 => '2',
                x if x.count_ones() == 3 => '3',
                x if x.count_ones() == 4 => '4',
                _ => '?',
            };
            print!("{}", ch);
        }
        println!();
    }
    println!("---");
}

fn step(map: &Tiles<u8>, newmap: &mut Tiles<u8>) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let p = Point::from((x, y));
            for (i, dir) in Point::CARDINAL_DIRECTIONS.iter().enumerate() {
                let mask = 1 << i;
                if map[p] & mask != 0 {
                    let np = (Point::from((x, y)) + *dir) % map.size();
                    newmap[np] |= mask;
                }
            }
        }
    }
}

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

fn routefinder(mut map: Tiles<u8>, mut start: Point, destinations: &[Point]) -> usize {
    let mut newmap = Tiles::new(map.width(), map.height(), 0);
    let mut elves = HashSet::with_capacity(1000);
    let mut new_elves = HashSet::with_capacity(1000);
    elves.insert(start);

    let mut destinations = destinations.iter().copied().peekable();

    let mut steps = 0;
    'outer: loop {
        steps += 1;
        newmap.reset(0);
        step(&map, &mut newmap);
        (map, newmap) = (newmap, map);
        'inner: for elf in elves.iter() {
            if *elf == start
                || elf == destinations.peek().unwrap()
                || matches!(map.get(*elf), Some(0))
            {
                new_elves.insert(*elf);
            }
            for n in elf.neighbors() {
                if n == *destinations.peek().unwrap() {
                    destinations.next().unwrap();
                    if destinations.peek().is_none() {
                        break 'outer;
                    } else {
                        new_elves.clear();
                        new_elves.insert(n);
                        start = n;
                        println!("reached destination");    
                        break 'inner;
                    }
                } else if n == start || matches!(map.get(n), Some(0)) {
                    new_elves.insert(n);
                }
            }
        }
        // println!("step {}:", steps);
        // printmap(&map, &new_elves);
        if new_elves.is_empty() {
            panic!("no elves left :(")
        }

        elves.clear();
        (elves, new_elves) = (new_elves, elves);
    }

    steps
}

pub const SOLVERS: &[Solver] = &[part1_quantum_elves, part2_forgetful_elves];
