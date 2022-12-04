use std::collections::VecDeque;

fn simulate(input: &str, days: usize) -> usize {
    let mut counts = VecDeque::from(vec![0; 9]);
    for counter in input.split(',') {
        let counter: usize = counter.parse().unwrap();
        counts[counter] += 1;
    }
    for _ in 0..days {
        counts[7] += counts[0];
        counts.rotate_left(1);
    }
    counts.iter().sum()
}

pub fn part_1(input: &str) -> usize {
    simulate(input, 80)
}

pub fn part_2(input: &str) -> usize {
    simulate(input, 256)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "3,4,3,1,2";

    const INPUT: &str = include_str!("../../../testdata/year_2021/day_06.txt");

    #[test_case(EXAMPLE, 5934)]
    #[test_case(INPUT, 389726)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 26984457539)]
    #[test_case(INPUT, 1743335992042)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
