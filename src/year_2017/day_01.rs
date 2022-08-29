pub fn part_1(input: &str) -> u32 {
    let mut a = input.bytes().rev().next().unwrap();
    let mut total = 0;
    for b in input.bytes() {
        if a == b {
            total += (b - b'0') as u32;
        }
        a = b;
    }
    total
}

pub fn part_2(input: &str) -> u32 {
    input
        .bytes()
        .zip(input.bytes().cycle().skip(input.len() / 2))
        .filter_map(|(a, b)| {
            if a == b {
                Some((a - b'0') as u32)
            } else {
                None
            }
        })
        .inspect(|x| println!("{x}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2017/testdata/day_01.txt").unwrap();
    }

    #[test_case("1122", 3)]
    #[test_case("1111", 4)]
    #[test_case("1234", 0)]
    #[test_case("91212129", 9)]
    #[test_case(&INPUT, 1136)]
    fn part_1_tests(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case("1212", 6)]
    #[test_case("1221", 0)]
    #[test_case("123425", 4)]
    #[test_case("123123", 12)]
    #[test_case("12131415", 4)]
    #[test_case(&INPUT, 1092)]
    fn part_2_tests(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
