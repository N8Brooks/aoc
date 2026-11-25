use std::{iter::successors, str::FromStr};

use hashbrown::DefaultHashBuilder;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    parse_lines(input)
        .filter(Line::is_axis_aligned)
        .flatten()
        .duplicates_with_hasher(DefaultHashBuilder::default())
        .count()
}

pub fn part_2(input: &str) -> usize {
    parse_lines(input)
        .flatten()
        .duplicates_with_hasher(DefaultHashBuilder::default())
        .count()
}

fn parse_lines(input: &str) -> impl Iterator<Item = Line> {
    input.lines().map(|line| line.parse().unwrap())
}

#[derive(Debug, Clone, Copy)]
struct Line(Point, Point);

impl Line {
    fn is_axis_aligned(&self) -> bool {
        let Self(Point(x1, y1), Point(x2, y2)) = self;
        x1 == x2 || y1 == y2
    }
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or("missing arrow")?;
        Ok(Self(start.parse()?, end.parse()?))
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = impl Iterator<Item = Point>;

    fn into_iter(self) -> Self::IntoIter {
        let Self(start @ Point(x1, y1), end @ Point(x2, y2)) = self;
        let delta = (x2.cmp(&x1) as isize, y2.cmp(&y1) as isize);
        successors(Some(start), move |&p| {
            (p != end).then(|| p.strict_add_signed(delta))
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("missing comma")?;

        Ok(Self(
            x.parse().map_err(|_| "invalid x coordinate")?,
            y.parse().map_err(|_| "invalid y coordinate")?,
        ))
    }
}

impl Point {
    #[inline]
    fn strict_add_signed(self, (dx, dy): (isize, isize)) -> Self {
        Self(self.0.strict_add_signed(dx), self.1.strict_add_signed(dy))
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test_case(EXAMPLE => 5)]
    #[test_case(INPUT => 5167)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 12)]
    #[test_case(INPUT => 17604)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
