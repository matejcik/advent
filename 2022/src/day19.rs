use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Add, Sub},
};

use bstr::io::BufReadExt;

use crate::{parse_nums, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl Resources {
    pub const fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
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

    pub const fn add_geodes(&self, geodes: i32) -> Self {
        Self {
            geodes: (self.geodes as i32 + geodes) as u32,
            ..*self
        }
    }

    pub const fn can_buy(&self, other: Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geodes >= other.geodes
    }
}

impl Add<Resources> for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
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
            geodes: self.geodes - rhs.geodes,
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
}

impl Blueprint {
    fn load(line: &[u8]) -> Self {
        let mut numbers = [0u64; 1 + 1 + 1 + 2 + 2];
        assert_eq!(parse_nums(line, &mut numbers), numbers.len());
        Self {
            id: numbers[0] as u32,
            ore_cost: Resources::new().add_ore(numbers[1] as i32),
            clay_cost: Resources::new().add_ore(numbers[2] as i32),
            obsidian_cost: Resources::new()
                .add_ore(numbers[3] as i32)
                .add_clay(numbers[4] as i32),
            geodes_cost: Resources::new()
                .add_ore(numbers[5] as i32)
                .add_obsidian(numbers[6] as i32),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SimState {
    time: u32,
    cash: Resources,
    bots: Resources,
}

impl SimState {
    pub const fn new() -> Self {
        Self {
            time: 0,
            cash: Resources::new(),
            bots: Resources::new().add_ore(1),
        }
    }

    pub fn step(&self) -> Self {
        Self {
            time: self.time + 1,
            cash: self.cash + self.bots,
            bots: self.bots,
        }
    }

    pub fn buy_ore_bot(&self, cost: &Resources) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots.add_ore(1),
        }
    }

    pub fn buy_clay_bot(&self, cost: &Resources) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots.add_clay(1),
        }
    }

    pub fn buy_obsidian_bot(&self, cost: &Resources) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots.add_obsidian(1),
        }
    }

    pub fn buy_geodes(&self, cost: &Resources) -> Self {
        Self {
            time: self.time,
            cash: self.cash - *cost,
            bots: self.bots.add_geodes(1),
        }
    }
}

const TIME_LIMIT: u32 = 24;

impl Blueprint {
    pub fn simulate_step(&self, cache: &mut HashMap<SimState, u32>, state: SimState) -> u32 {
        if state.time >= TIME_LIMIT {
            return state.cash.geodes;
        }
        if let Some(&result) = cache.get(&state) {
            return result;
        }
        let result = {
            let step = state.step();
            if step.time >= TIME_LIMIT {
                return step.cash.geodes;
            }
            let do_nothing = self.simulate_step(cache, step);
            let ore_bot = if step.cash.can_buy(self.ore_cost) {
                self.simulate_step(cache, step.buy_ore_bot(&self.ore_cost))
            } else {
                do_nothing
            };
            let clay_bot = if step.cash.can_buy(self.clay_cost) {
                self.simulate_step(cache, step.buy_clay_bot(&self.clay_cost))
            } else {
                do_nothing
            };
            let obsidian_bot = if step.cash.can_buy(self.obsidian_cost) {
                self.simulate_step(cache, step.buy_obsidian_bot(&self.obsidian_cost))
            } else {
                do_nothing
            };
            let geodes_bot = if step.cash.can_buy(self.geodes_cost) {
                self.simulate_step(cache, step.buy_geodes(&self.geodes_cost))
            } else {
                do_nothing
            };
            do_nothing.max(ore_bot.max(clay_bot.max(obsidian_bot.max(geodes_bot))))
        };
        cache.insert(state, result);
        result
    }

    pub fn simulate(&self) -> u32 {
        let mut cache = HashMap::with_capacity(50000);
        self.simulate_step(&mut cache, SimState::new())
    }
}

fn part1_dynamic_like(input: &mut dyn BufRead) -> String {
    input
        .byte_lines()
        .map(|line| {
            let blueprint = Blueprint::load(&line.unwrap());
            blueprint.id * blueprint.simulate()
        })
        .sum::<u32>()
        .to_string()
}

pub const SOLVERS: &[Solver] = &[part1_dynamic_like];
