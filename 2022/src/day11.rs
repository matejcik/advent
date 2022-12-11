use std::{collections::VecDeque, io::BufRead, ops::Deref};

use bstr::io::BufReadExt;

use crate::{parse_num, parse_nums, Solver};

const MAX_ITEMS: usize = 64;

enum Operation {
    Add(u64),
    Multiply(u64),
    Double,
    Power,
}

impl Operation {
    fn decode(slice: &[u8]) -> Self {
        match slice[0] {
            b'+' => {
                if slice[2] == b'o' {
                    // old + old
                    Operation::Double
                } else {
                    // old + <number>
                    Operation::Add(parse_num(&slice[2..]))
                }
            }
            b'*' => {
                if slice[2] == b'o' {
                    // old * old
                    Operation::Power
                } else {
                    // old * <number>
                    Operation::Multiply(parse_num(&slice[2..]))
                }
            }
            _ => panic!("Invalid operation"),
        }
    }

    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(n) => old + n,
            Operation::Multiply(n) => old * n,
            Operation::Double => old * 2,
            Operation::Power => old * old,
        }
    }
}

struct ThrownItem {
    monkey: u32,
    item: u64,
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_div: u64,
    throw: [u32; 2],
    activity: u64,
}

impl Monkey {
    fn load(source: &mut impl Iterator<Item = impl Deref<Target = [u8]>>) -> Self {
        // Monkey N:
        source.next().unwrap();
        //   Starting items: <number>, <number>, ...
        let line = source.next().unwrap();
        let mut numbers = [0; MAX_ITEMS];
        let parsed = parse_nums(&line["  Starting items: ".len()..], &mut numbers);
        //   Operation: new = old <op def>
        let line = source.next().unwrap();
        let operation = Operation::decode(&line["  Operation: new = old ".len()..]);
        //   Test: divisible by <number>
        let line = source.next().unwrap();
        let test_div = parse_num(&line["  Test: divisible by ".len()..]);
        //     If true: throw to monkey <number>
        let line = source.next().unwrap();
        let if_true = parse_num(&line["    If true: throw to monkey ".len()..]) as u32;
        //     If false: throw to monkey <number>
        let line = source.next().unwrap();
        let if_false = parse_num(&line["    If false: throw to monkey ".len()..]) as u32;

        let mut items = VecDeque::with_capacity(MAX_ITEMS);
        items.extend(numbers[..parsed].iter());
        Monkey {
            items,
            operation,
            test_div,
            throw: [if_false, if_true],
            activity: 0,
        }
    }

    fn process_next(&mut self, div_worry: bool) -> Option<ThrownItem> {
        let item = self.items.pop_front()?;
        self.activity += 1;
        let applied = self.operation.apply(item);
        let new = if div_worry { applied / 3} else { applied };
        let monkey = self.throw[(new % self.test_div == 0) as usize];
        Some(ThrownItem { monkey, item: new })
    }

    fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn dyn_monkey_business(input: &mut dyn BufRead, rounds: usize, div_worry: bool) -> String {
    let mut lines = input.byte_lines().flatten();
    let mut monkeys = Vec::with_capacity(20);
    // assume at least one monkey
    loop {
        monkeys.push(Monkey::load(&mut lines));
        if lines.next().is_none() {
            break;
        }
    }

    let modulo = monkeys.iter().map(|m| m.test_div).product::<u64>();

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(ThrownItem { monkey, item }) = monkeys[m].process_next(div_worry) {
                monkeys[monkey as usize].catch(item % modulo);
            }
        }
    }

    monkeys.sort_by_key(|m| m.activity);
    (monkeys[monkeys.len() - 1].activity * monkeys[monkeys.len() - 2].activity).to_string()
}

fn part1_monkey_business(input: &mut dyn BufRead) -> String {
    dyn_monkey_business(input, 20, true)
}

fn part2_monkey_business(input: &mut dyn BufRead) -> String {
    dyn_monkey_business(input, 10_000, false)
}

pub const SOLVERS: &[Solver] = &[part1_monkey_business, part2_monkey_business];
