use std::iter::once;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> isize {
    const ID: isize = 1;
    intcode(input, ID)
}

pub fn part_2(input: &str) -> isize {
    const ID: isize = 5;
    intcode(input, ID)
}

fn intcode(input: &str, id: isize) -> isize {
    let program = parse_program(input);
    once(id).intcode(program).last().unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    #[test_case(INPUT => 9006673)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(INPUT => 3629692)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
