use std::cell::LazyCell;

use regex::Regex;

pub fn part_1(input: &str) -> String {
    let (mut stacks, procedure) = parse_input(input);
    for (count, i, j) in procedure {
        for _ in 0..count {
            let value = stacks[i].pop().unwrap();
            stacks[j].push(value);
        }
    }
    String::from_utf8(stacks.iter().map(|stack| *stack.last().unwrap()).collect()).unwrap()
}

pub fn part_2(input: &str) -> String {
    let (mut stacks, procedure) = parse_input(input);
    for (count, i, j) in procedure {
        let r = stacks[i].len() - count..;
        let crates: Vec<_> = stacks[i].drain(r).collect();
        stacks[j].extend(crates);
    }
    String::from_utf8(stacks.iter().map(|stack| *stack.last().unwrap()).collect()).unwrap()
}

type Stack = Vec<u8>;
type Stacks = Vec<Stack>;
type Move = (usize, usize, usize);
type Procedure = Vec<Move>;

fn parse_input(input: &str) -> (Stacks, Procedure) {
    let re = LazyCell::new(|| Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap());
    let (stacks_input, procedure) = input.split_once("\n\n").unwrap();
    let line_len = input.find('\n').unwrap();
    let stacks_len = line_len / 4 + 1;
    let mut stacks = vec![Vec::new(); stacks_len];
    for line in stacks_input.lines().rev().skip(1) {
        let line = line.as_bytes();
        for i in 0..stacks_len {
            let byte = line[1 + i * 4];
            if byte != b' ' {
                stacks[i].push(byte);
            }
        }
    }
    let procedure = re
        .captures_iter(procedure)
        .map(|cap| {
            let count = cap[1].parse().unwrap();
            let i = cap[2].parse::<usize>().unwrap() - 1;
            let j = cap[3].parse::<usize>().unwrap() - 1;
            (count, i, j)
        })
        .collect();
    (stacks, procedure)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    #[test_case(EXAMPLE => "CMZ")]
    #[test_case(INPUT => "TDCHVHJTG")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => "MCD")]
    #[test_case(INPUT => "NGCMPJLHV")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
