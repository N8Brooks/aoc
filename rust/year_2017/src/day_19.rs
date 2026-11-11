use std::{ascii::Char, iter::successors};

pub fn part_1(input: &str) -> String {
    route(input).filter_map(Point::letter).collect()
}

pub fn part_2(input: &str) -> usize {
    route(input).count()
}

fn route(input: &str) -> impl Iterator<Item = Point> {
    let map = parse_map(input);
    let mut i = 0;
    let mut j = map[i].iter().position(|&b| b == Some(Straight)).unwrap();
    let (mut di, mut dj) = (1, 0);
    successors(map[i][j], move |&point| {
        if point == Crossing {
            (di, dj) = [(-dj, di), (dj, -di)]
                .into_iter()
                .find(|&(di, dj)| map[i.strict_add_signed(di)][j.strict_add_signed(dj)].is_some())
                .unwrap();
        }
        (i, j) = (i.strict_add_signed(di), j.strict_add_signed(dj));
        map[i][j]
    })
}

fn parse_map(input: &str) -> Vec<Vec<Option<Point>>> {
    input
        .lines()
        .map(|line| line.bytes().map(Point::parse).try_collect())
        .try_collect()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Point {
    Straight,
    Crossing,
    Letter(Char),
}

use Point::*;

impl Point {
    fn parse(b: u8) -> Result<Option<Self>, ()> {
        let point = match b {
            b' ' => return Ok(None),
            b'|' | b'-' => Straight,
            b'+' => Crossing,
            b'A'..=b'Z' => Letter(Char::from_u8(b).unwrap()),
            _ => return Err(()),
        };
        Ok(Some(point))
    }

    fn letter(self) -> Option<Char> {
        if let Letter(c) = self { Some(c) } else { None }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_19.txt");

    const EXAMPLE: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                ";

    #[test_case(EXAMPLE => "ABCDEF")]
    #[test_case(INPUT => "PBAZYFMHT")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 38)]
    #[test_case(INPUT => 16072)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
