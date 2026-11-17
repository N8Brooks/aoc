use std::{iter::successors, str::FromStr};

use hashbrown::HashMap;

use crate::day_14::Instruction::*;

pub fn part_1(input: &str) -> u64 {
    let (mut xs, mut os) = (0, 0);
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .filter_map(|instruction| match instruction {
            Mask(xs2, os2) => {
                (xs, os) = (xs2, os2);
                None
            }
            Mem(addr, value) => Some((addr, value & xs | os)),
        })
        .collect::<HashMap<_, _>>()
        .into_values()
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let (mut xs, mut os) = (0, 0);
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .filter_map(|instruction| match instruction {
            Mask(xs2, os2) => {
                (xs, os) = (xs2, os2);
                None
            }
            Mem(addr, value) => Some(
                successors(Some(xs), move |bits| bits.checked_sub(1).map(|b| b & xs))
                    .map(move |bits| ((addr | os) & !xs | bits, value)),
            ),
        })
        .flatten()
        .collect::<HashMap<_, _>>()
        .into_values()
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mask(u64, u64),
    Mem(u64, u64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (location, value) = s.split_once(" = ").unwrap();

        Ok(if location == "mask" {
            let (xs, os) = value.bytes().fold((0u64, 0u64), |(xs, os), b| {
                (xs << 1 | (b == b'X') as u64, os << 1 | (b == b'1') as u64)
            });

            Mask(xs, os)
        } else {
            let addr = location
                .strip_prefix("mem[")
                .and_then(|s| s.strip_suffix(']'))
                .unwrap()
                .parse()
                .unwrap();

            Mem(addr, value.parse().unwrap())
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    const EXAMPLE_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test_case(EXAMPLE_1 => 165)]
    #[test_case(INPUT => 14722016054794)]
    fn part_1(input: &str) -> u64 {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test_case(EXAMPLE_2 => 208)]
    #[test_case(INPUT => 3618217244644)]
    fn part_2(input: &str) -> u64 {
        super::part_2(input)
    }
}
