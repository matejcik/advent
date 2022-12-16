use std::{cmp::Ordering, io::BufRead, iter::Peekable};

use bstr::io::BufReadExt;

use crate::Solver;

macro_rules! digits {
    () => {
        (b'0'..=b'9')
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(u32),
    ListStart,
    ListEnd,
}

struct NumberTokenizer<I: Iterator<Item = u8>> {
    iter: Peekable<I>,
    last: Option<Token>,
    fake_nesting_level: u32,
    repeat_last: bool,
}

impl<I: Iterator<Item = u8>> NumberTokenizer<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            fake_nesting_level: 0,
            repeat_last: false,
            last: None,
        }
    }

    fn nest(&mut self) {
        self.fake_nesting_level += 1;
        self.repeat_last = true;
    }

    fn read_num(&mut self) -> u32 {
        let mut num = 0;
        while let Some(c) = self.iter.peek() {
            if digits!().contains(c) {
                num = num * 10 + (c - b'0') as u32;
                self.iter.next().expect("peek succeeded but next failed");
            } else {
                break;
            }
        }
        num
    }
}

impl<I: Iterator<Item = u8>> Iterator for NumberTokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b',') = self.iter.peek() {
            self.iter.next().expect("peek succeeded but next failed");
        };
        if self.repeat_last {
            self.repeat_last = false;
            return self.last;
        } else if self.fake_nesting_level > 0 {
            self.fake_nesting_level -= 1;
            return Some(Token::ListEnd);
        }
        self.last = match self.iter.peek() {
            Some(digits!()) => Some(Token::Number(self.read_num())),
            Some(b'[') => {
                self.iter.next().expect("peek succeeded but next failed");
                Some(Token::ListStart)
            }
            Some(b']') => {
                self.iter.next().expect("peek succeeded but next failed");
                Some(Token::ListEnd)
            }
            Some(_) => panic!("unexpected char: '{}'", *self.iter.peek().unwrap() as char),
            None => None,
        };
        self.last
    }
}

enum CmpState {
    Same,
    NumberDiffers(Ordering),
    FakeListLevelLeft(u32),
    FakeListLevelRight(u32),
}

fn is_digit(c: u8) -> bool {
    digits!().contains(&c)
}

#[derive(Debug)]
struct Number(Vec<u8>);

