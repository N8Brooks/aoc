use std::{
    cmp::Ordering,
    ops::{Add as _, Mul as _},
};

pub fn part_1(input: &str) -> usize {
    operate(input, &[usize::add, usize::mul])
}

pub fn part_2(input: &str) -> usize {
    operate(input, &[usize::add, usize::mul, concat])
}

pub fn operate(input: &str, ops: &[fn(usize, usize) -> usize]) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (test_value, nums) = line.split_once(": ").unwrap();
            let test_value = test_value.parse().unwrap();
            let nums: Vec<usize> = nums.split(' ').map(|num| num.parse().unwrap()).collect();
            let mut stack = vec![(nums[0], 1)];
            while let Some((acc, i)) = stack.pop() {
                match acc.cmp(&test_value) {
                    Ordering::Less => {
                        if i < nums.len() {
                            stack.extend(ops.iter().map(|op| (op(acc, nums[i]), i + 1)));
                        }
                    }
                    Ordering::Equal => return Some(test_value),
                    Ordering::Greater => {}
                }
            }
            None
        })
        .sum()
}

fn concat(a: usize, b: usize) -> usize {
    let shift = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10_usize.pow(shift) + b
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../test_data/year_2024/day_07.txt");

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test_case(EXAMPLE, 3749)]
    #[test_case(INPUT, 5512534574980)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 11387)]
    #[test_case(INPUT, 328790210468594)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }

    #[test_case(15, 6, 156)]
    #[test_case(48, 6, 486)]
    #[test_case(10, 10, 1010)]
    #[test_case(0, 1, 1)]
    #[test_case(1, 0, 10)]
    #[test_case(0, 0, 0)]
    fn concat(a: usize, b: usize, expected: usize) {
        assert_eq!(super::concat(a, b), expected);
    }
}
