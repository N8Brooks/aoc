use itertools::Itertools as _;
use num::Integer as _;

pub fn part_1(input: &str) -> usize {
    parse_input(input)
        .filter(|&id| {
            let digits = rdigits(id);
            let n = digits.len();
            let (left, right) = digits.split_at(n / 2);
            n.is_even() && left == right
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .filter(|&id| {
            let digits = rdigits(id);
            let n = digits.len();
            debug_assert!(n <= 10, "ID {id} has more than 10 digits");

            // id / n
            if n >= 2 && digits.iter().all_equal() {
                return true;
            }

            // 4 / 2, 6 / 2, 8 / 2, 10 / 2
            let (left, right) = digits.split_at(n / 2);
            if n >= 4 && n.is_even() && left == right {
                return true;
            }

            // 6 / 3, 8 / 4, 10 / 5
            let pairs = &digits.as_chunks::<2>().0;
            if n >= 6 && n.is_even() && pairs.iter().all_equal() {
                return true;
            }

            // 9 / 3
            n == 9 && digits[0..3] == digits[3..6] && digits[0..3] == digits[6..9]
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = usize> {
    input.split(',').flat_map(|line| {
        let (start, stop) = line.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let stop = stop.parse().unwrap();
        start..=stop
    })
}

fn rdigits(mut n: usize) -> Vec<usize> {
    let mut digits = Vec::with_capacity(10);
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    #[test_case(EXAMPLE => 1227775554)]
    #[test_case(INPUT => 23701357374)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 4174379265)]
    #[test_case(INPUT => 34284458938)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
