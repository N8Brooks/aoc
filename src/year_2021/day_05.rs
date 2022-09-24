use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn from_line(line: &str) -> (Coord, Coord) {
        let (start, stop) = line.split_once(" -> ").unwrap();
        (Coord::from_pair(start), Coord::from_pair(stop))
    }

    fn from_pair(pair: &str) -> Coord {
        let (x, y) = pair.split_once(',').unwrap();
        Coord {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn from_delta(start: &Coord, stop: &Coord) -> Coord {
        Coord {
            x: sign(stop.x, start.x),
            y: sign(stop.y, start.y),
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        *self = Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn sign(stop: i32, start: i32) -> i32 {
    match stop.cmp(&start) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub fn part_1(input: &str) -> usize {
    let mut counts: HashMap<Coord, usize> = HashMap::new();
    for line in input.lines() {
        let (mut coord_1, coord_2) = Coord::from_line(line);
        let delta = Coord::from_delta(&coord_1, &coord_2);
        if delta.x != 0 && delta.y != 0 {
            continue;
        }
        while coord_1 != coord_2 {
            *counts.entry(coord_1).or_default() += 1;
            coord_1 += delta;
        }
        *counts.entry(coord_2).or_default() += 1;
    }
    counts.values().filter(|&val| val > &1).count()
}

pub fn part_2(input: &str) -> usize {
    let mut counts: HashMap<Coord, usize> = HashMap::new();
    for line in input.lines() {
        let (mut coord_1, coord_2) = Coord::from_line(line);
        let delta = Coord::from_delta(&coord_1, &coord_2);
        while coord_1 != coord_2 {
            *counts.entry(coord_1).or_default() += 1;
            coord_1 += delta;
        }
        *counts.entry(coord_2).or_default() += 1;
    }
    counts.values().filter(|&val| val > &1).count()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    static EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    static INPUT: &str = include_str!("testdata/day_05.txt");

    #[test_case(EXAMPLE, 5)]
    #[test_case(INPUT, 5167)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 12)]
    #[test_case(INPUT, 17604)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
