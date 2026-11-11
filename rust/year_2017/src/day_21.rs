use std::{array, iter::successors};

use hashbrown::HashMap;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    count_on_pixels(input, 5)
}

pub fn part_2(input: &str) -> usize {
    count_on_pixels(input, 18)
}

fn count_on_pixels(input: &str, iterations: usize) -> usize {
    let rules2 = parse_rules::<2, 3>(input);
    let rules3 = parse_rules::<3, 4>(input);
    let mut cur = Vec::from(parse::<3>(".#./..#/###").map(Vec::from));

    for _ in 0..iterations {
        cur = if cur.len().is_multiple_of(2) {
            enhance(&cur, &rules2)
        } else {
            enhance(&cur, &rules3)
        };
    }

    cur.into_iter().flatten().filter(|&b| b).count()
}

fn parse_rules<const N: usize, const M: usize>(
    input: &str,
) -> HashMap<[[bool; N]; N], [[bool; M]; M]> {
    input
        .lines()
        .filter(|line| line.find('/') == Some(N))
        .flat_map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            let to = parse::<M>(to);
            parse::<N>(from).variants().map(move |from| (from, to))
        })
        .collect()
}

fn parse<const N: usize>(s: &str) -> [[bool; N]; N] {
    s.split('/')
        .map(|row| row.bytes().map(|b| b == b'#').collect_array().unwrap())
        .collect_array()
        .unwrap()
}

fn enhance<const N: usize, const M: usize>(
    grid: &[Vec<bool>],
    rules: &HashMap<[[bool; N]; N], [[bool; M]; M]>,
) -> Vec<Vec<bool>> {
    grid.as_chunks::<N>()
        .0
        .iter()
        .flat_map(|rows| {
            let chunks = rows.each_ref().map(|row| row.as_chunks::<N>().0);
            let squares = (0..chunks[0].len())
                .map(|j| &rules[&chunks.map(|row| row[j])])
                .collect_vec();
            (0..M).map(move |row| squares.iter().flat_map(|square| square[row]).collect())
        })
        .collect()
}

trait Square<const N: usize> {
    fn variants(self) -> impl Iterator<Item = Self>;
    fn rotate(self) -> Self;
}

impl<const N: usize, T: Copy> Square<N> for [[T; N]; N] {
    fn variants(self) -> impl Iterator<Item = Self> {
        successors(Some(self), |&m| Some(m.rotate()))
            .take(4)
            .flat_map(|m| {
                let mut flipped = m;
                flipped.reverse();
                [m, flipped]
            })
    }

    fn rotate(self) -> Self {
        array::from_fn(|row| array::from_fn(|col| self[N - col - 1][row]))
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_21.txt");

    const EXAMPLE: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test_case(EXAMPLE, 2 => 12)]
    fn example(input: &str, iterations: usize) -> usize {
        super::count_on_pixels(input, iterations)
    }

    #[test_case(INPUT => 194)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 2536879)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
