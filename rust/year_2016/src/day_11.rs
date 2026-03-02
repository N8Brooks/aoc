use std::{
    array,
    iter::{empty, once},
};

use hashbrown::HashSet;
use itertools::{Either::*, Itertools};

pub fn part_1(input: &str) -> usize {
    solve(input, false)
}

pub fn part_2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, part_2: bool) -> usize {
    let mut steps = 0;
    let mut seen = HashSet::new();
    let (state_0, pair_count) = parse_input(input, part_2);
    let mask = (1 << pair_count) - 1;
    let mut frontier = vec![(0, state_0)];
    seen.insert((0, canonicalize(&state_0, pair_count)));
    while !frontier.is_empty() {
        let mut next_frontier = Vec::new();
        for (floor_1, state) in frontier {
            if state == [[0; 2], [0; 2], [0; 2], [mask; 2]] {
                return steps;
            }

            let mut items = [(0usize, 0u8); 16];
            let mut item_count = 0;
            for (part, mut names) in state[floor_1].into_iter().enumerate() {
                while names > 0 {
                    let bit = names & names.wrapping_neg();
                    names &= names - 1;
                    items[item_count] = (part, bit);
                    item_count += 1;
                }
            }

            let can_move_down = floor_1 > 0
                && state[..floor_1]
                    .iter()
                    .any(|&[gens, chips]| gens | chips != 0);
            let directions = [
                (floor_1 < 3).then(|| floor_1 + 1),
                can_move_down.then(|| floor_1 - 1),
            ];

            directions
                .into_iter()
                .flatten()
                .flat_map(|floor_2| {
                    items
                        .into_iter()
                        .enumerate()
                        .take(item_count)
                        .flat_map(move |(i, item)| {
                            items
                                .into_iter()
                                .take(item_count)
                                .skip(i + 1)
                                .map(move |item_2| {
                                    map_state(state, floor_1, floor_2, &[item, item_2])
                                })
                                .chain(once(map_state(state, floor_1, floor_2, &[item])))
                        })
                        .map(move |state| (floor_2, state))
                })
                .filter(|(_, state)| {
                    state
                        .iter()
                        .all(|&[gens, chips]| gens == 0 || chips & !gens == 0)
                })
                .filter(|(floor_2, state)| seen.insert((*floor_2, canonicalize(state, pair_count))))
                .collect_into(&mut next_frontier);
        }
        frontier = next_frontier;
        steps += 1;
    }
    panic!("no solution found")
}

fn map_state(
    mut state: [[u8; 2]; 4],
    floor_1: usize,
    floor_2: usize,
    items: &[(usize, u8)],
) -> [[u8; 2]; 4] {
    for &(part, name) in items {
        state[floor_1][part] ^= name;
        state[floor_2][part] ^= name;
    }
    state
}

fn parse_input<'a>(input: &'a str, part_2: bool) -> ([[u8; 2]; 4], usize) {
    let mut names = Vec::with_capacity(8);
    let mut index = |name: &'a str| {
        names.iter().position(|&n| n == name).unwrap_or_else(|| {
            let i = names.len();
            names.push(name);
            i
        })
    };
    (
        input
            .lines()
            .zip(["first", "second", "third", "fourth"])
            .map(|(line, floor)| {
                let line = line
                    .strip_circumfix(&format!("The {floor} floor contains "), '.')
                    .unwrap();
                let it = if let Some((head, last)) = line.rsplit_once(", and ") {
                    // 3+
                    Right(Right(head.split(", ").chain(once(last))))
                } else if let Some((first, second)) = line.split_once(" and ") {
                    // 2
                    Right(Left([first, second].into_iter()))
                } else if line != "nothing relevant" {
                    // 1
                    Left(Right(once(line)))
                } else {
                    // 0
                    Left(Left(empty()))
                };
                let extra = (part_2 && floor == "first")
                    .then_some([
                        "an elerium generator",
                        "an elerium-compatible microchip",
                        "a dilithium generator",
                        "a dilithium-compatible microchip",
                    ])
                    .into_iter()
                    .flatten();
                it.chain(extra)
                    .fold([0, 0], |[generators, microchips], part| {
                        let part = part
                            .strip_prefix("a ")
                            .or_else(|| part.strip_prefix("an "))
                            .unwrap();
                        if let Some(name) = part.strip_suffix(" generator") {
                            [generators | (1 << index(name)), microchips]
                        } else if let Some(name) = part.strip_suffix("-compatible microchip") {
                            [generators, microchips | (1 << index(name))]
                        } else {
                            panic!("unexpected part description: {part}");
                        }
                    })
            })
            .collect_array()
            .unwrap(),
        names.len(),
    )
}

fn canonicalize(state: &[[u8; 2]; 4], pair_count: usize) -> [u8; 8] {
    let mut pairs = array::from_fn(|idx| {
        if idx >= pair_count {
            return 0;
        }
        let bit = 1 << idx;
        let gen_floor = state.iter().position(|[gens, _]| gens & bit != 0).unwrap() as u8;
        let chip_floor = state
            .iter()
            .position(|[_, chips]| chips & bit != 0)
            .unwrap() as u8;
        (gen_floor << 2) | chip_floor
    });
    pairs[..pair_count].sort_unstable();
    pairs
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    const EXAMPLE: &str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test_case(EXAMPLE => 11)]
    #[test_case(INPUT => 31)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 55)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
