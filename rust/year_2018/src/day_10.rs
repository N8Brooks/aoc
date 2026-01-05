use std::{
    cmp::Ordering,
    error::Error,
    fmt::{self, Display},
    iter::repeat_with,
    ops::Index,
};

use hashbrown::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn part_1(input: &str) -> String {
    iter_messages_sky_height_asc(input)
        .find_map(|message| message)
        .unwrap_or_else(|| unreachable!())
}

pub fn part_2(input: &str) -> usize {
    iter_messages_sky_height_asc(input)
        .take_while(|message| message.is_none())
        .count()
}

fn iter_messages_sky_height_asc(input: &str) -> impl Iterator<Item = Option<String>> {
    let mut points = input.lines().map(Point::from).collect_vec();
    let sky_0 = Sky::new(&points);
    repeat_with(move || {
        points.iter_mut().for_each(|point| point.update());
        Sky::new(&points)
    })
    .scan(sky_0, |a, b| match a.height.cmp(&b.height) {
        Ordering::Less => {
            let message = a.translate().expect("translatable message");
            Some(Some(message))
        }
        Ordering::Equal => panic!("expected different height"),
        Ordering::Greater => {
            *a = b;
            Some(None)
        }
    })
}

struct Point {
    position: [isize; 2],
    velocity: [isize; 2],
}

impl From<&str> for Point {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^position=< ?(\-?\d+),  ?(\-?\d+)> velocity=< ?(\-?\d+),  ?(\-?\d+)>$"
            )
            .unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let (position_x, position_y, velocity_x, velocity_y) = caps
            .iter()
            .skip(1)
            .map(|cap| cap.unwrap().as_str().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Point {
            position: [position_x, position_y],
            velocity: [velocity_x, velocity_y],
        }
    }
}

impl Point {
    fn update(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
    }
}

#[derive(Default)]
struct Sky {
    positions: HashSet<[isize; 2]>,
    height: usize,
    width: usize,
    min_x: isize,
    min_y: isize,
}

impl Sky {
    fn new(points: &[Point]) -> Sky {
        let positions = HashSet::from_iter(points.iter().map(|point| point.position));
        let (min_x, width) = Sky::get_axis_bounds(&positions, 0);
        let (min_y, height) = Sky::get_axis_bounds(&positions, 1);
        Sky {
            positions,
            width,
            height,
            min_x,
            min_y,
        }
    }

    fn get_axis_bounds(positions: &HashSet<[isize; 2]>, axis: usize) -> (isize, usize) {
        let (min_x, max_x) = positions
            .iter()
            .map(|position| position[axis])
            .minmax()
            .into_option()
            .unwrap();
        let width = usize::try_from(max_x + 1 - min_x).unwrap();
        (min_x, width)
    }

    fn get_delimiter(&self) -> String {
        (0..2).map(|_| ".".repeat(self.height) + "\n").collect()
    }

    fn translate(&self) -> Result<String, NoTranslation> {
        transpose(&self.to_string())
            .split(&self.get_delimiter())
            .map(transpose)
            .map(|letter| translate(&letter))
            .collect::<Result<_, _>>()
    }
}

fn transpose(input: &str) -> String {
    let input = input.lines().collect_vec();
    (0..input[0].len())
        .map(|i| {
            input
                .iter()
                .map(|line| line.index(i..=i))
                .collect::<String>()
        })
        .join("\n")
}

impl Display for Sky {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut plot = vec![".".repeat(self.width); self.height];
        for [x, y] in self.positions.iter() {
            let x = usize::try_from(x - self.min_x).unwrap();
            let y = usize::try_from(y - self.min_y).unwrap();
            plot[y].replace_range(x..=x, "#");
        }
        write!(f, "{}", plot.iter().join("\n"))
    }
}

#[derive(Debug)]
struct NoTranslation;

impl Display for NoTranslation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no translation")
    }
}

impl Error for NoTranslation {}

fn translate(letter: &str) -> Result<char, NoTranslation> {
    match letter {
        "\
.####.
#....#
#.....
#.....
#.....
#.....
#.....
#.....
#....#
.####." => Ok('C'),
        "\
######
#.....
#.....
#.....
#####.
#.....
#.....
#.....
#.....
######" => Ok('E'),
        "\
#...#
#...#
#...#
#####
#...#
#...#
#...#
#...#" => Ok('H'),
        "\
###
.#.
.#.
.#.
.#.
.#.
.#.
###" => Ok('I'),
        "\
...###
....#.
....#.
....#.
....#.
....#.
....#.
#...#.
#...#.
.###.." => Ok('J'),
        "\
#....#
#...#.
#..#..
#.#...
##....
##....
#.#...
#..#..
#...#.
#....#" => Ok('K'),
        "\
#####.
#....#
#....#
#....#
#####.
#..#..
#...#.
#...#.
#....#
#....#" => Ok('R'),
        _ => Err(NoTranslation),
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    #[test_case(EXAMPLE => "HI")]
    #[test_case(INPUT => "ERKECKJJ")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 3)]
    #[test_case(INPUT => 10645)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
