use itertools::Itertools as _;
use std::iter::{from_fn, once};

pub fn part_1(input: &str) -> usize {
    input_range(input)
        .filter(|&n| {
            let (has_double, asc) = rdigits(n)
                .tuple_windows()
                .fold((false, true), |(double, asc), (b, a)| {
                    (double || a == b, asc && a <= b)
                });
            has_double && asc
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input_range(input)
        .filter(|&n| {
            let (has_double, _, asc) = rdigits(n).chain(once(-1)).tuple_windows().fold(
                (false, 1, true),
                |(double, count, asc), (b, a)| {
                    let has_double = double || (a != b && count == 2);
                    let count = (a == b) as usize * count + 1;
                    (has_double, count, asc && a <= b)
                },
            );
            has_double && asc
        })
        .count()
}

fn input_range(input: &str) -> std::ops::RangeInclusive<usize> {
    let (start, end) = input.split_once('-').unwrap();
    let start = start.parse().unwrap();
    let end = end.parse().unwrap();
    start..=end
}

fn rdigits(mut n: usize) -> impl Iterator<Item = i8> {
    from_fn(move || {
        (n > 0).then(|| {
            let d = (n % 10) as i8;
            n /= 10;
            d
        })
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    #[test_case(INPUT, 495)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, 305)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
