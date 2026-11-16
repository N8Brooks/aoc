use std::str::FromStr;

use num::{Complex, pow::Pow as _};

use crate::day_12::Action::*;

pub fn part_1(input: &str) -> usize {
    let (mut pos, mut dir) = (Complex::new(0, 0), Complex::new(0, 1));
    for action in parse_actions(input) {
        match action {
            Direction(d) => pos += d,
            Rotate(r) => dir *= r,
            Forward(f) => pos += dir * f,
        }
    }
    pos.re.unsigned_abs() + pos.im.unsigned_abs()
}

pub fn part_2(input: &str) -> usize {
    let (mut pos, mut waypoint) = (Complex::new(0, 0), Complex::new(-1, 10));
    for action in parse_actions(input) {
        match action {
            Direction(d) => waypoint += d,
            Rotate(r) => waypoint *= r,
            Forward(f) => pos += waypoint * f,
        }
    }
    pos.re.unsigned_abs() + pos.im.unsigned_abs()
}

fn parse_actions(input: &str) -> impl Iterator<Item = Action> {
    input.lines().map(|line| line.parse().unwrap())
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Direction(Complex<isize>),
    Rotate(Complex<isize>),
    Forward(isize),
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, magnitude) = s.split_at(1);
        let magnitude = magnitude.parse().map_err(|_| "failed to parse magnitude")?;
        Ok(match action {
            "N" => Direction(Complex::new(-magnitude, 0)),
            "S" => Direction(Complex::new(magnitude, 0)),
            "E" => Direction(Complex::new(0, magnitude)),
            "W" => Direction(Complex::new(0, -magnitude)),
            "L" => Rotate(Complex::new(0, 1).pow(magnitude / 90)),
            "R" => Rotate(Complex::new(0, -1).pow(magnitude / 90)),
            "F" => Forward(magnitude),
            _ => return Err("unknown action"),
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    const EXAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test_case(EXAMPLE => 25)]
    #[test_case(INPUT => 2847)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 286)]
    #[test_case(INPUT => 29839)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
