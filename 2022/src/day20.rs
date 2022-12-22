use std::{io::BufRead, ops::Index};

use bstr::io::BufReadExt;

use crate::{parse_num, Solver};

struct SparseVec<T>(Vec<Vec<T>>);

impl<T: Clone> SparseVec<T> {
    const CHUNK_SIZE: usize = 128;

    fn with_capacity(capacity: usize) -> Self {
        let chunks = capacity.div_ceil(Self::CHUNK_SIZE);
        let mut chunk_vec = Vec::with_capacity(chunks);
        for _ in 0..chunks {
            chunk_vec.push(Vec::with_capacity(Self::CHUNK_SIZE * 2));
        }
        Self(chunk_vec)
    }

    fn rebalance(&mut self) {
        let len = self.len();
        let mut new_vec = Vec::with_capacity(self.0.len() * Self::CHUNK_SIZE);
        let mut chunk = Vec::with_capacity(Self::CHUNK_SIZE * 2);
        for prev_chunk in self.0.iter() {
            let mut prev_slice = prev_chunk.as_slice();
            while prev_slice.len() >= Self::CHUNK_SIZE - chunk.len() {
                let remain = Self::CHUNK_SIZE - chunk.len();
                chunk.extend_from_slice(&prev_slice[..remain]);
                prev_slice = &prev_slice[remain..];
                let len = chunk.len();
                new_vec.push(chunk);
                chunk = Vec::with_capacity(Self::CHUNK_SIZE * 2);
            }
            chunk.extend_from_slice(prev_slice);
        }
        if chunk.len() > 0 {
            new_vec.push(chunk);
        }
        self.0 = new_vec;
        assert_eq!(len, self.len());
    }

    pub fn insert(&mut self, idx: usize, item: T) {
        let mut total = 0;
        for chunk in self.0.iter_mut() {
            if idx <= total + chunk.len() {
                chunk.insert(idx - total, item);
                if chunk.len() >= Self::CHUNK_SIZE * 2 {
                    self.rebalance();
                }
                return;
            }
            total += chunk.len();
        }
        panic!("index out of bounds");
    }

    pub fn remove(&mut self, idx: usize) {
        let mut total = 0;
        for chunk in self.0.iter_mut() {
            if idx < total + chunk.len() {
                chunk.remove(idx - total);
                if chunk.len() < Self::CHUNK_SIZE / 2 {
                    self.rebalance();
                }
                return;
            }
            total += chunk.len();
        }
        panic!("index out of bounds");
    }

    pub fn iter_from(&self, idx: usize) -> impl Iterator<Item = &T> + Clone + '_ {
        let mut total = 0;
        for (chunk_idx, chunk) in self.0.iter().enumerate() {
            if idx < total + chunk.len() {
                return SparseVecIter {
                    sparse_vec: self,
                    chunk_idx,
                    item_idx: idx - total,
                };
            }
            total += chunk.len();
        }
        panic!("index out of bounds");
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|chunk| chunk.len()).sum()
    }
}

#[derive(Clone)]
struct SparseVecIter<'a, T: Clone> {
    sparse_vec: &'a SparseVec<T>,
    chunk_idx: usize,
    item_idx: usize,
}

impl<'a, T: Clone> Iterator for SparseVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk_idx >= self.sparse_vec.0.len() {
            self.chunk_idx = 0;
            self.item_idx = 0;
            return self.next();
        }
        let chunk = &self.sparse_vec.0[self.chunk_idx];
        if self.item_idx >= chunk.len() {
            self.chunk_idx += 1;
            self.item_idx = 0;
            return self.next();
        }
        let item = &chunk[self.item_idx];
        self.item_idx += 1;
        Some(item)
    }
}

impl<T> Index<usize> for SparseVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        let mut total = 0;
        for chunk in self.0.iter() {
            if idx < total + chunk.len() {
                return &chunk[idx - total];
            }
            total += chunk.len();
        }
        panic!("index out of bounds");
    }
}

impl<T: Clone> From<Vec<T>> for SparseVec<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut sparse_vec = Self::with_capacity(vec.len());
        for (i, chunk) in sparse_vec.0.iter_mut().enumerate() {
            let start = i * Self::CHUNK_SIZE;
            let end = vec.len().min((i + 1) * Self::CHUNK_SIZE);
            chunk.extend_from_slice(&vec[start..end]);
        }
        sparse_vec
    }
}

fn parse_num_negative(line: &[u8]) -> i64 {
    if line[0] == b'-' {
        -(parse_num(&line[1..]) as i64)
    } else {
        parse_num(line) as i64
    }
}

fn do_mix(data: &mut SparseVec<(usize, i64)>, start_finger: usize) -> usize {
    let len = data.len();
    let mut finger = start_finger;
    let mut cur = 0;
    while cur < len {
        while data[finger].0 != cur {
            finger = (finger + 1) % len
        }
        let (idx, n) = data[finger];
        assert_eq!(idx, cur);
        let dest_index = (finger as i64 + n).rem_euclid(len as i64 - 1) as usize;
        data.remove(finger);
        data.insert(dest_index, (idx, n));
        cur += 1;
    }
    finger
}

fn part1_mix_once(input: &mut dyn BufRead) -> String {
    let data = input
        .byte_lines()
        .flatten()
        .map(|line| parse_num_negative(&line) as i64)
        .enumerate()
        .collect::<Vec<_>>();

    let mut sparsevec = SparseVec::from(data);

    do_mix(&mut sparsevec, 0);

    let zero_idx = sparsevec.iter_from(0).position(|(_, n)| *n == 0).unwrap();
    sparsevec
        .iter_from(0)
        .map(|(_, n)| *n)
        .cycle()
        .skip(zero_idx)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i64>()
        .to_string()
}

fn part2_mix_ten_times(input: &mut dyn BufRead) -> String {
    const DECRYPTION_KEY: i64 = 811589153;
    let data = input
        .byte_lines()
        .flatten()
        .map(|line| parse_num_negative(&line) as i64 * DECRYPTION_KEY)
        .enumerate()
        .collect::<Vec<_>>();

    let mut sparsevec = SparseVec::from(data);

    let mut finger = 0;
    for _ in 0..10 {
        finger = do_mix(&mut sparsevec, finger);
    }

    let zero_idx = sparsevec.iter_from(0).position(|(_, n)| *n == 0).unwrap();
    sparsevec
        .iter_from(0)
        .map(|(_, n)| *n)
        .cycle()
        .skip(zero_idx)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i64>()
        .to_string()
}

pub const SOLVERS: &[Solver] = &[part1_mix_once, part2_mix_ten_times];
