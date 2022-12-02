pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|cals| {
            cals.lines()
                .map(|cals| cals.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut calories: Vec<_> = input
        .split("\n\n")
        .map(|cals| {
            cals.lines()
                .map(|cals| cals.parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    calories.sort_unstable();
    calories[calories.len() - 3..].iter().sum()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    static EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    static INPUT: &str = include_str!("testdata/day_01.txt");

    #[test_case(EXAMPLE, 24000)]
    #[test_case(INPUT, 68802)]
    fn part_2(input: &str, actual: usize) {
        assert_eq!(super::part_1(input), actual);
    }

    #[test_case(EXAMPLE, 45000)]
    #[test_case(INPUT, 205370)]
    fn part_1(input: &str, actual: usize) {
        assert_eq!(super::part_2(input), actual);
    }
}
