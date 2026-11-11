use std::iter::repeat_with;

use hashbrown::HashMap;
use num::Complex;

pub fn part_1(input: &str) -> usize {
    solve(input, 10_000, -Complex::ONE)
}

pub fn part_2(input: &str) -> usize {
    solve(input, 10_000_000, -Complex::I)
}

fn solve(input: &str, bursts: usize, cycle: Complex<i16>) -> usize {
    const CLEAN: Complex<i16> = Complex::I;
    const INFECTED: Complex<i16> = Complex::new(0, -1);

    let n = input.lines().count() as i16;
    let mut grid: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .map(move |(j, _)| (Complex::new(i as i16, j as i16), INFECTED))
        })
        .collect();
    let mut pos = Complex::new(n / 2, n / 2);
    let mut dir = Complex::new(-1, 0);
    repeat_with(|| {
        let state = grid.entry(pos).or_insert(CLEAN);
        dir *= *state;
        *state *= cycle;
        pos += dir;
        *state
    })
    .take(bursts)
    .filter(|&state| state == INFECTED)
    .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    const EXAMPLE: &str = "..#
#..
...";

    #[test_case(EXAMPLE => 5587)]
    #[test_case(INPUT => 5538)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 2511944)]
    #[test_case(INPUT => 2511090)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
