use advent2019;
use std::cmp;
use std::cmp::Ordering;

use reduce::Reduce;

#[derive(Debug)]
enum Segment {
    // x, ybot, ytop
    Vertical(i32, i32, i32),
    // y, xleft, xright
    Horizontal(i32, i32, i32)
}

use Segment::{Horizontal, Vertical};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point(i32, i32);

impl From<Point> for i32 {
    fn from(Point(mx, my): Point) -> i32 {
        mx.abs() + my.abs()
    }
}

impl Ord for Point {
    fn cmp(&self, Point(ox, oy): &Self) -> Ordering {
        let Self(mx, my) = self;
        (mx.abs() + my.abs()).cmp(&(ox.abs() + oy.abs()))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_intersection(a: &Segment, b: &Segment) -> Option<Point> {
    match a {
        Vertical(x, ybot, ytop) => match b {
            Horizontal(y, xleft, xright)
                if x >= xleft && x <=xright
                && y >= ybot && y <= ytop 
            => Some(Point(*x, *y)),
            Vertical(x2, y2bot, y2top) if x2 == x
            => {
                let bot = cmp::max(ybot, y2bot);
                let top = cmp::min(ytop, y2top);
                if bot <= top {
                    if bot.abs() < top.abs() {
                        Some(Point(*x, *bot))
                    } else {
                        Some(Point(*x, *top))
                    }
                } else {
                    None
                }
            },
            _ => None
        },
        Horizontal(y, xleft, xright) => match b {
            Horizontal(y2, x2left, x2right) if y2 == y
            => {
                let left = cmp::max(xleft, x2left);
                let right = cmp::min(xright, x2right);
                if left <= right {
                    if left.abs() < right.abs() {
                        Some(Point(*left, *y))
                    } else {
                        Some(Point(*right, *y))
                    }
                } else {
                    None
                }
            },
            Horizontal(_, _, _) => None,
            _ => find_intersection(&b, &a)
        }
    }
}

fn from_input(directions: String) -> Vec<Segment> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut ret = Vec::new();
    for dir in directions.split(",") {
        let dist = dir[1..].parse::<i32>().unwrap();
        match dir.chars().nth(0).unwrap() {
            'R' => {
                ret.push(Horizontal(y, x, x + dist));
                x += dist
            },
            'L' => {
                ret.push(Horizontal(y, x - dist, x));
                x -= dist
            },
            'U' => {
                ret.push(Vertical(x, y, y + dist));
                y += dist
            },
            'D' => {
                ret.push(Vertical(x, y - dist, y));
                y -= dist
            },
            _ => panic!("Unexpected character")
        }
    }
    ret
}

fn best_intersect(needle: &Segment, haystack: &Vec<Segment>) -> Option<Point> {
    haystack.into_iter()
        .filter_map(|s| match find_intersection(needle, s) {
            Some(Point(0, 0)) => None,
            x => x
        })
        .reduce(cmp::min)
}

const SAMPLE_INPUT: &str = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

fn main() {
    /*let lines: Vec<Vec<Segment>> = String::from(SAMPLE_INPUT)
        .split("\n")
        .map(|x| from_input(&String::from(x)))
        .collect();*/
    let lines: Vec<Vec<Segment>> = advent2019::load_input("03.txt")
        .into_iter()
        .map(from_input)
        .collect();

    let haystack = &lines[0];
    let needles = &lines[1];

    let p = needles.into_iter()
        .filter_map(|needle| best_intersect(needle, &haystack))
        .reduce(cmp::min)
        .unwrap();

    println!("{:?}: {}", p, i32::from(p))
}
