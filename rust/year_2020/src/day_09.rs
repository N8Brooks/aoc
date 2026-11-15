use itertools::{Itertools as _, MinMaxResult::MinMax};

pub fn part_1(input: &str) -> usize {
    let nums = parse_nums(input);
    missing::<25>(&nums)
}

pub fn part_2(input: &str) -> usize {
    find_contiguous_sum::<25>(input)
}

fn parse_nums(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn missing<const N: usize>(nums: &[usize]) -> usize {
    let mut nums = nums.iter().copied();
    let mut preamble: [usize; N] = nums.next_array().unwrap();
    let mut i = 0;
    nums.find(|&num| {
        let is_sum = preamble
            .into_iter()
            .array_combinations()
            .any(|[a, b]| a + b == num);
        preamble[i] = num;
        i = (i + 1) % N;
        !is_sum
    })
    .unwrap()
}

fn find_contiguous_sum<const N: usize>(input: &str) -> usize {
    let nums = parse_nums(input);
    let target = missing::<N>(&nums);
    let mut total = 0;
    let mut i = 0;
    nums.iter()
        .enumerate()
        .find_map(|(j, num)| {
            total += num;
            while total > target {
                total -= nums[i];
                i += 1;
            }
            if total == target
                && let MinMax(min, max) = nums[i..=j].iter().minmax()
            {
                Some(min + max)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    const EXAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test_case(INPUT => 22477624)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 127)]
    fn missing(input: &str) -> usize {
        let nums = super::parse_nums(input);
        super::missing::<5>(&nums)
    }

    #[test_case(INPUT => 2980044)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }

    #[test_case(EXAMPLE => 62)]
    fn find_contiguous_sum(input: &str) -> usize {
        super::find_contiguous_sum::<5>(input)
    }
}
