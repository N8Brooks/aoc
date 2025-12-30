use std::{cell::Cell, iter::repeat_with, rc::Rc};

use crate::intcode::{IntcodeExt as _, parse_program};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Unknown,
    Wall,
    Empty,
    Oxygen,
}

const START: (usize, usize) = (21, 21);

pub fn part_1(input: &str) -> usize {
    let mut map = parse_maze(input);

    let mut stack = vec![(START.0, START.1, 0)];
    while let Some((i, j, dist)) = stack.pop() {
        match map[i][j] {
            Tile::Unknown | Tile::Wall => {}
            Tile::Empty => {
                map[i][j] = Tile::Unknown;
                stack.extend([
                    (i + 1, j, dist + 1),
                    (i - 1, j, dist + 1),
                    (i, j + 1, dist + 1),
                    (i, j - 1, dist + 1),
                ]);
            }
            Tile::Oxygen => {
                return dist;
            }
        }
    }
    0
}

pub fn part_2(input: &str) -> usize {
    let mut map = parse_maze(input);
    let (oi, oj) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &tile)| (tile == Tile::Oxygen).then_some((i, j)))
        })
        .unwrap();
    let mut stack_1 = vec![(oi, oj)];
    let mut stack_2 = Vec::new();
    let mut minutes = 0;
    while !stack_1.is_empty() {
        for (i1, j2) in stack_1.drain(..) {
            for (i1, i2) in [(i1 + 1, j2), (i1 - 1, j2), (i1, j2 + 1), (i1, j2 - 1)] {
                if map[i1][i2] == Tile::Empty {
                    map[i1][i2] = Tile::Oxygen;
                    stack_2.push((i1, i2));
                }
            }
        }
        std::mem::swap(&mut stack_1, &mut stack_2);
        minutes += 1;
    }
    minutes - 1
}

fn parse_maze(input: &str) -> [[Tile; 43]; 43] {
    let dir_idx = Rc::new(Cell::new(0));
    let (mut i1, mut j1) = START;

    let program = parse_program(input);
    let mut intcode = repeat_with({
        const DIRS: [isize; 4] = [1, 4, 2, 3];
        let dir_idx = Rc::clone(&dir_idx);
        move || DIRS[dir_idx.get()]
    })
    .intcode(program);

    let right = |i: usize| (i + 1) % 4;
    let left = |i: usize| (i + 3) % 4;

    let mut map = [[Tile::Unknown; 43]; 43];

    while {
        let (i2, j2) = match dir_idx.get() {
            0 => (i1 - 1, j1),
            1 => (i1, j1 + 1),
            2 => (i1 + 1, j1),
            3 => (i1, j1 - 1),
            d => panic!("unexpected input {d}"),
        };
        match intcode.next().unwrap() {
            0 => {
                map[i2][j2] = Tile::Wall;
                dir_idx.update(left);
            }
            1 => {
                map[i2][j2] = Tile::Empty;
                (i1, j1) = (i2, j2);
                dir_idx.update(right);
            }
            2 => {
                map[i2][j2] = Tile::Oxygen;
                (i1, j1) = (i2, j2);
                dir_idx.update(right);
            }
            output => panic!("unexpected output {output}"),
        }
        (dir_idx.get(), (i1, j1)) != (0, START)
    } {}

    map
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    #[test_case(INPUT => 266)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 274)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
