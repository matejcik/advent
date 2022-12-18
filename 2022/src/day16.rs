use std::{cmp::Reverse, io::BufRead};

use bstr::io::BufReadExt;

use crate::{bitset::BitSet, parse_num, tiles::Tiles, Solver};

const MAX_NODES: usize = 60;

struct Graph {
    start_idx: usize,
    rates: Vec<u32>,
    neighbors: Tiles<u32>,
    have_flow: Vec<usize>,
}

impl Graph {
    fn load(lines: impl Iterator<Item = Vec<u8>>) -> Self {
        let mut names = Vec::with_capacity(MAX_NODES);
        let mut rates = Vec::with_capacity(MAX_NODES);
        let mut edges = Vec::with_capacity(MAX_NODES);
        let mut start_idx = 0;
        for line in lines {
            let pieces = line.split(|c| *c == b' ');
            let mut pieces = pieces.skip(1); // Valve
            let name: [u8; 2] = pieces.next().unwrap().try_into().unwrap();
            names.push(name);
            if &name == b"AA" {
                start_idx = names.len() - 1;
            }

            let mut pieces = pieces.skip(2); // has flow
            let rate = pieces.next().unwrap();
            rates.push(parse_num(&rate[b"rate=".len()..]) as u32);

            let pieces = pieces.skip(4); // tunnel(s) lead(s) to valve(s)
            edges.push(
                pieces
                    .map(|p| TryInto::<[u8; 2]>::try_into(&p[..2]).unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        let mut tiles = Tiles::new(names.len(), names.len(), u32::MAX);
        for (idx, edge) in edges.into_iter().enumerate() {
            for dest in edge {
                let dest = names.iter().position(|name| name == &dest).unwrap();
                tiles[(idx, dest)] = 1;
                tiles[(dest, idx)] = 1;
            }
        }

        let mut have_flow = (0..rates.len())
            .filter(|&idx| rates[idx] > 0)
            .collect::<Vec<usize>>();
        have_flow.sort_by_key(|&idx| Reverse(rates[idx]));

        let mut res = Self {
            start_idx,
            rates,
            have_flow,
            neighbors: tiles,
        };
        res.floyd_warshall();
        res
    }

    fn floyd_warshall(&mut self) {
        let tiles = &mut self.neighbors;
        for x in 0..tiles.width() {
            tiles[(x, x)] = 0;
        }
        for k in 0..tiles.width() {
            for i in 0..tiles.width() {
                for j in 0..tiles.width() {
                    tiles[(i, j)] = tiles[(i, j)].min(tiles[(i, k)].saturating_add(tiles[(k, j)]));
                }
            }
        }
    }

    fn flow_upper_bound_for(&self, closed: BitSet<u64>, remaining: u32) -> u32 {
        let mut res = 0;
        for &idx in &self.have_flow {
            if closed.contains(idx) {
                res += self.rates[idx];
            }
        }
        res * remaining
    }

    fn best_path(&self, limit: u32) -> u32 {
        let closed = BitSet::new_full_up_to(self.rates.len() - 1);
        self.best_path_rec(self.start_idx, closed, limit, 0, 0)
    }

    fn best_path_rec(
        &self,
        start: usize,
        closed: BitSet<u64>,
        remaining: u32,
        current_flow: u32,
        best_flow: u32,
    ) -> u32 {
        let mut best_flow = best_flow.max(current_flow);
        if remaining == 0 {
            return current_flow;
        }
        if current_flow + self.flow_upper_bound_for(closed, remaining) < best_flow {
            return 0; // no point in continuing as we can't improve best flow
        }
        for next in self.have_flow.iter().copied().filter(|&idx| closed.contains(idx)) {
            let time = self.neighbors[(start, next)] + 1;
            if remaining <= time {
                continue;
            }
            let new_closed = closed.with_removed(next);
            let new_remaining = remaining - time;
            let new_flow = current_flow + self.rates[next] * new_remaining;
            best_flow = best_flow.max(self.best_path_rec(
                next,
                new_closed,
                new_remaining,
                new_flow,
                best_flow,
            ));
        }
        best_flow
    }
}

fn part1_best_flow_in_30_minutes(input: &mut dyn BufRead) -> String {
    let graph = Graph::load(input.byte_lines().flatten());
    graph.best_path(30).to_string()
}

pub const SOLVERS: &[Solver] = &[part1_best_flow_in_30_minutes];
