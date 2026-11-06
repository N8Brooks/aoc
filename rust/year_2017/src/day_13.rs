pub fn part_1(input: &str) -> usize {
    parse_scanners(input)
        .filter(|(depth, range)| depth % ((range - 1) * 2) == 0)
        .map(|(depth, range)| depth * range)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut scanners: Vec<_> = parse_scanners(input)
        .map(|(depth, range)| (depth, 2 * (range - 1)))
        .collect();
    scanners.sort_unstable_by_key(|(_, period)| *period);
    (0..)
        .find(|delay| {
            scanners
                .iter()
                .all(|(depth, period)| (depth + delay) % period != 0)
        })
        .unwrap()
}

fn parse_scanners(input: &str) -> impl Iterator<Item = (usize, usize)> {
    input.lines().map(|line| {
        let (depth, range) = line.split_once(": ").unwrap();
        (depth.parse().unwrap(), range.parse().unwrap())
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "0: 3
1: 2
4: 4
6: 4";

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    #[test_case(EXAMPLE => 24)]
    #[test_case(INPUT => 2604)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 10)]
    #[test_case(INPUT => 3941460)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
