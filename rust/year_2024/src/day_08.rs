use std::iter::once;

use hashbrown::HashSet;

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let n = input.lines().next().unwrap().len(); // assume square matrix
    let iter_axis = |a: usize, b: usize| -> Box<dyn Iterator<Item = usize>> {
        if a > b {
            Box::new(once(a + a - b).filter(move |&c| c < n))
        } else {
            Box::new(a.checked_sub(b - a).into_iter())
        }
    };
    count_antinodes(input, iter_axis)
}

pub fn part_2(input: &str) -> usize {
    let n = input.lines().next().unwrap().len(); // assume square matrix
    let iter_axis = |a: usize, b: usize| -> Box<dyn Iterator<Item = usize>> {
        if a > b {
            Box::new((a..n).step_by(a - b))
        } else {
            Box::new((0..=a).rev().step_by(b - a))
        }
    };
    count_antinodes(input, iter_axis)
}

fn count_antinodes(
    input: &str,
    iter_axis: impl Fn(usize, usize) -> Box<dyn Iterator<Item = usize>>,
) -> usize {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.bytes().enumerate().map(move |(j, c)| (c, (i, j))))
        .filter(|(c, _)| *c != b'.')
        .into_group_map()
        .into_values()
        .flat_map(|antennas| {
            antennas
                .into_iter()
                .tuple_combinations()
                .flat_map(|((i1, j1), (i2, j2))| {
                    let rows_1 = iter_axis(i1, i2);
                    let cols_1 = iter_axis(j1, j2);
                    let rows_2 = iter_axis(i2, i1);
                    let cols_2 = iter_axis(j2, j1);
                    rows_1.zip(cols_1).chain(rows_2.zip(cols_2))
                })
        })
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test_case(EXAMPLE, 14)]
    #[test_case(INPUT, 291)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 34)]
    #[test_case(INPUT, 1015)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
