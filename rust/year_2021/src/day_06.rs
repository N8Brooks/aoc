pub fn part_1(input: &str) -> u64 {
    simulate(input, 80)
}

pub fn part_2(input: &str) -> u64 {
    simulate(input, 256)
}

fn simulate(input: &str, days: usize) -> u64 {
    let mut counts = [0; 9];
    for counter in input.split(',') {
        let counter: usize = counter.parse().unwrap();
        counts[counter] += 1;
    }
    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test_case(EXAMPLE => 5934)]
    #[test_case(INPUT => 389726)]
    fn part_1(input: &str) -> u64 {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 26984457539)]
    #[test_case(INPUT => 1743335992042)]
    fn part_2(input: &str) -> u64 {
        super::part_2(input)
    }
}
