use std::iter::once;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let [diff_1, _, diff_3] =
        once(0)
            .chain(parse_adapters(input))
            .array_windows()
            .fold([0; 3], |mut counts, [a, b]| {
                counts[b - a - 1] += 1;
                counts
            });
    diff_1 * diff_3
}

pub fn part_2(input: &str) -> usize {
    let [.., (_, ways)] = parse_adapters(input).into_iter().fold(
        [(0, 0), (0, 0), (0, 1)],
        |acc @ [_, b, c], adapter| {
            let ways = acc
                .into_iter()
                .skip_while(|&(rating, _)| adapter - rating > 3)
                .map(|(_, ways)| ways)
                .sum();
            [b, c, (adapter, ways)]
        },
    );
    ways
}

fn parse_adapters(input: &str) -> Vec<usize> {
    let mut adapters: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);
    adapters
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    const EXAMPLE_1: &str = "16
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

    const EXAMPLE_2: &str = "28
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

    #[test_case(EXAMPLE_1 => 35)]
    #[test_case(EXAMPLE_2 => 220)]
    #[test_case(INPUT => 2760)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 8)]
    #[test_case(EXAMPLE_2 => 19208)]
    #[test_case(INPUT => 13816758796288)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
