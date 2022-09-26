use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

pub fn part_1(input: &str) -> u32 {
    min_map(input, u32::abs_diff)
}

pub fn part_2(input: &str) -> u32 {
    min_map(input, arithmetic_diff)
}

fn arithmetic_diff(i: u32, j: u32) -> u32 {
    let diff = i.abs_diff(j);
    diff * (diff + 1) / 2
}

pub fn min_map(input: &str, func: fn(u32, u32) -> u32) -> u32 {
    let mut counts = HashMap::new();
    for i in input.split(',') {
        let i = i.parse::<u32>().unwrap();
        *counts.entry(i).or_insert(0) += 1;
    }
    let (min, max) = match counts.keys().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min, max),
        MinMaxResult::NoElements | MinMaxResult::OneElement(_) => return 0,
    };
    (min..=max)
        .map(|i| counts.iter().map(|(&j, count)| func(i, j) * count).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    static EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    static INPUT: &str = include_str!("testdata/day_07.txt");

    #[test_case(EXAMPLE, 37)]
    #[test_case(INPUT, 345035)]
    fn part_1(input: &str, actual: u32) {
        assert_eq!(super::part_1(input), actual);
    }

    #[test_case(EXAMPLE, 168)]
    #[test_case(INPUT, 97038163)]
    fn part_2(input: &str, actual: u32) {
        assert_eq!(super::part_2(input), actual);
    }
}
