use std::str::FromStr;

use num::complex::Complex;

use crate::day_02::Dir::*;

pub fn part_1(input: &str) -> isize {
    let Complex { re: i, im: j } = parse_input(input)
        .map(|(dir, mag)| match dir {
            Forward => Complex::new(0, mag),
            Down => Complex::new(mag, 0),
            Up => Complex::new(-mag, 0),
        })
        .sum();

    i * j
}

pub fn part_2(input: &str) -> isize {
    let mut aim = 0;

    let Complex { re: i, im: j } = parse_input(input)
        .filter_map(|(dir, mag)| match dir {
            Forward => Some(Complex::new(aim * mag, mag)),
            Down => {
                aim += mag;
                None
            }
            Up => {
                aim -= mag;
                None
            }
        })
        .sum();

    i * j
}

fn parse_input(input: &str) -> impl Iterator<Item = (Dir, isize)> {
    input.lines().map(|line| {
        let (dir, mag) = line.split_once(' ').unwrap();
        (dir.parse().unwrap(), mag.parse().unwrap())
    })
}

enum Dir {
    Forward,
    Down,
    Up,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Dir::Forward,
            "down" => Dir::Down,
            "up" => Dir::Up,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    #[test_case(EXAMPLE => 150)]
    #[test_case(INPUT => 1855814)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 900)]
    #[test_case(INPUT => 1845455714)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
