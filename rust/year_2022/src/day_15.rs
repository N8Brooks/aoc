use std::{cell::LazyCell, ops::RangeInclusive};

use itertools::Itertools;
use num::Integer;
use regex::Regex;

struct ScannerBeaconPair {
    scanner_x: isize,
    scanner_y: isize,
    beacon_x: isize,
    beacon_y: isize,
}

impl ScannerBeaconPair {
    fn from(line: &str) -> ScannerBeaconPair {
        let re = LazyCell::new(|| {
            Regex::new(r"\bSensor at x=(?P<scanner_x>\-?\w+), y=(?P<scanner_y>\-?\w+): closest beacon is at x=(?P<beacon_x>\-?\w+), y=(?P<beacon_y>\-?\w+)\b").unwrap()
        });
        let cap = re.captures(line).unwrap();
        let scanner_x = cap["scanner_x"].parse().unwrap();
        let scanner_y = cap["scanner_y"].parse().unwrap();
        let beacon_x = cap["beacon_x"].parse().unwrap();
        let beacon_y = cap["beacon_y"].parse().unwrap();
        ScannerBeaconPair {
            scanner_x,
            scanner_y,
            beacon_x,
            beacon_y,
        }
    }

    fn dist_from(&self, (x, y): &(isize, isize)) -> usize {
        self.scanner_x.abs_diff(*x) + self.scanner_y.abs_diff(*y)
    }

    fn min_beacon_dist(&self) -> usize {
        self.dist_from(&(self.beacon_x, self.beacon_y))
    }

    fn is_outside(&self, coordinate: &(isize, isize)) -> bool {
        self.dist_from(coordinate) > self.min_beacon_dist()
    }

    fn bot_vertex_y(&self) -> isize {
        self.scanner_x - self.min_beacon_dist() as isize
    }

    fn top_vertex_y(&self) -> isize {
        self.scanner_y + self.min_beacon_dist() as isize
    }
}

pub fn part_1(input: &str, row_y: isize) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let pair = ScannerBeaconPair::from(line);
            let dist = pair.min_beacon_dist();
            let delta_y = pair.scanner_y.abs_diff(row_y);
            dist.checked_sub(delta_y).map(|overshot| {
                let overshot = overshot as isize;
                let start = pair.scanner_x - overshot;
                let end = pair.scanner_x + overshot;
                start..=end
            })
        })
        .sorted_unstable_by_key(|range| *range.start())
        .scan(isize::MIN, |cursor_0, range| {
            let cursor_1 = *range.end().max(cursor_0);
            let count = cursor_1 - range.start().max(cursor_0);
            *cursor_0 = cursor_1;
            Some(count as usize)
        })
        .sum()
}

pub fn part_2(input: &str, range: RangeInclusive<isize>) -> isize {
    let pairs = input.lines().map(ScannerBeaconPair::from).collect_vec();

    // The y-intercepts of the "back slash" and "forward slash" bounding diagonals
    let y_ints_neg_m = get_y_ints(&pairs, |pair| pair.scanner_x);
    let y_ints_pos_m = get_y_ints(&pairs, |pair| -pair.scanner_x);

    // Lines only cross if they have y-intercepts that are equal modulo 2
    let (evens_neg_m, odds_neg_m): (Vec<isize>, Vec<isize>) =
        y_ints_neg_m.partition(|y| y.is_odd());
    let (evens_pos_m, odds_pos_m): (Vec<isize>, Vec<isize>) =
        y_ints_pos_m.partition(|y| y.is_odd());

    // Find all possible crossing points between lines
    let possible_coords_a = get_possible_coords(&evens_neg_m, &evens_pos_m);
    let possible_coords_b = get_possible_coords(&odds_neg_m, &odds_pos_m);

    let (x, y) = possible_coords_a
        .chain(possible_coords_b)
        .filter(|coord| range.contains(&coord.0) && range.contains(&coord.1))
        .unique()
        .find(|coord| pairs.iter().all(|pair| pair.is_outside(coord)))
        .unwrap();

    4_000_000 * x + y
}

fn get_y_ints(
    pairs: &[ScannerBeaconPair],
    func: fn(&ScannerBeaconPair) -> isize,
) -> impl Iterator<Item = isize> + '_ {
    pairs
        .iter()
        .map(move |pair| pair.bot_vertex_y() - 1 + func(pair))
        .chain(
            pairs
                .iter()
                .map(move |pair| pair.top_vertex_y() + 1 + func(pair)),
        )
        .unique()
}

fn get_possible_coords<'a>(
    neg_m: &'a [isize],
    pos_m: &'a [isize],
) -> impl Iterator<Item = (isize, isize)> + 'a {
    neg_m
        .iter()
        .cartesian_product(pos_m)
        .map(|(y_0, y_1)| (y_0.abs_diff(*y_1) as isize / 2, (y_0 + y_1) / 2))
}

#[cfg(test)]
mod test {
    use std::ops::RangeInclusive;

    use test_case::test_case;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    #[test_case(EXAMPLE, 10 => 26)]
    #[test_case(INPUT, 2_000_000 => 4717631)]
    fn part_1(input: &str, row_y: isize) -> usize {
        super::part_1(input, row_y)
    }

    #[test_case(EXAMPLE, 0..=20 => 56000011)]
    #[test_case(INPUT, 0..=4_000_000 => 13197439355220)]
    fn part_2(input: &str, max_var: RangeInclusive<isize>) -> isize {
        super::part_2(input, max_var)
    }
}
