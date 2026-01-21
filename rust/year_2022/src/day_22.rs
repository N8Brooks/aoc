use std::{cell::LazyCell, iter::successors};

use regex::Regex;

#[derive(Debug)]
enum Path {
    Left,
    Right,
    Distance(usize),
}

use Path::*;

pub fn part_1(input: &str) -> usize {
    let (board, path) = parse_input(input);
    let n = board.len() as isize;
    let m = board.iter().map(|row| row.len()).max().unwrap() as isize;
    let (mut di, mut dj) = (0, 1);
    let (mut i, mut j) = board
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &tile)| tile == Some(true))
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    for p in path {
        match p {
            Left => (di, dj) = (-dj, di),
            Right => (di, dj) = (dj, -di),
            Distance(distance) => {
                (i, j) = successors(Some((i, j)), |&(i, j)| {
                    let i = (i as isize + di).rem_euclid(n) as usize;
                    let j = (j as isize + dj).rem_euclid(m) as usize;
                    Some((i, j))
                })
                .filter_map(|(i, j)| Some(((i, j), board[i].get(j).flatten_ref()?)))
                .take(distance + 1)
                .take_while(|&(_, &open)| open)
                .last()
                .unwrap()
                .0;
            }
        }
    }
    final_password((i, j), (di, dj))
}

fn parse_input(input: &str) -> (Vec<Vec<Option<bool>>>, Vec<Path>) {
    let re = LazyCell::new(|| Regex::new(r"(?:L|R|\d+)").unwrap());
    let (board, path) = input.split_once("\n\n").unwrap();
    let board = board
        .lines()
        .map(|line| {
            line.bytes()
                .map(|byte| match byte {
                    b' ' => None,
                    b'#' => Some(false),
                    b'.' => Some(true),
                    _ => panic!("unsupported byte {byte}"),
                })
                .collect()
        })
        .collect();
    let path = re
        .captures_iter(path)
        .map(|token| match &token[0] {
            "L" => Left,
            "R" => Right,
            distance => Distance(distance.parse().unwrap()),
        })
        .collect();
    (board, path)
}

fn final_password((i, j): (usize, usize), dir: (isize, isize)) -> usize {
    let (ip1, jp1) = (i + 1, j + 1);
    let facing = match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        (i, j) => panic!("unknown direction ({i}, {j})"),
    };
    1000 * ip1 + 4 * jp1 + facing
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

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    #[test_case(EXAMPLE => 6032)]
    #[test_case(INPUT => 97356)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    // #[test_case(EXAMPLE => 5031)]
    // #[test_case(INPUT => 0)]
    // fn part_2(input: &str, expected: usize) {
    //     assert_eq!(super::part_2(input), expected);
    // }
}
