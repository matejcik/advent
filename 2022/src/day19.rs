use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Add, Deref, Sub},
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
    time: u32,
    cash: Resources,
    bots: Resources,
    geodes: u32,
}

impl SimState {
    pub const fn new() -> Self {
        Self {
            time: 0,
            cash: Resources::new(),
            bots: Resources::new().add_ore(1),
            geodes: 0,
        }
    }

    pub fn step(&self) -> Self {
        Self {
            time: self.time + 1,
            cash: self.cash + self.bots,
            bots: self.bots,
            geodes: self.geodes,
        }
    }

    pub fn buy_ore_bot(&self, cost: &Resources) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots.add_ore(1),
            geodes: self.geodes,
        }
    }

    pub fn buy_clay_bot(&self, cost: &Resources) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots.add_clay(1),
            geodes: self.geodes,
        }
    }

    pub fn buy_obsidian_bot(&self, cost: &Resources) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots.add_obsidian(1),
            geodes: self.geodes,
        }
    }

    pub fn buy_geodes(&self, cost: &Resources, remaining: u32) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots,
            geodes: self.geodes + remaining,
        }
    }
}

const TRIANGULAR_NUMBERS: [u32; 35] = [
    0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231,
    253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595,
];

struct Simulation {
    time_limit: u32,
    best_result: u32,
    blueprint: Blueprint,
    cache: HashMap<SimState, u32>,
}

impl Simulation {
    pub fn simulate_step(&mut self, state: SimState) -> u32 {
        assert!(state.time < self.time_limit);
        if let Some(&result) = self.cache.get(&state) {
            return result;
        }
        let result = {
            let step = state.step();
            if step.time >= self.time_limit {
                return step.geodes;
            }
            let remaining = self.time_limit - step.time;
            if step.geodes + TRIANGULAR_NUMBERS[remaining as usize] <= self.best_result {
                return 0;
            }
            let do_nothing = self.simulate_step(step);
            self.best_result = self.best_result.max(do_nothing);
            if state.cash.can_buy(&self.blueprint.geodes_cost) {
                let sim =
                    self.simulate_step(step.buy_geodes(&self.blueprint.geodes_cost, remaining));
                self.best_result = self.best_result.max(sim);
            };
            if state.cash.can_buy(&self.blueprint.obsidian_cost) {
                let sim = self.simulate_step(step.buy_obsidian_bot(&self.blueprint.obsidian_cost));
                self.best_result = self.best_result.max(sim);
            };
            if state.cash.can_buy(&self.blueprint.clay_cost) {
                let sim = self.simulate_step(step.buy_clay_bot(&self.blueprint.clay_cost));
                self.best_result = self.best_result.max(sim);
            }
            if state.bots.ore < self.blueprint.max_ore_required
                && state.cash.can_buy(&self.blueprint.ore_cost)
            {
                let sim = self.simulate_step(step.buy_ore_bot(&self.blueprint.ore_cost));
                self.best_result = self.best_result.max(sim);
            };
            self.best_result
        };
        self.cache.insert(state, result);
        result
    }

    pub fn simulate(blueprint: Blueprint, time_limit: u32) -> u32 {
        let mut sim = Self {
            time_limit,
            best_result: 0,
            blueprint,
            cache: HashMap::with_capacity(50000),
        };
        sim.simulate_step(SimState::new())
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