impl Number {
    fn compare(&self, other: &Self) -> Ordering {
        let mut left = NumberTokenizer::new(self.0.iter().copied());
        let mut right = NumberTokenizer::new(other.0.iter().copied());
        loop {
            match (left.next(), right.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(x), Some(y)) if x == y => (),
                (Some(Token::Number(a)), Some(Token::Number(b))) if a != b => {
                    return a.cmp(&b);
                }
                (Some(Token::ListStart), Some(Token::Number(_))) => right.nest(),
                (Some(Token::Number(_)), Some(Token::ListStart)) => left.nest(),
                (Some(Token::ListEnd), _) => return Ordering::Less,
                (_, Some(Token::ListEnd)) => return Ordering::Greater,
                _ => unreachable!(),
            };
        }
    }

    fn compare_bad(&self, other: &Self) -> Ordering {
        unimplemented!()
        // let (mut left, mut right) = (self.0.iter().peekable(), other.0.iter().peekable());
        // let mut state = CmpState::Same;
        // loop {
        //     let items = (left.peek(), right.peek());
        //     let (Some(&&(mut a)), Some(&&(mut b))) = items else {
        //         return match items {
        //             (Some(_), None) => Ordering::Greater,
        //             (None, Some(_)) => Ordering::Less,
        //             (None, None) => Ordering::Equal,
        //             _ => unreachable!(),
        //         }
        //     };
        //     state = match state {
        //         // fake list - left / right version, closing the lists
        //         CmpState::FakeListLevelLeft(n) if !is_digit(b) => {
        //             for _ in 0..n {
        //                 if b != b']' {
        //                     // left fake list is shorter
        //                     return Ordering::Less;
        //                 }
        //                 right.next().expect("peek succeeded but next failed");
        //                 b = **right.peek().expect("malformed right input - list not closed enough");
        //             }
        //             CmpState::Same
        //         }
        //         CmpState::FakeListLevelRight(n) if !is_digit(a) => {
        //             for _ in 0..n {
        //                 if a != b']' {
        //                     // right fake list is shorter
        //                     return Ordering::Greater;
        //                 }
        //                 left.next().expect("peek succeeded but next failed");
        //                 a = **left.peek().expect("malformed left input - list not closed enough");
        //             }
        //             CmpState::Same
        //         }
        //         CmpState::NumberDiffers(ord) => {
        //             if !is_digit(a) && !is_digit(b) {
        //                 // numbers had the same length, previous ordering applies
        //                 return ord;
        //             } else if !is_digit(a) {
        //                 // left number is shorter = smaller
        //                 return Ordering::Less;
        //             } else if !is_digit(b) {
        //                 // right number is shorter = smaller
        //                 return Ordering::Greater;
        //             }
        //             state
        //         }
        //         _ => match (a, b) {
        //             (digits!(), b'[') => {
        //                 let mut n = 0;
        //                 while let Some(&&b'[') = right.peek() {
        //                     n += 1;
        //                     right.next().expect("peek succeeded but next failed");
        //                 }
        //                 state = CmpState::FakeListLevelLeft(n);
        //                 continue;
        //             }
        //             (b'[', digits!()) => {
        //                 let mut n = 0;
        //                 while let Some(&&b'[') = left.peek() {
        //                     n += 1;
        //                     left.next().expect("peek succeeded but next failed");
        //                 }
        //                 state = CmpState::FakeListLevelRight(n);
        //                 continue;
        //             }
        //             (digits!(), digits!()) if a != b => {
        //                 // numbers differ
        //                 CmpState::NumberDiffers(a.cmp(&b))
        //             }
        //             _ if a == b => {
        //                 // same char, continue
        //                 CmpState::Same
        //             }
        //             (digits!(), _) => {
        //                 // shorter number wins
        //                 return Ordering::Greater;
        //             }
        //             (_, digits!()) => {
        //                 // shorter number wins
        //                 return Ordering::Less;
        //             }
        //             (b',', b']') => {
        //                 // shorter list wins, right is shorter
        //                 return Ordering::Greater;
        //             }
        //             (b']', b',') => {
        //                 // shorter list wins, left is shorter
        //                 return Ordering::Less;
        //             }
        //             _ => panic!("chars are {} and {}", a as char, b as char),
        //         }
        //     };
        //     left.next().expect("peek succeeded but next failed");
        //     right.next().expect("peek succeeded but next failed");
        // }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

fn part1_compare_by_pairs(input: &mut dyn BufRead) -> String {
    let mut lines = input.byte_lines();
    let mut i = 1;
    let mut pairs_sum = 0;
    loop {
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        let left = Number(a);
        let right = Number(b);
        if left <= right {
            pairs_sum += i;
        }
        i += 1;
        if lines.next().is_none() {
            break;
        }
    }
    pairs_sum.to_string()
}

fn part2_compare_all(input: &mut dyn BufRead) -> String {
    let mut numbers = input
        .byte_lines()
        .flatten()
        .filter_map(|line| (!line.is_empty()).then(move || Number(line)))
        .collect::<Vec<Number>>();
    let two = b"[[2]]";
    let six = b"[[6]]";
    numbers.push(Number(two.to_vec()));
    numbers.push(Number(six.to_vec()));
    numbers.sort();
    numbers
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n.0 == two || n.0 == six {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>()
        .to_string()
}

pub const SOLVERS: &[Solver] = &[part1_compare_by_pairs, part2_compare_all];

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn basic_comparison() {
        let left = b"6";
        let right = b"[6]";
        assert_eq!(Number(left.to_vec()), Number(right.to_vec()));
    }

    #[test]
    fn bad_case() {
        let left = b"[[[7,[2,3,3],5,8,9],[[1,8,6,7]],6,[8,[0,8,0,7,10],[8],[6,9,1],1],9]]";
        let right = b"[[7,4],[[[8,7],3],[0,8,9],6],[8,[3,[],5,[10,0],2],[1,2,[9,4],0]],[[],[],10,[[3],4,2]],[10,[8,7],4,[[3,3,5,6],[],[9,8,4,1],[0],10],[]]]";
        assert!(Number(left.to_vec()) > Number(right.to_vec()));
    }
}
