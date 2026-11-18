use std::array;

use hashbrown::{DefaultHashBuilder, HashSet};
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    simulate::<3>(input)
}

pub fn part_2(input: &str) -> usize {
    simulate::<4>(input)
}

fn simulate<const M: usize>(input: &str) -> usize
where
    [(); 3usize.pow(M as u32) - 1]:,
{
    let directions = const { neighbors::<M>() };

    let mut positions: HashSet<[i8; M]> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, b)| ([x as i8, y as i8], b))
        })
        .filter(|&(_, b)| b == b'#')
        .map(|(coords, _)| array::from_fn(|i| coords.get(i).copied().unwrap_or(0)))
        .collect();

    for _ in 0..6 {
        let counts = positions
            .iter()
            .flat_map(|pos| {
                directions
                    .iter()
                    .map(move |dir| array::from_fn(|i| pos[i] + dir[i]))
            })
            .counts_with_hasher(DefaultHashBuilder::default());

        positions = counts
            .into_iter()
            .filter(|&(cube, count)| count == 3 || (count == 2 && positions.contains(&cube)))
            .map(|(cube, _)| cube)
            .collect();
    }

    positions.len()
}

const fn neighbors<const M: usize>() -> [[i8; M]; 3usize.pow(M as u32) - 1]
where
    [(); 3usize.pow(M as u32) - 1]:,
{
    let n = 3usize.pow(M as u32) - 1;

    array::from_fn(const |i| {
        let value = i + (i >= n / 2) as usize;

        array::from_fn(const |j| (value / 3usize.pow(j as u32) % 3) as i8 - 1)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_17.txt");

    const EXAMPLE: &str = ".#.
..#
###";

    #[test_case(EXAMPLE => 112)]
    #[test_case(INPUT => 368)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 848)]
    #[test_case(INPUT => 2696)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
