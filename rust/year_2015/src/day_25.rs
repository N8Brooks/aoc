use std::num::NonZeroU64;

pub fn part_1(input: &str) -> u64 {
    let (i, j) = parse_input(input);
    let n = pair(i.get() - 1, j.get() - 1);
    20151125 * modpow(252533, n, 33554393) % 33554393
}

fn parse_input(input: &str) -> (NonZeroU64, NonZeroU64) {
    let (i, j) = input
        .trim()
        .strip_circumfix(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
            '.',
        )
        .unwrap()
        .split_once(", column ")
        .unwrap();
    (i.parse().unwrap(), j.parse().unwrap())
}

/// Cantor pairing function,  0-indexed
fn pair(i: u64, j: u64) -> u64 {
    (i + j) * (i + j + 1) / 2 + j
}

fn modpow(mut base: u64, mut exp: u64, n: u64) -> u64 {
    if n == 1 {
        return 0;
    }
    let mut result = 1 % n;
    base %= n;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % n;
        }
        base = (base * base) % n;
        exp >>= 1;
    }
    result
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_25.txt");

    #[test_case(INPUT => 19980801)]
    fn part_1(input: &str) -> u64 {
        super::part_1(input)
    }

    //   | 1   2   3   4   5   6
    // ---+---+---+---+---+---+---+
    // 1 |  1   3   6  10  15  21
    // 2 |  2   5   9  14  20
    // 3 |  4   8  13  19
    // 4 |  7  12  18
    // 5 | 11  17
    // 6 | 16
    #[test_case((1, 1) => 1)]
    #[test_case((1, 2) => 3)]
    #[test_case((1, 3) => 6)]
    #[test_case((1, 4) => 10)]
    #[test_case((1, 5) => 15)]
    #[test_case((1, 6) => 21)]
    #[test_case((2, 1) => 2)]
    #[test_case((2, 2) => 5)]
    #[test_case((2, 3) => 9)]
    #[test_case((2, 4) => 14)]
    #[test_case((2, 5) => 20)]
    #[test_case((3, 1) => 4)]
    #[test_case((3, 2) => 8)]
    #[test_case((3, 3) => 13)]
    #[test_case((3, 4) => 19)]
    #[test_case((4, 1) => 7)]
    #[test_case((4, 2) => 12)]
    #[test_case((4, 3) => 18)]
    #[test_case((5, 1) => 11)]
    #[test_case((5, 2) => 17)]
    #[test_case((6, 1) => 16)]
    fn pair((i, j): (u64, u64)) -> u64 {
        super::pair(i - 1, j - 1) + 1
    }
}
