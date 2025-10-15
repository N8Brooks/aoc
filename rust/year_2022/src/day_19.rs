use hashbrown::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use ndarray::Array1;
use ndarray::{prelude::*, Zip};
use regex::Regex;

// This is really slow

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Blueprint \d+: Each ore robot costs (?P<ore_robot_ore_cost>\d+) ore. Each clay robot costs (?P<clay_robot_ore_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_robot_ore_cost>\d+) ore and (?P<obsidian_robot_clay_cost>\d+) clay. Each geode robot costs (?P<geode_robot_ore_cost>\d+) ore and (?P<geode_robot_obsidian_cost>\d+) obsidian.$").unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    resources: Array1<usize>,
    robots: Array1<usize>,
}

impl Default for State {
    fn default() -> Self {
        State {
            resources: array![0, 0, 0, 0],
            robots: array![1, 0, 0, 0],
        }
    }
}

#[derive(Debug)]
struct StateSpace(Array2<usize>);

impl From<&str> for StateSpace {
    fn from(line: &str) -> Self {
        let cap = RE.captures(line).unwrap();
        StateSpace(array![
            [cap["ore_robot_ore_cost"].parse().unwrap(), 0, 0, 0,],
            [cap["clay_robot_ore_cost"].parse().unwrap(), 0, 0, 0],
            [
                cap["obsidian_robot_ore_cost"].parse().unwrap(),
                cap["obsidian_robot_clay_cost"].parse().unwrap(),
                0,
                0
            ],
            [
                cap["geode_robot_ore_cost"].parse().unwrap(),
                0,
                cap["geode_robot_obsidian_cost"].parse().unwrap(),
                0,
            ]
        ])
    }
}

impl StateSpace {
    fn moves(&self, state: &State) -> Vec<State> {
        self.0
            .outer_iter()
            .enumerate()
            .flat_map(|(robot, resources)| {
                Zip::from(&state.resources)
                    .and(resources)
                    .all(|a, b| a >= b)
                    .then(|| {
                        let mut new_state = state.clone();
                        new_state.resources -= &resources;
                        new_state.robots[robot] += 1;
                        new_state
                    })
            })
            .chain(Some(state.clone()))
            .map(|mut new_state| {
                new_state.resources += &state.robots;
                new_state
            })
            .collect()
    }

    fn max_geodes(&self, n_minutes: usize) -> usize {
        // Inlcudes optimizations from /u/jonathan_paulson
        let max_ore_robots = *self.0.slice(s![.., 0]).iter().max().unwrap();
        let max_clay_robots = *self.0.slice(s![.., 1]).iter().max().unwrap();
        let max_obsidian_robots = *self.0.slice(s![.., 2]).iter().max().unwrap();
        (0..n_minutes)
            .fold(HashSet::from([State::default()]), |states, i| {
                let turns_remaining = n_minutes - i - 1;
                let max_ore = max_ore_robots * turns_remaining;
                let max_clay = max_clay_robots * turns_remaining;
                let max_obsidian = max_obsidian_robots * turns_remaining;
                states
                    .iter()
                    .flat_map(|state| self.moves(state))
                    .map(|mut state| {
                        state.resources[0] = state.resources[0].min(max_ore);
                        state.resources[1] = state.resources[1].min(max_clay);
                        state.resources[2] = state.resources[2].min(max_obsidian);
                        state.robots[0] = state.robots[0].min(max_ore_robots);
                        state.robots[1] = state.robots[1].min(max_clay_robots);
                        state.robots[2] = state.robots[2].min(max_obsidian_robots);
                        state
                    })
                    .sorted_by_cached_key(|state| self.fitness(state, turns_remaining))
                    .take(1_000_000)
                    .collect::<HashSet<State>>()
            })
            .into_iter()
            .map(|state| state.resources[3])
            .max()
            .unwrap()
    }

    fn fitness(&self, state: &State, turns_remaining: usize) -> usize {
        usize::MAX - (state.resources[3] + (turns_remaining + 1) * state.robots[3])
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let state_space = StateSpace::from(line);
            let max_geodes = state_space.max_geodes(24);
            let blueprint_id = i + 1;
            blueprint_id * max_geodes
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .take(3)
        .map(|line| StateSpace::from(line).max_geodes(32))
        .product()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. \
Each clay robot costs 2 ore. \
Each obsidian robot costs 3 ore and 14 clay. \
Each geode robot costs 2 ore and 7 obsidian.
\
Blueprint 2: Each ore robot costs 2 ore. \
Each clay robot costs 3 ore. \
Each obsidian robot costs 3 ore and 8 clay. \
Each geode robot costs 3 ore and 12 obsidian.";

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_19.txt");

    #[test_case(EXAMPLE, 33)]
    #[test_case(INPUT, 1081)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 3472)]
    #[test_case(INPUT, 2415)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
