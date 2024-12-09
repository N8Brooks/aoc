use hashbrown::HashSet;

use itertools::Itertools as _;

type IterAxis = [Box<dyn Iterator<Item = usize>>; 2];

pub fn part_1(input: &str) -> usize {
    let (antennas, n) = parse_input(input);
    let iter_axis = |x1: usize, x2: usize| -> IterAxis {
        if x2 > x1 {
            let step = x2 - x1;
            [
                Box::new((x1.checked_sub(step)).into_iter()),
                Box::new((x2 + step < n).then(|| x2 + step).into_iter()),
            ]
        } else {
            let step = x1 - x2;
            [
                Box::new((x1 + step < n).then(|| x1 + step).into_iter()),
                Box::new(x2.checked_sub(step).into_iter()),
            ]
        }
    };
    count_antinodes(antennas, iter_axis)
}

pub fn part_2(input: &str) -> usize {
    let (antennas, n) = parse_input(input);
    let iter_axis = |x1: usize, x2: usize| -> IterAxis {
        if x2 > x1 {
            let step = x2 - x1;
            [
                Box::new((0..=x1).rev().step_by(step)),
                Box::new((x2..n).step_by(step)),
            ]
        } else {
            let step = x1 - x2;
            [
                Box::new((x1..n).step_by(step)),
                Box::new((0..=x2).rev().step_by(step)),
            ]
        }
    };
    count_antinodes(antennas, iter_axis)
}

fn parse_input(input: &str) -> (impl Iterator<Item = Vec<(usize, usize)>>, usize) {
    let n = input.lines().next().unwrap().len(); // Assume square matrix
    let group_map = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.bytes().enumerate().map(move |(j, c)| (c, (i, j))))
        .filter(|(c, _)| *c != b'.')
        .into_group_map()
        .into_values();
    (group_map, n)
}

fn count_antinodes(
    antennas: impl Iterator<Item = Vec<(usize, usize)>>,
    iter_axis: impl Fn(usize, usize) -> IterAxis,
) -> usize {
    antennas
        .flat_map(|v| {
            v.into_iter()
                .tuple_combinations()
                .flat_map(|((i1, j1), (i2, j2))| {
                    let [is1, is2] = iter_axis(i1, i2);
                    let [js1, js2] = iter_axis(j1, j2);
                    is1.zip(js1).chain(is2.zip(js2))
                })
        })
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    const EXAMPLE: &str = "\
............
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
