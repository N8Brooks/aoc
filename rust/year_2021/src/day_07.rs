use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

pub fn part_1(input: &str) -> isize {
    let mut counts: HashMap<isize, isize> = HashMap::new();
    for x in input.split(',') {
        let x = x.parse().unwrap();
        *counts.entry(x).or_default() += 1;
    }
    counts
        .keys()
        .map(|x_0| {
            counts
                .iter()
                .map(|(x_1, count)| (x_1 - x_0).abs() * count)
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> isize {
    let mut counts: HashMap<isize, isize> = HashMap::new();
    for x in input.split(',') {
        let x = x.parse().unwrap();
        *counts.entry(x).or_default() += 1;
    }
    let (min, max) = match counts.keys().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min, max),
        _ => (0, 0),
    };
    (min..max + 1)
        .map(|x_0| {
            counts
                .iter()
                .map(|(x_1, count)| {
                    let n = (x_1 - x_0).abs();
                    let fuel_cost = n * (n + 1) / 2;
                    fuel_cost * count
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    const INPUT: &str = include_str!("../../../testdata/year_2021/day_07.txt");

    #[test_case(EXAMPLE, 37)]
    #[test_case(INPUT, 345035)]
    fn part_1(input: &str, expected: isize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 168)]
    #[test_case(INPUT, 97038163)]
    fn part_2(input: &str, expected: isize) {
        assert_eq!(super::part_2(input), expected);
    }
}
