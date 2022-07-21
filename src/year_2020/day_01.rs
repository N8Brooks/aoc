use std::{cmp::Ordering, collections::HashSet};

pub fn part_1(input: &str) -> i32 {
    let mut memo = HashSet::new();
    for line in input.split('\n') {
        let num: i32 = line.parse().unwrap();
        let complement = 2020 - num;
        if memo.contains(&complement) {
            return num * complement;
        }
        memo.insert(num);
    }
    panic!("No valid 2-sum");
}

pub fn part_2(input: &str) -> i32 {
    let mut nums: Vec<i32> = input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();
    nums.sort();
    for (i, x) in nums.iter().enumerate() {
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        let complement = 2020 - x;
        while j < k {
            let two_sum = nums[j] + nums[k];
            match two_sum.cmp(&complement) {
                Ordering::Less => j += 1,
                Ordering::Greater => k -= 1,
                Ordering::Equal => return x * nums[j] * nums[k],
            }
        }
    }
    panic!("No valid 3-sum");
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::{part_1, part_2};
    use std::fs::read_to_string;

    static EXAMPLE: &str = "1721
979
366
299
675
1456";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE), 514579);
    }

    #[test]
    fn part_1_input() {
        let input = read_to_string("src/year_2020/testdata/day_01.txt").unwrap();
        assert_eq!(part_1(&input), 482811);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE), 241861950);
    }

    #[test]
    fn part_2_input() {
        let input = read_to_string("src/year_2020/testdata/day_01.txt").unwrap();
        assert_eq!(part_2(&input), 193171814);
    }
}
