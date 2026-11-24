use std::iter::successors;

use hashbrown::DefaultHashBuilder;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let point_pairs = input
        .lines()
        .map(parse_line)
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);
    count_overlaps(point_pairs)
}

pub fn part_2(input: &str) -> usize {
    count_overlaps(input.lines().map(parse_line))
}

fn parse_line(line: &str) -> ((usize, usize), (usize, usize)) {
    let (start, stop) = line.split_once(" -> ").unwrap();
    (parse_point(start), parse_point(stop))
}

fn parse_point(point: &str) -> (usize, usize) {
    let (x, y) = point.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn count_overlaps(lines: impl IntoIterator<Item = ((usize, usize), (usize, usize))>) -> usize {
    lines
        .into_iter()
        .flat_map(|((x1, y1), (x2, y2))| {
            let dx = x2.cmp(&x1) as isize;
            let dy = y2.cmp(&y1) as isize;
            successors(Some((x1, y1)), move |&(x, y)| {
                ((x, y) != (x2, y2)).then(|| (x.strict_add_signed(dx), y.strict_add_signed(dy)))
            })
        })
        .counts_with_hasher(DefaultHashBuilder::default())
        .into_values()
        .filter(|&count| count >= 2)
        .count()
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
