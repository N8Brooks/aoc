use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            match line
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .minmax()
            {
                MinMax(min, max) => max - min,
                OneElement(_) => panic!("one element"),
                NoElements => panic!("no elements"),
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .combinations(2)
                .find_map(|ab| {
                    let a = ab.iter().max().unwrap();
                    let b = ab.iter().min().unwrap();
                    if a % b == 0 {
                        Some(a / b)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE_1: &str = "5 1 9 5
7 5 3
2 4 6 8";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2017/testdata/day_02.txt").unwrap();
    }

    #[test_case(EXAMPLE_1, 18)]
    #[test_case(&INPUT, 51833)]
    fn part_1_tests(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    static EXAMPLE_2: &str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test_case(EXAMPLE_2, 9)]
    #[test_case(&INPUT, 288)]
    fn part_2_tests(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
