use std::ops::{BitAnd, BitOr};

pub fn part_1(input: &str) -> usize {
    sum_answers(input, BitOr::bitor)
}

pub fn part_2(input: &str) -> usize {
    sum_answers(input, BitAnd::bitand)
}

fn sum_answers(input: &str, mut op: impl FnMut(u32, u32) -> u32) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.bytes().fold(0, |acc, b| acc | (1 << (b - b'a'))))
                .reduce(&mut op)
                .unwrap_or(0)
                .count_ones() as usize
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    const EXAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test_case(EXAMPLE => 11)]
    #[test_case(INPUT => 6532)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 6)]
    #[test_case(INPUT => 3427)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
