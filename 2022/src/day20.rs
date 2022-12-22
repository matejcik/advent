use std::{io::BufRead, ops::Index};

use bstr::io::BufReadExt;

use crate::{parse_num, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry {
    index: u16,
    value: i16,
}

struct Mixer {
    chunks: Vec<Vec<Entry>>,
    indices: Vec<u16>,
}

impl Mixer {
    const CHUNK_SIZE: usize = 128;
    const CHUNK_CAPACITY: usize = 128 + 64;

    fn with_capacity(capacity: usize) -> Self {
        let chunks = capacity.div_ceil(Self::CHUNK_SIZE);
        let mut chunk_vec = Vec::with_capacity(chunks);
        for _ in 0..chunks {
            chunk_vec.push(Vec::with_capacity(Self::CHUNK_CAPACITY));
        }
        Self {
            chunks: chunk_vec,
            indices: Vec::with_capacity(capacity),
        }
    }

    fn chunk_and_index(&self, global_index: usize) -> (usize, usize) {
        let mut total = 0;
        for (chunk_idx, chunk) in self.chunks.iter().enumerate() {
            if global_index < total + chunk.len() {
                return (chunk_idx, global_index - total);
            }
            total += chunk.len();
        }
        panic!("index out of bounds");
    }

    fn global_index(&self, chunk: usize, index: usize) -> usize {
        let chunk_offset = self
            .chunks
            .iter()
            .take(chunk)
            .map(|chunk| chunk.len())
            .sum::<usize>();
        chunk_offset + index
    }

    fn rebalance(&mut self) {
        let len = self.len();
        let mut new_vec = Vec::with_capacity(self.chunks.len() * Self::CHUNK_SIZE);
        let mut chunk = Vec::with_capacity(Self::CHUNK_CAPACITY);
        for prev_chunk in self.chunks.iter() {
            let mut prev_slice = prev_chunk.as_slice();
            while prev_slice.len() >= Self::CHUNK_SIZE - chunk.len() {
                let remain = Self::CHUNK_SIZE - chunk.len();
                chunk.extend_from_slice(&prev_slice[..remain]);
                prev_slice = &prev_slice[remain..];
                new_vec.push(chunk);
                chunk = Vec::with_capacity(Self::CHUNK_CAPACITY);
            }
            chunk.extend_from_slice(prev_slice);
        }
        if chunk.len() > 0 {
            new_vec.push(chunk);
        }
        for c in 0..new_vec.len() {
            for i in 0..new_vec[c].len() {
                let entry = new_vec[c][i];
                self.indices[entry.index as usize] = c as u16;
            }
        }
        self.chunks = new_vec;
        assert_eq!(len, self.len());
    }

    pub fn mix_item(&mut self, item: usize, encryption_key: i64) {
        let src_chunk_idx = self.indices[item] as usize;
        let chunk = &self.chunks[src_chunk_idx];
        let src_index = chunk
            .iter()
            .position(|entry| entry.index == item as u16)
            .unwrap();
        let src_global_index = self.global_index(src_chunk_idx, src_index);

        let entry = chunk[src_index];

        let value = entry.value as i64 * encryption_key;
        // calculate shift
        let dest_global_index =
            (src_global_index as i64 + value).rem_euclid(self.len() as i64 - 1) as usize;
        if dest_global_index == src_global_index {
            // shift is zero, do nothing
            return;
        }
        // drop item from source chunk
        self.chunks[src_chunk_idx].remove(src_index);
        // calculate destination chunk and index
        // (doing this after item is dropped because it affects the global index)
        let (mut dest_chunk, mut dest_i) = self.chunk_and_index(dest_global_index);
        // update the chunk index in indices
        if dest_chunk > 0 && dest_i == 0 {
            // prefer pushing back into a previous chunk instead of inserting front of next chunk
            dest_chunk -= 1;
            dest_i = self.chunks[dest_chunk].len();
        }
        self.indices[item] = dest_chunk as u16;
        // insert item into destination chunk
        self.chunks[dest_chunk].insert(dest_i, entry);
        if self.chunks[dest_chunk].len() >= self.chunks[dest_chunk].capacity()
            || self.chunks[src_chunk_idx].len() == 0
        {
            // rebalance if the destination chunk is full or the source chunk is empty
            self.rebalance();
        }
    }

    pub fn mix(&mut self, encryption_key: i64) {
        for i in 0..self.len() {
            self.mix_item(i, encryption_key);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entry> + Clone {
        self.chunks.iter().flatten()
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.len()).sum()
    }
}

impl Index<usize> for Mixer {
    type Output = Entry;

    fn index(&self, idx: usize) -> &Self::Output {
        let mut total = 0;
        for chunk in self.chunks.iter() {
            if idx < total + chunk.len() {
                return &chunk[idx - total];
            }
            total += chunk.len();
        }
        panic!("index out of bounds");
    }
}

impl From<Vec<i16>> for Mixer {
    fn from(vec: Vec<i16>) -> Self {
        let mut sparse_vec = Self::with_capacity(vec.len());
        for (i, chunk) in sparse_vec.chunks.iter_mut().enumerate() {
            let start = i * Self::CHUNK_SIZE;
            let end = vec.len().min((i + 1) * Self::CHUNK_SIZE);
            chunk.extend(vec[start..end].iter().enumerate().map(|(ii, n)| Entry {
                index: (start + ii) as u16,
                value: *n,
            }));
        }
        sparse_vec.indices = (0..vec.len())
            .map(|n| (n / Self::CHUNK_SIZE) as u16)
            .collect();
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

fn part1_mix_once(input: &mut dyn BufRead) -> String {
    let data = input
        .byte_lines()
        .flatten()
        .map(|line| parse_num_negative(&line) as i16)
        .collect::<Vec<_>>();

    let mut sparsevec = Mixer::from(data);
    sparsevec.mix(1);

    let zero_idx = sparsevec.iter().position(|e| e.value == 0).unwrap();
    sparsevec
        .iter()
        .map(|e| e.value as i64)
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
        .map(|line| parse_num_negative(&line) as i16)
        .collect::<Vec<_>>();

    let mut sparsevec = Mixer::from(data);
    for _ in 0..10 {
        sparsevec.mix(DECRYPTION_KEY);
    }
    let zero_idx = sparsevec.iter().position(|e| e.value == 0).unwrap();
    sparsevec
        .iter()
        .map(|e| e.value as i64 * DECRYPTION_KEY)
        .cycle()
        .skip(zero_idx)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i64>()
        .to_string()
}

pub const SOLVERS: &[Solver] = &[part1_mix_once, part2_mix_ten_times];
