use std::{array, ascii::Char, iter::successors, str::FromStr};

use DanceMove::*;

pub fn part_1(input: &str) -> String {
    dance_once::<16>(input)
}

fn dance_once<const N: usize>(input: &str) -> String {
    let mut programs: [_; N] = array::from_fn(|i| Char::from_u8(b'a' + i as u8).unwrap());
    programs.dance_all(input.split(',').map(|line| line.parse().unwrap()));
    programs.as_str().into()
}

pub fn part_2(input: &str) -> String {
    let programs_0: [_; 16] = array::from_fn(|i| Char::from_u8(b'a' + i as u8).unwrap());
    let moves: Vec<DanceMove> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut dances = successors(Some(programs_0), |programs| {
        let mut next = *programs;
        next.dance_all(moves.iter().copied());
        Some(next)
    });
    let period = dances
        .clone()
        .skip(1)
        .position(|state| state == programs_0)
        .unwrap()
        + 1;
    dances.nth(1_000_000_000 % period).unwrap().as_str().into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(Char, Char),
}

impl FromStr for DanceMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, args) = s.split_at_checked(1).ok_or(())?;

        match kind {
            "s" => args.parse().map(Spin).map_err(|_| ()),
            "x" => {
                let (i, j) = args.split_once('/').ok_or(())?;
                Ok(Exchange(
                    i.parse().map_err(|_| ())?,
                    j.parse().map_err(|_| ())?,
                ))
            }
            "p" => {
                let (a, b) = args.split_once('/').ok_or(())?;
                let ([a], [b]) = (a.as_bytes(), b.as_bytes()) else {
                    return Err(());
                };
                let a = Char::from_u8(*a).ok_or(())?;
                let b = Char::from_u8(*b).ok_or(())?;
                Ok(Partner(a, b))
            }
            _ => Err(()),
        }
    }
}

trait Dance {
    fn dance(&mut self, dance_move: DanceMove);

    fn dance_all(&mut self, moves: impl IntoIterator<Item = DanceMove>) {
        for dance_move in moves {
            self.dance(dance_move);
        }
    }
}

impl Dance for [Char] {
    fn dance(&mut self, dance_move: DanceMove) {
        match dance_move {
            Spin(n) => self.rotate_right(n % self.len()),
            Exchange(i, j) => self.swap(i, j),
            Partner(a, b) => {
                let i = self.iter().position(|&p| p == a).unwrap();
                let j = self.iter().position(|&p| p == b).unwrap();
                self.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "s1,x3/4,pe/b";

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    #[test_case(INPUT => "fnloekigdmpajchb")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => "baedc")]
    fn part_1_example(input: &str) -> String {
        super::dance_once::<5>(input)
    }

    #[test_case(INPUT => "amkjepdhifolgncb")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
