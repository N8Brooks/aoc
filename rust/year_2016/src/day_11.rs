use std::iter::{self, empty, once};

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
    let (state_0, mask) = parse_input(input, part_2);
    let mut frontier = vec![(0, state_0)];
    while !frontier.is_empty() {
        let mut next_frontier = Vec::new();
        for (floor_1, state) in frontier {
            if state == [[0; 2], [0; 2], [0; 2], [mask; 2]] {
                return steps;
            }

            let one = state[floor_1]
                .into_iter()
                .enumerate()
                .flat_map(|(part, mut names)| {
                    iter::from_fn(move || {
                        (names > 0).then(|| {
                            let bit = names & names.wrapping_neg();
                            names &= names - 1;
                            (part, bit)
                        })
                    })
                });
            let two = one.clone().combinations(2);

            one.map(|x| vec![x])
                .chain(two)
                .flat_map(|parts| {
                    [floor_1.checked_sub(1), (floor_1 < 3).then(|| floor_1 + 1)]
                        .into_iter()
                        .flatten()
                        .map(move |floor_2| {
                            let mut state = state;
                            for (part, name) in &parts {
                                state[floor_1][*part] ^= *name;
                                state[floor_2][*part] ^= *name;
                            }
                            (floor_2, state)
                        })
                })
                .filter(|(_, state)| {
                    state
                        .iter()
                        .all(|&[gens, chips]| gens == 0 || chips & !gens == 0)
                })
                .filter(|state| seen.insert(*state))
                .collect_into(&mut next_frontier);
        }
        frontier = next_frontier;
        steps += 1;
    }
    panic!("no solution found")
}

fn parse_input<'a>(input: &'a str, part_2: bool) -> ([[u8; 2]; 4], u8) {
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
        1u8.checked_shl(names.len().try_into().unwrap())
            .unwrap_or(0)
            .wrapping_sub(1),
    )
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
