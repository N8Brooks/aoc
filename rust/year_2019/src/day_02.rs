use std::iter::empty;

use itertools::Itertools as _;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> isize {
    const NOUN: isize = 12;
    const VERB: isize = 2;
    let program = parse_program(input);
    intcode(program, NOUN, VERB)
}

pub fn part_2(input: &str) -> isize {
    const TARGET: isize = 19690720;
    let program = parse_program(input);
    let (noun, verb) = (0..=99)
        .cartesian_product(0..=99)
        .find(|&(noun, verb)| intcode(program.clone(), noun, verb) == TARGET)
        .unwrap();
    100 * noun + verb
}

fn intcode(mut program: Vec<isize>, noun: isize, verb: isize) -> isize {
    program[1] = noun;
    program[2] = verb;
    let mut computer = empty().intcode(program);
    assert!(computer.next().is_none());
    computer.memory[0]
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    // #[test_case("1,9,10,3,2,3,11,0,99,30,40,50" => vec![3500,9,10,70,2,3,11,0,99,30,40,50])]
    // #[test_case("1,0,0,0,99" => vec![2,0,0,0,99])]
    // #[test_case("2,3,0,3,99" => vec![2,3,0,6,99])]
    // #[test_case("2,4,4,5,99,0" => vec![2,4,4,5,99,9801])]
    // #[test_case("1,1,1,4,99,5,6,0,99" => vec![30,1,1,4,2,5,6,0,99])]
    // fn intcode(input: &str) -> Vec<usize> {
    //     let mut intcode = super::Intcode::new(input);
    //     intcode.run();
    //     intcode.memory
    // }

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    #[test_case(INPUT => 3760627)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(INPUT => 7195)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
