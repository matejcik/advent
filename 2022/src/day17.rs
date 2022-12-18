use std::io::BufRead;

use crate::Solver;

#[rustfmt::skip]
mod shapes {
    pub struct ShapeInfo {
        pub data: &'static [u8],
        pub shift: usize,
    }

    pub const SHAPE_MINUS: ShapeInfo = ShapeInfo {
        data: &[
            0b1111,
        ],
        shift: 1,
    };
    pub const SHAPE_PLUS: ShapeInfo = ShapeInfo {
        data: &[
            0b010,
            0b111,
            0b010,
        ],
        shift: 2,
    };
    pub const SHAPE_L: ShapeInfo = ShapeInfo {
        data: &[
            0b111,
            0b001,
            0b001,
        ],
        shift: 2,
    };
    pub const SHAPE_I: ShapeInfo = ShapeInfo {
        data: &[
            0b1,
            0b1,
            0b1,
            0b1,
        ],
        shift: 4,
    };
    pub const SHAPE_BLOCK: ShapeInfo = ShapeInfo {
        data: &[
            0b11,
            0b11,
        ],
        shift: 3,
    };
}

const SHAPES: &[shapes::ShapeInfo] = &[
    shapes::SHAPE_MINUS,
    shapes::SHAPE_PLUS,
    shapes::SHAPE_L,
    shapes::SHAPE_I,
    shapes::SHAPE_BLOCK,
];

#[derive(Debug)]
pub struct Shape {
    shape: &'static [u8],
    shift: usize,
    height_until_floor: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FallOrSettle {
    Fall,
    Settle,
}

impl Shape {
    fn new(shape_info: &shapes::ShapeInfo, chamber_height: usize) -> Self {
        Self {
            shape: shape_info.data,
            shift: shape_info.shift,
            height_until_floor: chamber_height + 3,
        }
    }

    fn intersects(&self, chamber: &Vec<u8>) -> bool {
        if self.height_until_floor >= chamber.len() {
            return false;
        }
        for (entry, rocks) in self
            .shape
            .iter()
            .zip(chamber[self.height_until_floor..].iter())
        {
            if (*entry << self.shift) & *rocks != 0 {
                return true;
            }
        }
        false
    }

    fn try_shift(&mut self, shift_by: isize, chamber: &Vec<u8>) -> Result<(), ()> {
        let shift = self.shift as isize + shift_by;
        if shift < 0 || shift > 7 {
            return Err(());
        }
        for entry in self.shape {
            if (*entry << shift) & 0b1000_0000 != 0 {
                return Err(());
            }
        }
        let new = Self {
            shift: shift as usize,
            ..*self
        };
        if new.intersects(chamber) {
            Err(())
        } else {
            self.shift = shift as usize;
            Ok(())
        }
    }

    fn settle(&self, chamber: &mut Vec<u8>) {
        for (entry, rocks) in self
            .shape
            .iter()
            .zip(chamber[self.height_until_floor..].iter_mut())
        {
            // each colliding entry is ORed
            *rocks |= *entry << self.shift;
        }
        // extend the chamber if necessary
        let first_line_above_chamber = chamber.len() - self.height_until_floor;
        if first_line_above_chamber < self.shape.len() {
            chamber.extend(
                self.shape[first_line_above_chamber..]
                    .iter()
                    .map(|entry| *entry << self.shift),
            );
        }
    }

    fn fall_or_settle(&mut self, chamber: &mut Vec<u8>) -> FallOrSettle {
        if self.height_until_floor > chamber.len() {
            // if we're above the chamber, we're falling without checking
            self.height_until_floor -= 1;
            return FallOrSettle::Fall;
        }
        if self.height_until_floor == 0 {
            self.settle(chamber);
            return FallOrSettle::Settle;
        }
        let new = Self {
            height_until_floor: self.height_until_floor - 1,
            ..*self
        };
        if new.intersects(chamber) {
            self.settle(chamber);
            FallOrSettle::Settle
        } else {
            self.height_until_floor -= 1;
            FallOrSettle::Fall
        }
    }

