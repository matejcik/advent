use std::{io::BufRead, ops::Deref};

use bstr::io::BufReadExt;

use crate::{parse_num, Solver};

pub const fn ltr(line1: u32, line2: u32, line3: u32, line4: u32, line5: u32, line6: u32) -> u32 {
    line1 << (4 * 5)
        | line2 << (4 * 4)
        | line3 << (4 * 3)
        | line4 << (4 * 2)
        | line5 << (4 * 1)
        | line6 << (4 * 0)
}

const LETTERS: [u32; 26] = [
    // TODO formatting so that the letters are visible
    ltr(0b0110, 0b1001, 0b1001, 0b1111, 0b1001, 0b1001),
    ltr(0b1110, 0b1001, 0b1110, 0b1001, 0b1001, 0b1110),
    ltr(0b0111, 0b1000, 0b1000, 0b1000, 0b1000, 0b0111),
    ltr(0b1110, 0b1001, 0b1001, 0b1001, 0b1001, 0b1110),
    ltr(0b1111, 0b1000, 0b1110, 0b1000, 0b1000, 0b1111),
    ltr(0b1111, 0b1000, 0b1110, 0b1000, 0b1000, 0b1000),
    ltr(0b0110, 0b1001, 0b1000, 0b1011, 0b1001, 0b0111),
    ltr(0b1001, 0b1001, 0b1111, 0b1001, 0b1001, 0b1001),
    ltr(0b111, 0b010, 0b010, 0b010, 0b010, 0b111),
    ltr(0b0011, 0b0001, 0b0001, 0b0001, 0b1001, 0b0110),
    ltr(0b1001, 0b1010, 0b1100, 0b1010, 0b1010, 0b1001),
    ltr(0b1000, 0b1000, 0b1000, 0b1000, 0b1000, 0b1111),
    ltr(0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001),
    ltr(0b1001, 0b1101, 0b1011, 0b1001, 0b1001, 0b1001),
    ltr(0b0110, 0b1001, 0b1001, 0b1001, 0b1001, 0b0110),
    ltr(0b1110, 0b1001, 0b1001, 0b1110, 0b1000, 0b1000),
    ltr(0b0110, 0b1001, 0b1001, 0b1001, 0b0110, 0b0001),
    ltr(0b1110, 0b1001, 0b1001, 0b1110, 0b1010, 0b1001),
    ltr(0b0111, 0b1000, 0b0110, 0b0001, 0b0001, 0b1110),
    ltr(0b111, 0b010, 0b010, 0b010, 0b010, 0b010),
    ltr(0b1001, 0b1001, 0b1001, 0b1001, 0b1001, 0b0110),
    ltr(0b1001, 0b1001, 0b1001, 0b1001, 0b0110, 0b0110),
    ltr(0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001),
    ltr(0b1001, 0b1001, 0b0110, 0b0110, 0b1001, 0b1001),
    ltr(0b1001, 0b1001, 0b0110, 0b0010, 0b0010, 0b0010),
    ltr(0b1111, 0b0001, 0b0010, 0b0100, 0b1000, 0b1111),
];

struct Computer<T>
where
    T: Iterator,
    T::Item: Deref<Target = [u8]>,
{
    pub register: i32,
    add_after: i32,
    cycles_until_add: usize,
    instructions: T,
}

impl<T> Computer<T>
where
    T: Iterator,
    T::Item: Deref<Target = [u8]>,
{
    fn new(instructions: T) -> Self {
        Self {
            register: 1,
            add_after: 0,
            cycles_until_add: 0,
            instructions,
        }
    }

    fn load_instr(&mut self, instr: &[u8]) {
        match instr[0] {
            b'n' => {
                // noop
                self.cycles_until_add = 1;
                self.add_after = 0;
            }
            b'a' => {
                // addx
                self.cycles_until_add = 2;
                if instr[5] == b'-' {
                    self.add_after = -(parse_num(&instr[6..]) as i32);
                } else {
                    self.add_after = parse_num(&instr[5..]) as i32;
                }
            }
            _ => panic!("invalid instruction"),
        }
    }

    fn step(&mut self) -> bool {
        if self.cycles_until_add == 0 {
            self.register += self.add_after;
            if let Some(instr) = self.instructions.next() {
                self.load_instr(&instr);
            } else {
                return false;
            }
        }
        self.cycles_until_add -= 1;
        true
    }
}

#[allow(unused)]
fn part1_interesting_values(input: &mut dyn BufRead) -> String {
    let mut cycle = 0;
    let mut add_after = 0;
    let mut cycles_until_add = 0;
    let mut register = 1;
    let mut byte_lines = input.byte_lines();

    let mut interesting_values = 0;
    loop {
        if (cycle - 20) % 40 == 0 {
            interesting_values += cycle * register;
        }
        if cycle == cycles_until_add {
            register += add_after;
            if let Some(Ok(line)) = byte_lines.next() {
                match line[0] {
                    b'n' => {
                        // noop
                        cycles_until_add = cycle + 1;
                        add_after = 0;
                    }
                    b'a' => {
                        // addx
                        cycles_until_add = cycle + 2;
                        if line[5] == b'-' {
                            add_after = -(parse_num(&line[6..]) as i32);
                        } else {
                            add_after = parse_num(&line[5..]) as i32;
                        }
                    }
                    _ => panic!("invalid instruction"),
                }
            } else {
                break;
            }
        }
        cycle += 1;
    }
    interesting_values.to_string()
}

fn part1_with_iter(input: &mut dyn BufRead) -> String {
    let byte_lines = input.byte_lines().flatten();
    let mut computer = Computer::new(byte_lines);
    let mut interesting_values = 0;
    let mut cycle = 0;
    loop {
        if (cycle - 20) % 40 == 0 {
            interesting_values += cycle as i32 * computer.register;
        }
        cycle += 1;
        if !computer.step() {
            break;
        }
    }
    interesting_values.to_string()
}

fn part2_crt(input: &mut dyn BufRead) -> String {
    let byte_lines = input.byte_lines().flatten();
    let mut computer = Computer::new(byte_lines);
    computer.step();
    let mut cycle = 0;
    let mut letters = [0u32; 8];
    loop {
        let pixel = cycle % 40;
        let letter = (pixel / 5) as usize;
        if pixel % 5 != 4 {
            letters[letter] <<= 1;
            if (pixel - computer.register).abs() <= 1 {
                letters[letter] |= 1;
            }
        }
        if !computer.step() {
            break;
        }
        cycle += 1;
    }
    let mut result = String::with_capacity(8);
    for letter in letters {
        let idx = LETTERS.iter().position(|ltr| *ltr == letter);
        result.push(if let Some(idx) = idx {
            (idx as u8 + b'A') as char
        } else {
            '?'
        });
    }
    result
}

pub const SOLVERS: &[Solver] = &[part1_with_iter, part2_crt];
