use hashbrown::HashSet;
use itertools::Itertools as _;
use num::complex::Complex;

pub fn part_1(input: &str) -> usize {
    guard_positions(input).len()
}

pub fn part_2(input: &str) -> usize {
    let (pos, obstacles, (m, n)) = parse_input(input);
    // Maybe remove first guard position
    guard_positions(input)
        .into_iter()
        .filter(|&obstacle| {
            let f = |(pos_1, mut dir_1): (Complex<i16>, Complex<i16>)| loop {
                let pos_2 = pos_1 + dir_1;
                if !(0..m).contains(&pos_2.re) || !(0..n).contains(&pos_2.im) {
                    return None;
                }
                if obstacle == pos_2 || obstacles[pos_2.re as usize][pos_2.im as usize] {
                    dir_1 *= Complex::new(0, -1);
                } else {
                    return Some((pos_2, dir_1));
                }
            };
            let dir = Complex::new(-1, 0);
            let mut x_1 = (pos, dir);
            let mut x_2 = if let Some(x_2) = f(x_1) {
                x_2
            } else {
                return false;
            };
            while x_1 != x_2 {
                x_2 = if let Some(x_2) = f(x_2).and_then(f) {
                    x_2
                } else {
                    return false;
                };
                x_1 = f(x_1).expect("followed x_2's path");
            }
            true
        })
        .count()
}

fn guard_positions(input: &str) -> HashSet<Complex<i16>> {
    let (pos, obstacles, (m, n)) = parse_input(input);
    let f = |(pos_1, dir_1): (Complex<i16>, Complex<i16>)| {
        let pos_2 = pos_1 + dir_1;
        if !(0..m).contains(&pos_2.re) || !(0..n).contains(&pos_2.im) {
            return None;
        }
        if obstacles[pos_2.re as usize][pos_2.im as usize] {
            let dir_2 = dir_1 * Complex::new(0, -1);
            let pos_2 = pos_1 + dir_2;
            Some((pos_2, dir_2))
        } else {
            Some((pos_2, dir_1))
        }
    };
    let mut positions = HashSet::from_iter([pos]);
    let dir = Complex::new(-1, 0);
    let mut x = (pos, dir);
    while let Some(y) = f(x) {
        x = y;
        positions.insert(x.0);
    }
    positions
}

fn parse_input(input: &str) -> (Complex<i16>, Vec<Vec<bool>>, (i16, i16)) {
    let mut guard = None;
    let map = input.lines().collect_vec();
    let (m, n) = (map.len(), map[0].len());
    let mut obstacles = vec![vec![false; n]; m];
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    guard = Some(Complex::new(i as i16, j as i16));
                }
                '#' => {
                    obstacles[i][j] = true;
                }
                '.' => {}
                _ => panic!("unexpected char: {}", c),
            }
        }
    }
    (guard.unwrap(), obstacles, (m as i16, n as i16))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

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
