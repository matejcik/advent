use advent2019;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
enum Axis { X, Y }

#[derive(Debug)]
struct Segment {
    axis: Axis,
    x: i32, y: i32,
    step: i32,
    distance: u32
}

fn step_ofs(seg: &Segment) -> (i32, i32) {
    let sgn = if seg.step > 0 { 1 } else { -1 };
    match seg.axis {
        Axis::X => (sgn, 0),
        Axis::Y => (0, sgn)
    }
}

fn from_input(directions: String) -> Vec<Segment> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut total_dist: u32 = 0;
    let mut ret = Vec::new();
    for dir in directions.split(",") {
        let dist = dir[1..].parse::<i32>().unwrap();
        match dir.chars().nth(0).unwrap() {
            'R' => {
                ret.push(Segment{axis: Axis::X, x, y, step: dist, distance: total_dist});
                x += dist;
            },
            'L' => {
                ret.push(Segment{x, y, axis: Axis::X, step: -dist, distance: total_dist});
                x -= dist;
            },
            'U' => {
                ret.push(Segment{x, y, axis: Axis::Y, step: dist, distance: total_dist});
                y += dist;
            },
            'D' => {
                ret.push(Segment{x, y, axis: Axis::Y, step: -dist, distance: total_dist});
                y -= dist;
            },
            _ => panic!("Unexpected character")
        }
        total_dist += dist as u32;
    }
    ret
}

fn make_pixels(wire: &Vec<Segment>) -> HashMap<(i32, i32), u32> {
    let mut pixels: HashMap<(i32, i32), u32> = HashMap::new();
    let (mut x, mut y) = (0i32, 0i32);

    for seg in wire {
        let (ofs_x, ofs_y) = step_ofs(seg);
        for d in 1..seg.step.abs() as u32 {
            x += ofs_x; y += ofs_y;
            println!("{},{}", x, y);
            if !pixels.contains_key(&(x, y)) {
                pixels.insert((x, y), seg.distance + d);
            }
        }
    }

    pixels
}

const SAMPLE_INPUT: &str = "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

fn main() {
    /*let lines: Vec<Vec<Segment>> = String::from(SAMPLE_INPUT)
        .split("\n")
        .map(|x| from_input(String::from(x)))
        .collect();*/
    let lines: Vec<Vec<Segment>> = advent2019::load_input("03.txt")
        .into_iter()
        .map(from_input)
        .collect();

    let haystack = make_pixels(&lines[0]);

    let needles = &lines[1];
    let (mut x, mut y) = (0i32, 0i32);
    let mut min_dist = 99999999;

    for seg in needles {
        let (ofs_x, ofs_y) = step_ofs(seg);
        for d in 1..seg.step.abs() as u32 {
            x += ofs_x; y += ofs_y;
            match haystack.get(&(x, y)) {
                Some(dist) => {
                    let current_dist = dist + seg.distance + d;
                    println!("found intersection with distance {}: wire A is {}, wire B is {}", current_dist, dist, seg.distance + d);
                    min_dist = cmp::min(min_dist, current_dist)
                },
                _ => {}
            }
        }
    }

    println!("{}", min_dist)
}
