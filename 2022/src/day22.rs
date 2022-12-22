use std::io::BufRead;

use bstr::io::BufReadExt;

use crate::tiles::Tiles;
use crate::Solver;

struct Tokenizer(Vec<u8>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Left,
    Right,
    Number(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East = 0,
    South,
    West,
    North,
}

impl Direction {
    pub const fn left(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }

    pub const fn right(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }

    pub const fn step(&self) -> (isize, isize) {
        match self {
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::North => (0, -1),
        }
    }
}

impl Tokenizer {
    pub fn iter(&self) -> impl Iterator<Item = Token> + '_ {
        let mut iterator = self.0.iter().copied().peekable();
        std::iter::from_fn(move || match iterator.peek()? {
            b'L' => {
                iterator.next();
                Some(Token::Left)
            }
            b'R' => {
                iterator.next();
                Some(Token::Right)
            }
            b'0'..=b'9' => {
                let mut number = 0;
                while let Some(b'0'..=b'9') = iterator.peek() {
                    number = number * 10 + iterator.next().unwrap() - b'0';
                }
                Some(Token::Number(number))
            }
            _ => None,
        })
    }
}

#[derive(Debug)]
struct Walker {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Walker {
    pub fn new(tiles: &Tiles<u8>) -> Self {
        for x in 0..tiles.width() {
            if tiles[(x, 0)] == b'.' {
                return Self {
                    x,
                    y: 0,
                    direction: Direction::East,
                };
            }
        }
        panic!("No starting point found");
    }

    pub fn walk(&mut self, token: Token, map: &mut Tiles<u8>) {
        match token {
            Token::Left => self.direction = self.direction.left(),
            Token::Right => self.direction = self.direction.right(),
            Token::Number(n) => {
                let (dx, dy) = self.direction.step();
                let mut i = 0;
                let mut x = self.x;
                let mut y = self.y;
                loop {
                    x = (x as isize + dx).rem_euclid(map.width() as isize) as usize;
                    y = (y as isize + dy).rem_euclid(map.height() as isize) as usize;
                    match map[(x, y)] {
                        b'.' => {
                            self.x = x;
                            self.y = y;
                            i += 1;
                        }
                        b'#' => {
                            break;
                        }
                        b' ' => {}
                        _ => unreachable!(),
                    }
                    if i == n {
                        break;
                    }
                }
            }
        }
    }
}

fn part1_2d_walk(input: &mut dyn BufRead) -> String {
    let lines = input.byte_lines().flatten().collect::<Vec<_>>();
    let max_line_width = lines
        .iter()
        .take(lines.len() - 2)
        .map(|line| line.len())
        .max()
        .unwrap();
    let mut tiles = Tiles::new(max_line_width, lines.len() - 2, b' ');
    for y in 0..tiles.height() {
        let start = tiles.line_width * y;
        let end = tiles.line_width.min(lines[y].len());
        tiles.entries[start..start + end].copy_from_slice(&lines[y]);
    }

    let instructions = Tokenizer(lines.into_iter().last().unwrap());

    let mut walker = Walker::new(&tiles);
    for instr in instructions.iter() {
        walker.walk(instr, &mut tiles);
    }
    ((walker.y + 1) * 1000 + (walker.x + 1) * 4 + walker.direction as usize).to_string()
}

pub const SOLVERS: &[Solver] = &[part1_2d_walk];
