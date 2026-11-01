use hashbrown::DefaultHashBuilder;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .all_unique_with_hasher(DefaultHashBuilder::default())
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .map(|word| {
                    word.bytes().fold([0; 26], |mut counts, b| {
                        counts[(b - b'a') as usize] += 1;
                        counts
                    })
                })
                .all_unique_with_hasher(DefaultHashBuilder::default())
        })
        .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    #[test_case("aa bb cc dd ee" => 1)]
    #[test_case("aa bb cc dd aa" => 0)]
    #[test_case("aa bb cc dd aaa" => 1)]
    #[test_case(INPUT => 325)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case("abcde fghij" => 1)]
    #[test_case("abcde xyz ecdab" => 0)]
    #[test_case("a ab abc abd abf abj" => 1)]
    #[test_case("iiii oiii ooii oooi oooo" => 1)]
    #[test_case("oiii ioii iioi iiio" => 0)]
    #[test_case(INPUT => 119)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
