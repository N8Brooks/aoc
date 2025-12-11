use std::iter;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let n = input.lines().count() - 1;
    let mut rows: Vec<_> = input
        .lines()
        .take(n)
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse().unwrap()))
        .collect();
    let lists = iter::from_fn(move || -> Option<Vec<usize>> {
        rows.iter_mut().map(|row| row.next()).collect()
    })
    .map(|list| list.into_iter());
    input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .zip_eq(lists)
        .map(move |(op, list)| -> usize {
            match op {
                "*" => list.product(),
                "+" => list.sum(),
                _ => panic!("unknown op {op}"),
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let n = input.lines().count() - 1;
    let mut rows: Vec<_> = input
        .lines()
        .take(n)
        .map(|line| {
            line.bytes()
                .map(|b| (b != b' ').then(|| (b - b'0') as usize))
        })
        .collect();
    let cols =
        iter::from_fn(|| -> Option<Vec<_>> { rows.iter_mut().map(|row| row.next()).collect() });
    let nums: Vec<_> = cols
        .map(|col| {
            col.into_iter()
                .flatten()
                .reduce(|num, digit| num * 10 + digit)
        })
        .collect();
    let lists = nums
        .split(|num| num.is_none())
        .map(|nums| nums.iter().map(|num| num.unwrap()));
    input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .zip_eq(lists)
        .map(move |(op, list)| -> usize {
            match op {
                "*" => list.product(),
                "+" => list.sum(),
                _ => panic!("unknown op {op}"),
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test_case(EXAMPLE => 4277556)]
    #[test_case(INPUT => 5873191732773)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 3263827)]
    #[test_case(INPUT => 11386445308378)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
