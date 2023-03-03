use std::ops::RangeInclusive;

use itertools::Itertools;
use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\bSensor at x=(?P<beacon_x>\-?\w+), y=(?P<beacon_y>\-?\w+): closest beacon is at x=(?P<scanner_x>\-?\w+), y=(?P<scanner_y>\-?\w+)\b").unwrap();
}

struct ScannerBeaconPair {
    beacon_x: isize,
    beacon_y: isize,
    scanner_x: isize,
    scanner_y: isize,
}

impl ScannerBeaconPair {
    fn from(line: &str) -> ScannerBeaconPair {
        let cap = RE.captures(line).unwrap();
        let beacon_x = cap["beacon_x"].parse().unwrap();
        let beacon_y = cap["beacon_y"].parse().unwrap();
        let scanner_x = cap["scanner_x"].parse().unwrap();
        let scanner_y = cap["scanner_y"].parse().unwrap();
        ScannerBeaconPair {
            beacon_x,
            beacon_y,
            scanner_x,
            scanner_y,
        }
    }

    fn dist(&self) -> usize {
        let diff_x = self.scanner_x.abs_diff(self.beacon_x);
        let diff_y = self.scanner_y.abs_diff(self.beacon_y);
        diff_x + diff_y
    }
}

pub fn part_1(input: &str, row_y: isize) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let pair = ScannerBeaconPair::from(line);
            let dist = pair.dist();
            let delta_y = pair.beacon_y.abs_diff(row_y);
            dist.checked_sub(delta_y).map(|overshot| {
                let overshot = overshot as isize;
                let start = pair.beacon_x - overshot;
                let end = pair.beacon_x + overshot;
                (start, end)
            })
        })
        .sorted_unstable()
        .scan(isize::MIN, |end_0, (start_1, end_1)| {
            let end_1 = end_1.max(*end_0);
            let count = end_1 - start_1.max(*end_0);
            *end_0 = end_1;
            Some(count as usize)
        })
        .sum()
}

pub fn part_2(input: &str, range: RangeInclusive<isize>) -> isize {
    let pairs = input
        .lines()
        .map(|line| ScannerBeaconPair::from(line))
        .collect_vec();

    let (y_int_neg_m, y_int_pos_m): (Vec<isize>, Vec<isize>) = pairs
        .iter()
        .flat_map(|pair| {
            let dist = pair.dist() as isize;
            let bot_vertex_y = pair.beacon_x - dist - 1;
            let top_vertex_y = pair.beacon_y + dist + 1;
            [
                (bot_vertex_y + pair.beacon_x, bot_vertex_y - pair.beacon_x),
                (top_vertex_y + pair.beacon_x, top_vertex_y - pair.beacon_x),
            ]
        })
        .unzip();

    let (even_y_int_pos_m, odd_y_int_pos_m): (Vec<isize>, Vec<isize>) =
        y_int_pos_m.iter().partition(|y| y.is_odd());
    let (even_y_int_neg_m, odd_y_int_neg_m): (Vec<isize>, Vec<isize>) =
        y_int_neg_m.iter().partition(|y| y.is_odd());

    let possible_coordinates_a = odd_y_int_pos_m
        .iter()
        .cartesian_product(odd_y_int_neg_m)
        .map(|(y_0, y_1)| (y_0.abs_diff(y_1) as isize / 2, (y_0 + y_1) / 2));
    let possible_coordinates_b = even_y_int_pos_m
        .iter()
        .cartesian_product(even_y_int_neg_m)
        .map(|(y_0, y_1)| (y_0.abs_diff(y_1) as isize / 2, (y_0 + y_1) / 2));

    let (x, y) = possible_coordinates_b
        .chain(possible_coordinates_a)
        .filter(|(x, y)| range.contains(x) && range.contains(y))
        .unique()
        .filter(|(x, y)| {
            pairs.iter().all(|pair| {
                let minimum_beacon_dist = pair.dist();
                let possible_beacon_dist = pair.beacon_x.abs_diff(*x) + pair.beacon_y.abs_diff(*y);
                possible_beacon_dist > minimum_beacon_dist
            })
        })
        .next()
        .unwrap();

    4_000_000 * x + y
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

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_15.txt");

    #[test_case(EXAMPLE, 10, 26)]
    #[test_case(INPUT, 2_000_000, 4717631)]
    fn part_1(input: &str, row_y: isize, expected: usize) {
        assert_eq!(super::part_1(input, row_y), expected);
    }

    #[test_case(EXAMPLE, 0..=20, 56000011)]
    #[test_case(INPUT, 0..=4_000_000, 13197439355220)]
    fn part_2(input: &str, max_var: RangeInclusive<isize>, expected: isize) {
        assert_eq!(super::part_2(input, max_var), expected);
    }
}
