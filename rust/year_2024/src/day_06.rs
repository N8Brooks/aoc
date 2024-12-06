use std::collections::HashSet;

use itertools::Itertools;
use num::complex::Complex;

pub fn part_1(input: &str) -> usize {
    let (mut guard, obstacles, (m, n)) = parse_input(input);
    let mut direction = Complex::new(-1, 0);
    let mut positions = HashSet::new();
    loop {
        positions.insert(guard);
        guard += direction;
        if !(0..m).contains(&guard.re) || !(0..n).contains(&guard.im) {
            break;
        }
        if obstacles.contains(&guard) {
            guard -= direction;
            direction *= Complex::new(0, -1);
        } else {
            positions.insert(guard);
        }
    }
    positions.len()
}

pub fn part_2(input: &str) -> usize {
    let (guard, mut obstacles, (m, n)) = parse_input(input);
    (0..m)
        .cartesian_product(0..n)
        .filter(|&(i, j)| {
            let mut guard = guard;
            let mut direction = Complex::new(-1, 0);
            let mut positions = HashSet::new();
            if !obstacles.insert(Complex::new(i, j)) {
                return false;
            }
            loop {
                if !positions.insert((guard, direction)) {
                    obstacles.remove(&Complex::new(i, j));
                    return true;
                }
                guard += direction;
                if !(0..m).contains(&guard.re) || !(0..n).contains(&guard.im) {
                    obstacles.remove(&Complex::new(i, j));
                    return false;
                }
                if obstacles.contains(&guard) {
                    guard -= direction;
                    direction *= Complex::new(0, -1);
                }
            }
        })
        .count()
}

fn parse_input(input: &str) -> (Complex<isize>, HashSet<Complex<isize>>, (isize, isize)) {
    let mut obstacles = HashSet::new();
    let mut guard = None;
    let map = input.lines().collect_vec();
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    guard = Some(Complex::new(i as isize, j as isize));
                }
                '#' => {
                    obstacles.insert(Complex::new(i as isize, j as isize));
                }
                '.' => {}
                _ => panic!("unexpected char: {}", c),
            }
        }
    }
    (
        guard.unwrap(),
        obstacles,
        (map.len() as isize, map[0].len() as isize),
    )
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../testdata/year_2024/day_06.txt");

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test_case(EXAMPLE, 41)]
    #[test_case(INPUT, 5534)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 6)]
    #[test_case(INPUT, 2262)] // SLOW
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