    fn print(&self, chamber: &Vec<u8>) {
        let top = chamber
            .len()
            .max(self.height_until_floor + self.shape.len());
        for i in (0..top).rev() {
            let rocks = chamber.get(i).copied().unwrap_or(0);
            let falling = if i >= self.height_until_floor {
                self.shape
                    .get(i - self.height_until_floor)
                    .copied()
                    .unwrap_or(0)
            } else {
                0
            };
            print_line(rocks, falling << self.shift);
        }
        println!("+-------+");
    }
}

fn print_line(rocks: u8, falling: u8) {
    print!("|");
    for i in (0..7).rev() {
        if rocks & (1 << i) != 0 {
            print!("#");
        } else if falling & (1 << i) != 0 {
            print!("@");
        } else {
            print!(".");
        }
    }
    println!("|");
}

fn part1_tower_height(input: &mut dyn BufRead) -> String {
    const ROCK_LIMIT: usize = 2022;

    let mut chamber = Vec::with_capacity(5000);
    let mut shape = Shape::new(&SHAPES[0], chamber.len());
    let mut shape_counter = 0;

    let mut directions = Vec::with_capacity(15000);
    input.read_until(b'\n', &mut directions).unwrap();
    assert!(matches!(directions.pop(), Some(b'\n')));

    for dir in directions.iter().cycle() {
        match dir {
            b'>' => shape.try_shift(-1, &chamber),
            b'<' => shape.try_shift(1, &chamber),
            _ => unreachable!(),
        }
        .unwrap_or_default();
        // println!("{}", *dir as char);
        // shape.print(&chamber);
        // println!();
        match shape.fall_or_settle(&mut chamber) {
            FallOrSettle::Fall => {}
            FallOrSettle::Settle => {
                shape_counter += 1;
                if shape_counter == ROCK_LIMIT {
                    break;
                }
                shape = Shape::new(&SHAPES[shape_counter % SHAPES.len()], chamber.len());
            }
        }
        // shape.print(&chamber);
        // println!();
    }
    chamber.len().to_string()
}

pub const SOLVERS: &[Solver] = &[part1_tower_height];

#[allow(unused)]
mod tests {
    use super::*;

    fn plus() -> Shape {
        Shape {
            shape: shapes::SHAPE_PLUS.data,
            shift: shapes::SHAPE_PLUS.shift,
            height_until_floor: 0,
        }
    }

    #[test]
    fn test_shift() {
        let chamber = vec![];
        let mut shape = plus();

        assert!(shape.try_shift(-2, &chamber).is_ok());
        assert!(shape.try_shift(-1, &chamber).is_err());

        shape = plus();

        assert!(shape.try_shift(2, &chamber).is_ok());
        assert!(shape.try_shift(1, &chamber).is_err());
    }

    #[test]
    fn test_shift_with_chamber() {
        let chamber = vec![0b0000_0001, 0b0000_0000, 0b0000_0001];
        let mut shape = plus();
        assert!(shape.try_shift(-2, &chamber).is_ok());

        let chamber = vec![0b0000_0000, 0b0000_0001, 0b0000_0000];
        let mut shape = plus();
        assert!(shape.try_shift(-2, &chamber).is_err());

        let chamber = vec![0b0000_0011, 0b0000_0000, 0b0000_0011];
        let mut shape = plus();
        assert!(shape.try_shift(-1, &chamber).is_ok());
        assert_eq!(shape.shift, 1);
        assert!(shape.try_shift(-1, &chamber).is_err());
        assert_eq!(shape.shift, 1);

        let chamber = vec![0b0100_0001, 0b0100_0000, 0b0100_0001];
        let mut shape = plus();
        assert!(shape.try_shift(1, &chamber).is_ok());
        assert!(shape.try_shift(1, &chamber).is_err());

        let chamber = vec![0b0000_0011];
        let mut shape = plus();
        assert!(shape.try_shift(-1, &chamber).is_ok());
        assert!(shape.try_shift(-1, &chamber).is_err());

        let chamber = vec![0b0000_0011, 0b0000_0011];
        let mut shape = plus();
        assert!(shape.try_shift(-1, &chamber).is_err());
    }

    #[test]
    fn test_fall() {
        let mut chamber = vec![];
        let mut shape = plus();
        shape.shift = 0;
        assert_eq!(shape.fall_or_settle(&mut chamber), FallOrSettle::Settle);
        assert_eq!(chamber, shape.shape);

        let mut chamber = vec![];
        let mut shape = plus();
        shape.height_until_floor = 1;
        assert_eq!(shape.fall_or_settle(&mut chamber), FallOrSettle::Fall);
        assert_eq!(chamber, vec![]);
        assert_eq!(shape.height_until_floor, 0);

        let mut chamber = vec![0b1111_1111];
        let mut shape = plus();
        shape.height_until_floor = 1;
        assert_eq!(shape.fall_or_settle(&mut chamber), FallOrSettle::Settle);
        assert_eq!(
            chamber,
            vec![0b1111_1111, 0b0000_1000, 0b0001_1100, 0b0000_1000]
        );
    }
}
