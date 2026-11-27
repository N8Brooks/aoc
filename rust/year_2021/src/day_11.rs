use std::iter::{self, repeat_with};

use arrayvec::ArrayVec;
use itertools::{Itertools as _, iproduct};

type Grid = [[u8; 10]; 10];

pub fn part_1(input: &str) -> usize {
    flash_counts(input).take(100).sum()
}

pub fn part_2(input: &str) -> usize {
    flash_counts(input).position(|count| count == 100).unwrap() + 1
}

fn flash_counts(input: &str) -> impl Iterator<Item = usize> {
    let mut grid = parse_grid(input).unwrap();
    let mut flashed = ArrayVec::<_, 100>::new();

    repeat_with(move || {
        let mut increment = |(i, j): (usize, usize), initial: bool| {
            let v = &mut grid[i][j];
            *v += u8::from(initial || *v != 0);
            let flashed = *v == 10;
            if flashed {
                *v = 0;
            }
            flashed
        };

        iproduct!(0..10, 0..10)
            .filter(|&pos| increment(pos, true))
            .collect_into(&mut flashed);

        iter::from_fn(|| {
            let (i, j) = flashed.pop()?;

            let is = i.saturating_sub(1)..=(i + 1).min(9);
            let js = j.saturating_sub(1)..=(j + 1).min(9);

            iproduct!(is, js)
                .filter(|&pos| increment(pos, false))
                .collect_into(&mut flashed);

            Some(())
        })
        .count()
    })
}

fn parse_grid(input: &str) -> Option<Grid> {
    input
        .lines()
        .collect_array()?
        .try_map(|line| line.bytes().map(|b| b - b'0').collect_array())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test_case(EXAMPLE => 1656)]
    #[test_case(INPUT => 1757)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 195)]
    #[test_case(INPUT => 422)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
