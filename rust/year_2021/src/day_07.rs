pub fn part_1(input: &str) -> usize {
    let mut positions = parse_input(input);
    let middle = positions.len() / 2;
    let median = *positions.select_nth_unstable(middle).1;
    positions.into_iter().map(|p| p.abs_diff(median)).sum()
}

pub fn part_2(input: &str) -> usize {
    let positions = parse_input(input);
    let mean = positions.iter().sum::<usize>() / positions.len();
    [mean, mean + 1]
        .into_iter()
        .map(|target| {
            positions
                .iter()
                .map(|&p| {
                    let d = p.abs_diff(target);
                    d * (d + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    #[test_case("16,1,2,0,4,2,7,1,2,14" => 37)]
    #[test_case(INPUT => 345035)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case("16,1,2,0,4,2,7,1,2,14" => 168)]
    #[test_case(INPUT => 97038163)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
