use std::collections::HashMap;

use num::Integer;

fn find_weakness(nums: &Vec<i64>, n: usize) -> i64 {
    let mut counts = HashMap::new();

    nums.iter().take(n).for_each(|num| {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    for (old_num, new_num) in nums.iter().zip(nums[n..].iter()) {
        // Check counts for those that sum to new_num
        let half = new_num / 2;
        if new_num.is_even() && counts.contains_key(&half) && counts[&half] >= 2 {
            // pass
        } else if counts
            .keys()
            .filter(|&num| *num != new_num)
            .any(|&num| counts.contains_key(&(new_num - num)))
        {
            // pass
        } else {
            return *new_num;
        }

        // Remove old_num
        let count = counts[old_num];
        if count == 1 {
            counts.remove(old_num);
        } else {
            counts.insert(old_num, count - 1);
        }

        // Insert new_num
        counts
            .entry(new_num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    panic!("No weakness found")
}

pub fn part_1(input: &str, n: usize) -> i64 {
    let nums: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    find_weakness(&nums, n)
}

pub fn part_2<'a>(input: &'a str, n: usize) -> i64 {
    let nums: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let weakness = find_weakness(&nums, n);
    let (mut i, mut j) = (0, 0);
    let mut range_total = 0;
    loop {
        match range_total.cmp(&weakness) {
            std::cmp::Ordering::Less => {
                range_total += nums[j];
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                let min = nums[i..j].iter().min().unwrap();
                let max = nums[i..j].iter().max().unwrap();
                return min + max;
            }
            std::cmp::Ordering::Greater => {
                range_total -= nums[i];
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "35
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

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_09.txt").unwrap();
    }

    #[test_case(EXAMPLE, 127, 5)]
    #[test_case(&INPUT, 22477624, 25)]
    fn part_1_examples(input: &str, expected: i64, n: usize) {
        assert_eq!(part_1(input, n), expected);
    }

    #[test_case(EXAMPLE, 62, 5)]
    #[test_case(&INPUT, 2980044, 25)]
    fn part_2_examples(input: &str, expected: i64, n: usize) {
        assert_eq!(part_2(input, n), expected);
    }
}
