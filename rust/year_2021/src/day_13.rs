use std::str::FromStr;

use hashbrown::DefaultHashBuilder;
use itertools::Itertools as _;
use util::str;

type Point = [usize; 2];

pub fn part_1(input: &str) -> usize {
    let (points, folds) = parse_input(input);
    let (axis, value) = folds.into_iter().next().unwrap();
    points
        .map(|point| axis.fold(point, value))
        .unique_with_hasher(DefaultHashBuilder::default())
        .count()
}

pub fn part_2(input: &str) -> String {
    let (points, folds) = parse_input(input);

    let mut points: Vec<_> = points.collect();

    let mut paper = [[b'.'; 40]; 6];

    for (axis, value) in folds {
        points.retain_unstable(|point| {
            *point = axis.fold(*point, value);

            let [x, y] = *point;
            if let Some(cell) = paper.get_mut(y).and_then(|row| row.get_mut(x)) {
                *cell = b'#';
                false
            } else {
                true
            }
        });
    }

    str::from_image(&paper)
}

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = [usize; 2]>,
    impl Iterator<Item = (Axis, usize)>,
) {
    let (points, folds) = input.rsplit_once("\n\n").unwrap();

    let points = points.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        [x.parse().unwrap(), y.parse().unwrap()]
    });

    let folds = folds.lines().map(|fold| {
        let (axis, value) = fold
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();
        let value: usize = value.parse().unwrap();
        (axis.parse().unwrap(), value)
    });

    (points, folds)
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X = 0,
    Y = 1,
}

impl FromStr for Axis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => return Err(()),
        })
    }
}

impl Axis {
    fn fold(self, mut point: Point, value: usize) -> Point {
        let coord = &mut point[self as usize];
        *coord = value - coord.abs_diff(value);
        point
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    const EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test_case(EXAMPLE => 17)]
    #[test_case(INPUT => 759)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => "HECRZKPR")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
