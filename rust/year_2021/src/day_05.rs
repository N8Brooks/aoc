use num::{signum, Complex};
use std::collections::HashMap;

fn parse_line(line: &str) -> (Complex<isize>, Complex<isize>) {
    let (start, stop) = line.split_once(" -> ").unwrap();
    (parse_point(start), parse_point(stop))
}

fn parse_point(point: &str) -> Complex<isize> {
    let (x, y) = point.split_once(',').unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    Complex::new(x, y)
}

fn count_overlaps(iter: impl IntoIterator<Item = (Complex<isize>, Complex<isize>)>) -> usize {
    let mut overlaps: HashMap<Complex<isize>, usize> = HashMap::new();
    for (start, stop) in iter {
        let mut cursor = start;
        let delta = Complex::new(signum(stop.re - start.re), signum(stop.im - start.im));
        if cursor != stop {
            *overlaps.entry(cursor).or_default() += 1;
        }
        while cursor != stop {
            cursor += delta;
            *overlaps.entry(cursor).or_default() += 1;
        }
    }
    overlaps.values().filter(|&count| *count > 1).count()
}

pub fn part_1(input: &str) -> usize {
    let point_pairs = input
        .lines()
        .map(parse_line)
        .filter(|(start, stop)| start.re == stop.re || start.im == stop.im);
    count_overlaps(point_pairs)
}

pub fn part_2(input: &str) -> usize {
    count_overlaps(input.lines().map(parse_line))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

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

    const INPUT: &str = include_str!("../../../test_data/year_2021/day_05.txt");

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
