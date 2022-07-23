fn pass_id(pass: &str) -> u32 {
    pass.chars().fold(0, |i, c| match c {
        'L' | 'F' => 2 * i,
        'R' | 'B' => 2 * i + 1,
        _ => panic!("Invalid characters on pass"),
    })
}

pub fn part_1(input: &str) -> u32 {
    input.split('\n').map(pass_id).max().unwrap()
}

/// For the un-sorted ints, the missing value can be
/// found in O(n) time and O(1) extra space. The sum
/// with the missing number included can be found
/// using the arithmetic sum of the series. The sum
/// without the missing number included can be found
/// as the sum of the digits. Subtracting these gives
/// the result.
/// ```rs
/// let count = nums.count();
/// let min = nums.min();
/// let max = nums.max();
/// let sum_with_missing = (count + 1) * (min + max) / 2;
/// let sum = nums.sum();
/// let missing = sum_with_missing - sum;
/// ```
///
/// Equivalent to all of the following:
/// ```rs
/// (nums.count +  1) * (min + max) / 2 - nums.sum();
/// (count + 1) * (min + max) / 2 - sum
/// (count + 1) * (min +min + count) / 2 - sum
/// (max - min + 1) * (min + max) / 2 - sum
/// (count + 1) * (max + max - count) / 2 - sum
/// ```
pub fn part_2<'a>(input: &'a str) -> u32 {
    let mut min = u32::MAX;
    let mut max = u32::MIN;
    let mut sum = 0;
    for id in input.split('\n').map(pass_id) {
        min = min.min(id);
        max = max.max(id);
        sum += id;
    }
    (max - min + 1) * (min + max) / 2 - sum
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_05.txt").unwrap();
    }

    #[test_case("FBFBBFFRLR", 357)]
    #[test_case("BFFFBBFRRR", 567)]
    #[test_case("FFFBBBFRRR", 119)]
    #[test_case("BBFFBBFRLL", 820)]
    #[test_case(&INPUT, 953)]
    fn part_1_examples(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(&INPUT, 615)]
    fn part_2_examples(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
