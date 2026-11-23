use hashbrown::DefaultHashBuilder;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let counts = parse_input(input);
    counts
        .keys()
        .map(|a| counts.iter().map(|(&b, count)| a.abs_diff(b) * count).sum())
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let counts = parse_input(input);
    let (&min, &max) = counts.keys().minmax().into_option().unwrap();
    (min..max + 1)
        .map(|a| {
            counts
                .iter()
                .map(|(&b, count)| {
                    let n = a.abs_diff(b);
                    let fuel_cost = n * (n + 1) / 2;
                    fuel_cost * count
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> std::collections::HashMap<usize, usize, DefaultHashBuilder> {
    input
        .split(',')
        .map(|x| x.parse().unwrap())
        .counts_with_hasher(DefaultHashBuilder::default())
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
