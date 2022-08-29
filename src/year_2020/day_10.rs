use std::collections::VecDeque;

use itertools::Itertools;

pub fn part_1(input: &str) -> u64 {
    let mut counts = [0, 0, 0, 1];
    input
        .lines()
        .map(|joltage| joltage.parse().unwrap())
        .chain(0..=0)
        .sorted_unstable()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .for_each(|diff| counts[diff] += 1);
    let [_, one, _, three] = counts;
    one * three
}

struct Memo {
    joltage: i64,
    arrangements: i64,
}

pub fn part_2(input: &str) -> u64 {
    let mut memory = VecDeque::from([Memo {
        joltage: 0,
        arrangements: 1,
    }]);
    input
        .lines()
        .map(|joltage| joltage.parse().unwrap())
        .sorted_unstable()
        .for_each(|joltage| {
            while memory.front().unwrap().joltage < joltage - 3 {
                memory.pop_front();
            }
            let arrangements = memory.iter().map(|memo| memo.arrangements).sum();
            memory.push_back(Memo {
                joltage,
                arrangements,
            });
        });
    memory.back().unwrap().arrangements as u64
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    static EXAMPLE_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_10.txt").unwrap();
    }

    #[test_case(EXAMPLE_1, 35)]
    #[test_case(EXAMPLE_2, 220)]
    #[test_case(&INPUT, 2760)]
    fn part_1_tests(input: &str, expected: u64) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE_1, 8)]
    #[test_case(EXAMPLE_2, 19208)]
    #[test_case(&INPUT, 13816758796288)]
    fn part_2_tests(input: &str, expected: u64) {
        assert_eq!(part_2(input), expected);
    }
}
