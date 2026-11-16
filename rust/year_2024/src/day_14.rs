use std::cmp::Ordering::*;

use itertools::Itertools as _;
use num::Integer as _;

pub fn part_1(input: &str, m: i32, n: i32) -> i32 {
    let mid_m = m / 2;
    let mid_n = n / 2;
    parse_input(input)
        .filter_map(|(i, j, di, dj)| {
            let i = (i + di * 100).rem_euclid(m);
            let j = (j + dj * 100).rem_euclid(n);
            match (i.cmp(&mid_m), j.cmp(&mid_n)) {
                (Less, Less) => Some(0),
                (Less, Greater) => Some(1),
                (Greater, Less) => Some(2),
                (Greater, Greater) => Some(3),
                _ => None,
            }
        })
        .fold([0; 4], |mut acc, q| {
            acc[q] += 1;
            acc
        })
        .into_iter()
        .product()
}

pub fn part_2(input: &str, m: i32, n: i32) -> i32 {
    let (is, js, dis, djs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = parse_input(input).multiunzip();

    let [a, b] = [(is, dis, m), (js, djs, n)].map(|(xs, dxs, size)| {
        (0..size)
            .min_by_key(|&t| {
                let (sum, sum_sq) = xs
                    .iter()
                    .zip(&dxs)
                    .map(|(&x, &dx)| (x + dx * t).rem_euclid(size))
                    .fold((0, 0), |(sum, sq), x| (sum + x, sq + x * x));

                xs.len() as i32 * sum_sq - sum * sum
            })
            .unwrap()
    });

    // Chinese Remainder Theorem
    let inverse = m.extended_gcd(&n).x.rem_euclid(n);
    a + m * ((b - a).rem_euclid(n) * inverse).rem_euclid(n)
}

fn parse_input(input: &str) -> impl Iterator<Item = (i32, i32, i32, i32)> + '_ {
    input.lines().map(|line| {
        let (p, v) = line.split_once(" ").unwrap();
        let p = p.strip_prefix("p=").unwrap();
        let v = v.strip_prefix("v=").unwrap();
        let (pj, pi) = p.split_once(",").unwrap();
        let (vj, vi) = v.split_once(",").unwrap();
        (
            pi.parse().unwrap(),
            pj.parse().unwrap(),
            vi.parse().unwrap(),
            vj.parse().unwrap(),
        )
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test_case(EXAMPLE, 7, 11, 12)]
    #[test_case(INPUT, 103, 101, 230900224)]
    fn part_1(input: &str, m: i32, n: i32, expected: i32) {
        assert_eq!(super::part_1(input, m, n), expected);
    }

    #[test_case(INPUT, 103, 101, 6532)]
    fn part_2(input: &str, m: i32, n: i32, expected: i32) {
        assert_eq!(super::part_2(input, m, n), expected);
    }
}
