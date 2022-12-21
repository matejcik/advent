use std::{cell::RefCell, collections::HashMap, io::BufRead};

use bstr::io::BufReadExt;

use crate::{parse_num, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Monkey(u64);

impl Monkey {
    // if the monkey has an lowercase ascii name, it has the bits 0x60 set
    const MONKEY_ASCII_MASK: u64 = 0x60_60_60_60;

    pub fn load_monkey(line: &[u8]) -> Self {
        Self(u32::from_ne_bytes(line.try_into().unwrap()) as u64)
    }

    pub const fn is_monkey(&self) -> bool {
        // 0x60
        self.0 & Self::MONKEY_ASCII_MASK == Self::MONKEY_ASCII_MASK
    }

    pub const fn is_number(&self) -> bool {
        !self.is_monkey()
    }

    pub fn to_string(&self) -> String {
        if self.is_monkey() {
            let bytes = self.0.to_ne_bytes();
            std::str::from_utf8(&bytes).unwrap().to_string()
        } else {
            self.0.to_string()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    pub fn from_char(c: u8) -> Self {
        match c {
            b'+' => Self::Add,
            b'-' => Self::Sub,
            b'*' => Self::Mul,
            b'/' => Self::Div,
            _ => unreachable!(),
        }
    }

    pub fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }

    pub fn unapply(&self, result: u64, side: Side, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => match side {
                Side::Left => result - right,
                Side::Right => result - left,
            },
            Self::Sub => match side {
                Side::Left => result + right,
                Side::Right => left - result,
            },
            Self::Mul => match side {
                Side::Left => result / right,
                Side::Right => result / left,
            },
            Self::Div => match side {
                Side::Left => result * right,
                Side::Right => left / result,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonkeyOp {
    left: Monkey,
    right: Monkey,
    op: Operation,
    human_side: RefCell<Option<Side>>,
    result: RefCell<Option<u64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn other(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl MonkeyOp {
    pub fn load(line: &[u8]) -> Self {
        if line.len() == 4 + 3 + 4 {
            Self {
                left: Monkey::load_monkey(line[0..4].try_into().unwrap()),
                right: Monkey::load_monkey(line[4 + 3..4 + 3 + 4].try_into().unwrap()),
                op: Operation::from_char(line[5]),
                human_side: RefCell::new(None),
                result: RefCell::new(None),
            }
        } else {
            let number = parse_num(line);
            Self::number(Monkey(number))
        }
    }

    pub const fn number(m: Monkey) -> Self {
        assert!(m.is_number());
        Self {
            left: m,
            right: Monkey(1),
            op: Operation::Mul,
            human_side: RefCell::new(None),
            result: RefCell::new(Some(m.0)),
        }
    }

    pub fn calculate(&self, others: &HashMap<Monkey, Self>) -> u64 {
        let left = if self.left.is_monkey() {
            others[&self.left].calculate(others)
        } else {
            self.left.0
        };
        let right = if self.right.is_monkey() {
            others[&self.right].calculate(others)
        } else {
            self.right.0
        };
        let result = self.op.apply(left, right);
        self.result.replace(Some(result));
        result
    }

    pub fn find_human(&self, human: Monkey, others: &HashMap<Monkey, Self>) -> Option<Side> {
        if self.human_side.borrow().is_none() {
            let human_side = if self.left == human {
                Some(Side::Left)
            } else if self.right == human {
                Some(Side::Right)
            } else {
                let left_human = if self.left.is_monkey() {
                    others[&self.left].find_human(human, others)
                } else {
                    None
                };
                let right_human = if self.right.is_monkey() {
                    others[&self.right].find_human(human, others)
                } else {
                    None
                };
                if left_human.is_some() {
                    Some(Side::Left)
                } else if right_human.is_some() {
                    Some(Side::Right)
                } else {
                    None
                }
            };
            self.human_side.replace(human_side);
        }
        *self.human_side.borrow()
    }

    pub fn left_result(&self, others: &HashMap<Monkey, Self>) -> u64 {
        if self.left.is_number() {
            self.left.0
        } else {
            others[&self.left].result.borrow().unwrap()
        }
    }

    pub fn right_result(&self, others: &HashMap<Monkey, Self>) -> u64 {
        if self.right.is_number() {
            self.right.0
        } else {
            others[&self.right].result.borrow().unwrap()
        }
    }

    pub fn get(&self, side: Side) -> Monkey {
        match side {
            Side::Left => self.left,
            Side::Right => self.right,
        }
    }

    pub fn human_value(&self, human: Monkey, others: &HashMap<Monkey, Self>, passdown: u64) -> u64 {
        let left_result = self.left_result(others);
        let right_result = self.right_result(others);
        let side = self.human_side.borrow().unwrap();
        let result = self.op.unapply(passdown, side, left_result, right_result);
        if self.left == human || self.right == human {
            result
        } else {
            let human_side = self.get(side);
            others[&human_side].human_value(human, others, result)
        }
    }
}

fn part1_monkey_tree(input: &mut dyn BufRead) -> String {
    let monkeys = input
        .byte_lines()
        .flatten()
        .map(|line| {
            let monkey = Monkey::load_monkey(&line[..4]);
            let op = MonkeyOp::load(&line[6..]);
            (monkey, op)
        })
        .collect::<HashMap<_, _>>();

    let root = Monkey::load_monkey(b"root");
    monkeys[&root].calculate(&monkeys).to_string()
}

fn part2_human_value(input: &mut dyn BufRead) -> String {
    let monkeys = input
        .byte_lines()
        .flatten()
        .map(|line| {
            let monkey = Monkey::load_monkey(&line[..4]);
            let op = MonkeyOp::load(&line[6..]);
            (monkey, op)
        })
        .collect::<HashMap<_, _>>();

    let root = Monkey::load_monkey(b"root");
    let root_monkey = &monkeys[&root];
    // prepare results
    root_monkey.calculate(&monkeys);
    // find human side
    let human = Monkey::load_monkey(b"humn");
    root_monkey.find_human(human, &monkeys);
    // calculate human value
    let human_side = root_monkey.human_side.borrow().unwrap();
    let human_side_monkey = &monkeys[&root_monkey.get(human_side)];
    match human_side {
        Side::Left => {
            human_side_monkey.human_value(human, &monkeys, root_monkey.right_result(&monkeys))
        }
        Side::Right => {
            human_side_monkey.human_value(human, &monkeys, root_monkey.left_result(&monkeys))
        }
    }
    .to_string()
}

pub const SOLVERS: &[Solver] = &[part1_monkey_tree, part2_human_value];
