use std::io::BufRead;

use crate::{
    tiles::{Point, Tiles},
    Solver,
};

const MAP_SIZE: usize = 140 * 30;

const ELF_BIT: u8 = 0b1_0000;

fn convert_map(input: &mut dyn BufRead) -> (Tiles<u8>, Point, Point) {
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

fn printmap(map: &Tiles<u8>) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let entry = map[(x, y)];
            let ch = match entry {
                0 => '.',
                0b0001 => 'v',
                0b0010 => '>',
                0b0100 => '^',
                0b1000 => '<',
                ELF_BIT => 'E',
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

fn step(spawn: Point, map: &Tiles<u8>, newmap: &mut Tiles<u8>) {
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

    for y in 0..map.height() {
        for x in 0..map.width() {
            let p = Point::from((x, y));
            if map[p] & ELF_BIT == 0 {
                continue;
            }
            if matches!(newmap.get(p), Some(0)) {
                newmap[p] = ELF_BIT;
            }
            for dir in Point::CARDINAL_DIRECTIONS.iter() {
                let np = Point::from((x, y)) + *dir;
                if matches!(newmap.get(np), Some(0)) {
                    newmap[np] = ELF_BIT;
                }
            }
        }
    }
    if newmap[spawn] == 0 {
        newmap[spawn] = ELF_BIT;
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

fn routefinder(mut map: Tiles<u8>, mut spawn: Point, destinations: &[Point]) -> usize {
    let mut newmap = Tiles::new(map.width(), map.height(), 0);
    let mut destinations = destinations.iter().copied().peekable();

    let mut steps = 0;
    loop {
        steps += 1;
        newmap.reset(0);
        step(spawn, &map, &mut newmap);
        (map, newmap) = (newmap, map);
        // println!("step {}:", steps);
        // printmap(&map);
        let dest = destinations.peek().unwrap();
        if map[*dest] == ELF_BIT {
            let next_spawn = destinations.next().unwrap();
            // clear elves from the map
            map.entries.iter_mut().for_each(|e| *e &= !ELF_BIT);
            // count one additional step to get to the actual destination outside the map
            steps += 1;
            // if no next destination, we're done
            if destinations.peek().is_none() {
                break;
            }
            // run the step
            newmap.reset(0);
            step(next_spawn, &map, &mut newmap);
            (map, newmap) = (newmap, map);
            // clear a potential spawned elf
            map[next_spawn] &= !ELF_BIT;
            // continue up to next destination
            spawn = next_spawn;
        }
    }

    steps
}

pub const SOLVERS: &[Solver] = &[part1_quantum_elves, part2_forgetful_elves];
