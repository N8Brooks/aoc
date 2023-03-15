use std::iter::repeat;

use lazy_static::lazy_static;
use num::Complex;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct IsOpen(bool);

#[derive(Debug)]
enum PathToken {
    Left,
    Right,
    Distance(usize),
}

pub fn part_1(input: &str) -> usize {
    let (board, path_tokens) = parse_input(input);
    let n_rows = board.len() as isize;
    let n_cols = board.iter().map(|row| row.len()).max().unwrap() as isize;
    let mut direction: Complex<isize> = Complex::new(0, 1);
    let mut location = Complex::new(
        0,
        board[0]
            .iter()
            .position(|tile| *tile == Some(IsOpen(true)))
            .unwrap(),
    );
    for path_token in path_tokens {
        match path_token {
            PathToken::Left => direction *= &Complex::new(0, 1),
            PathToken::Right => direction *= &Complex::new(0, -1),
            PathToken::Distance(distance) => {
                location = repeat(direction)
                    // Introduces signed indexes to go beyond the board's bounds
                    .scan(
                        Complex::new(location.re as isize, location.im as isize),
                        |location, direction| {
                            *location += direction;
                            Some(*location)
                        },
                    )
                    // Translates the signed model back into the unsigned, wrapped board
                    .map(|Complex { re: i, im: j }| Complex {
                        re: i.rem_euclid(n_rows) as usize,
                        im: j.rem_euclid(n_cols) as usize,
                    })
                    // Translates the board into a map of just tiles
                    .filter_map(|location @ Complex { re: i, im: j }| {
                        board[i]
                            .get(j)
                            .unwrap_or(&None)
                            .map(|tile| (location, tile))
                    })
                    .take(distance)
                    .take_while(|(_, tile)| *tile == IsOpen(true))
                    .last()
                    .map(|(location, _)| location)
                    .unwrap_or(location);
            }
        }
    }
    final_password(location, direction)
}

fn parse_input(input: &str) -> (Vec<Vec<Option<IsOpen>>>, Vec<PathToken>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:L|R|\d+)").unwrap();
    }
    let (board, path) = input.split_once("\n\n").unwrap();
    let board = board
        .lines()
        .map(|line| {
            line.bytes()
                .map(|byte| match byte {
                    b' ' => None,
                    b'#' => Some(IsOpen(false)),
                    b'.' => Some(IsOpen(true)),
                    _ => panic!("unsupported byte {byte}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let path = RE
        .captures_iter(path)
        .map(|token| match &token[0] {
            "L" => PathToken::Left,
            "R" => PathToken::Right,
            distance => PathToken::Distance(distance.parse().unwrap()),
        })
        .collect();
    (board, path)
}

fn final_password(location: Complex<usize>, direction: Complex<isize>) -> usize {
    let Complex { re: i, im: j } = location;
    let one_indexed_i = i + 1;
    let one_indexed_j = j + 1;
    let facing = match direction {
        Complex { re: 0, im: 1 } => 0,
        Complex { re: 1, im: 0 } => 1,
        Complex { re: -1, im: 0 } => 2,
        Complex { re: 0, im: -1 } => 3,
        _ => panic!("unknown direction {direction}"),
    };
    1000 * one_indexed_i + 4 * one_indexed_j + facing
}

// pub fn part_2(input: &str) -> usize {
//     todo!();
// }

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_22.txt");

    #[test_case(EXAMPLE, 6032)]
    #[test_case(INPUT, 97356)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    // #[test_case(EXAMPLE, 5031)]
    // #[test_case(INPUT, 0)]
    // fn part_2(input: &str, expected: usize) {
    //     assert_eq!(super::part_2(input), expected);
    // }
}
