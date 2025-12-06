use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    parse_input(input)
        .filter(|&id| {
            let digits = rdigits(id);
            digits.len() & 1 == 0 && digits[..digits.len() / 2] == digits[digits.len() / 2..]
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .filter(|&id| {
            let digits = rdigits(id);
            let n = digits.len();
            debug_assert!(n <= 10, "ID {} has more than 10 digits", id);
            n >= 2 && digits.iter().all_equal()
                || n >= 4 && n & 1 == 0 && digits[..digits.len() / 2] == digits[digits.len() / 2..]
                || n >= 6 && n & 1 == 0 && digits.as_chunks::<2>().0.iter().all_equal()
                || n == 9 && digits[0..3] == digits[3..6] && digits[0..3] == digits[6..9]
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = usize> {
    input.split(',').flat_map(|line| {
        let (start, stop) = line.split_once('-').unwrap();
        let start: usize = start.parse().unwrap();
        let stop: usize = stop.parse().unwrap();
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

    #[test_case(EXAMPLE, 1227775554)]
    #[test_case(INPUT, 23701357374)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 4174379265)]
    #[test_case(INPUT, 34284458938)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
