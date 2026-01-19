use std::{cmp::Ordering::*, mem, num::NonZeroUsize};

use itertools::{Itertools as _, iproduct};

pub fn part_1(input: &str) -> usize {
    let points = parse_input(input);
    let (&min_i, &max_i) = points
        .iter()
        .map(|(i, _)| i)
        .minmax()
        .into_option()
        .unwrap();
    let (&min_j, &max_j) = points
        .iter()
        .map(|(_, j)| j)
        .minmax()
        .into_option()
        .unwrap();

    let closest = |i1: usize, j1: usize| {
        points
            .iter()
            .enumerate()
            .fold((None, usize::MAX), |(res, min), (idx, (i2, j2))| {
                let dist = i2.abs_diff(i1) + j2.abs_diff(j1);
                match dist.cmp(&min) {
                    Greater => (res, min),
                    Equal => (None, min),
                    Less => (Some(idx), dist),
                }
            })
            .0
    };

    let mut counts = vec![0usize; points.len()];
    iproduct!(min_i + 1..max_i, min_j + 1..max_j)
        .filter_map(|(i, j)| closest(i, j))
        .for_each(|idx| counts[idx] += 1);

    let mut counts: Vec<Option<NonZeroUsize>> = unsafe { mem::transmute(counts) };
    iproduct!(min_i + 1..max_i, [min_j, max_j])
        .chain(iproduct!([min_i, max_i], min_j + 1..max_j))
        .filter_map(|(i, j)| closest(i, j))
        .for_each(|idx| counts[idx] = None);

    counts
        .into_iter()
        .flatten()
        .map(NonZeroUsize::get)
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    count_close(input, 10_000)
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (j, i) = line.split_once(", ").unwrap();
            (i.parse().unwrap(), j.parse().unwrap())
        })
        .collect()
}

fn count_close(input: &str, k: usize) -> usize {
    let points = parse_input(input);
    let (&min_i, &max_i) = points
        .iter()
        .map(|(i, _)| i)
        .minmax()
        .into_option()
        .unwrap();
    let (&min_j, &max_j) = points
        .iter()
        .map(|(_, j)| j)
        .minmax()
        .into_option()
        .unwrap();
    iproduct!(min_i..=max_i, min_j..=max_j)
        .filter(|&(i1, j1)| {
            let dist: usize = points
                .iter()
                .map(|(i2, j2)| i2.abs_diff(i1) + j2.abs_diff(j1))
                .sum();
            dist < k
        })
        .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    #[test_case(EXAMPLE => 17)]
    #[test_case(INPUT => 3293)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 45176)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }

    #[test_case(EXAMPLE => 16)]
    fn count_close(input: &str) -> usize {
        super::count_close(input, 32)
    }
}
