use core::time;
use std::{
    collections::HashSet,
    io::BufRead,
    ops::{Add, Deref, Mul, Sub},
};

use bstr::io::BufReadExt;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{parse_nums, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

impl Resources {
    pub const fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
        }
    }

    pub const fn add_ore(&self, ore: i32) -> Self {
        Self {
            ore: (self.ore as i32 + ore) as u32,
            ..*self
        }
    }

    pub const fn add_clay(&self, clay: i32) -> Self {
        Self {
            clay: (self.clay as i32 + clay) as u32,
            ..*self
        }
    }

    pub const fn add_obsidian(&self, obsidian: i32) -> Self {
        Self {
            obsidian: (self.obsidian as i32 + obsidian) as u32,
            ..*self
        }
    }

    pub const fn can_buy(&self, other: &Self) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }
}

impl Add<Resources> for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

impl Sub<Resources> for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
        }
    }
}

impl Mul<u32> for Resources {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    id: u32,
    ore_cost: Resources,
    clay_cost: Resources,
    obsidian_cost: Resources,
    geodes_cost: Resources,
    max_ore_required: u32,
}

impl Blueprint {
    fn load(line: impl Deref<Target = [u8]>) -> Self {
        let mut numbers = [0u64; 1 + 1 + 1 + 2 + 2];
        assert_eq!(parse_nums(&line, &mut numbers), numbers.len());
        let ore_cost = Resources::new().add_ore(numbers[1] as i32);
        let clay_cost = Resources::new().add_ore(numbers[2] as i32);
        let obsidian_cost = Resources::new()
            .add_ore(numbers[3] as i32)
            .add_clay(numbers[4] as i32);
        let geodes_cost = Resources::new()
            .add_ore(numbers[5] as i32)
            .add_obsidian(numbers[6] as i32);
        Self {
            id: numbers[0] as u32,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geodes_cost,
            max_ore_required: clay_cost.ore.max(obsidian_cost.ore).max(geodes_cost.ore),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SimState {
    time_remaining: u32,
    cash: Resources,
    bots: Resources,
    geodes: u32,
}

impl SimState {
    pub const fn new(time_remaining: u32) -> Self {
        Self {
            time_remaining,
            cash: Resources::new(),
            bots: Resources::new().add_ore(1),
            geodes: 0,
        }
    }

    pub fn steps(&self, n: u32) -> Self {
        assert!(n <= self.time_remaining);
        Self {
            time_remaining: self.time_remaining - n,
            cash: self.cash + self.bots * n,
            bots: self.bots,
            geodes: self.geodes,
        }
    }

    fn div_round_up(a: u32, b: u32) -> u32 {
        (a + b - 1) / b
    }

    pub fn can_afford_when(&self, cost: &Resources) -> Option<u32> {
        if self.time_remaining <= 1 {
            return None;
        }
        if self.cash.can_buy(cost) {
            return Some(1);
        }
        let ore_units = cost.ore.saturating_sub(self.cash.ore);
        let clay_units = cost.clay.saturating_sub(self.cash.clay);
        let obsidian_units = cost.obsidian.saturating_sub(self.cash.obsidian);
        let mut max_time = Self::div_round_up(ore_units, self.bots.ore);
        if clay_units > 0 {
            if self.bots.clay == 0 {
                return None;
            } else {
                max_time = max_time.max(Self::div_round_up(clay_units, self.bots.clay));
            }
        }
        if obsidian_units > 0 {
            if self.bots.obsidian == 0 {
                return None;
            } else {
                max_time = max_time.max(Self::div_round_up(obsidian_units, self.bots.obsidian));
            }
        }
        if max_time >= self.time_remaining - 1 {
            None
        } else {
            Some(max_time + 1)
        }
    }

    pub fn buy_ore_bot(&self, cost: &Resources) -> Self {
        Self {
            time_remaining: self.time_remaining,
            cash: self.cash - *cost,
            bots: self.bots.add_ore(1),
            geodes: self.geodes,
        }
    }

    pub fn buy_clay_bot(&self, cost: &Resources) -> Self {
        Self {
            time_remaining: self.time_remaining,
            cash: self.cash - *cost,
            bots: self.bots.add_clay(1),
            geodes: self.geodes,
        }
    }

    pub fn buy_obsidian_bot(&self, cost: &Resources) -> Self {
        Self {
            time_remaining: self.time_remaining,
            cash: self.cash - *cost,
            bots: self.bots.add_obsidian(1),
            geodes: self.geodes,
        }
    }

    pub fn buy_geodes(&self, cost: &Resources) -> Self {
        Self {
            time_remaining: self.time_remaining,
            cash: self.cash - *cost,
            bots: self.bots,
            geodes: self.geodes + self.time_remaining,
        }
    }
}

const TRIANGULAR_NUMBERS: [u32; 35] = [
    0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231,
    253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595,
];

struct Simulation {
    best_result: u32,
    blueprint: Blueprint,
}

impl Simulation {
    pub fn simulate_step(&mut self, state: SimState) {
        assert!(state.time_remaining > 0);
        if state.geodes + TRIANGULAR_NUMBERS[state.time_remaining as usize] <= self.best_result {
            return;
        }
        if let Some(when) = state.can_afford_when(&self.blueprint.geodes_cost) {
            self.simulate_step(state.steps(when).buy_geodes(&self.blueprint.geodes_cost));
        }
        if let Some(when) = state.can_afford_when(&self.blueprint.obsidian_cost) {
            self.simulate_step(
                state
                    .steps(when)
                    .buy_obsidian_bot(&self.blueprint.obsidian_cost),
            );
        }
        if let Some(when) = state.can_afford_when(&self.blueprint.clay_cost) {
            self.simulate_step(state.steps(when).buy_clay_bot(&self.blueprint.clay_cost));
        }
        if state.bots.ore < self.blueprint.max_ore_required {
            if let Some(when) = state.can_afford_when(&self.blueprint.ore_cost) {
                self.simulate_step(state.steps(when).buy_ore_bot(&self.blueprint.ore_cost));
            }
        }
        self.best_result = self.best_result.max(state.geodes);
    }

    pub fn simulate(blueprint: Blueprint, time_limit: u32) -> u32 {
        let mut sim = Self {
            best_result: 0,
            blueprint,
        };
        sim.simulate_step(SimState::new(time_limit));
        sim.best_result
    }
}

fn part1_dynamic_like(input: &mut dyn BufRead) -> String {
    const TIME_LIMIT: u32 = 24;
    let blueprints = input
        .byte_lines()
        .flatten()
        .map(Blueprint::load)
        .collect::<Vec<_>>();

    blueprints
        .par_iter()
        .map(|&blueprint| blueprint.id * Simulation::simulate(blueprint, TIME_LIMIT))
        .sum::<u32>()
        .to_string()
}

fn part2_more_steps_less_elephants(input: &mut dyn BufRead) -> String {
    const TIME_LIMIT: u32 = 32;
    let blueprints = input
        .byte_lines()
        .take(3)
        .flatten()
        .map(Blueprint::load)
        .collect::<Vec<_>>();

    blueprints
        .par_iter()
        .map(|&blueprint| Simulation::simulate(blueprint, TIME_LIMIT))
        .product::<u32>()
        .to_string()
}

pub const SOLVERS: &[Solver] = &[part1_dynamic_like, part2_more_steps_less_elephants];
