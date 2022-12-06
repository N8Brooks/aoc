use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .bytes()
        .tuple_windows()
        .position(|(a, b, c, d)| [a, b, c, d].iter().all_unique())
        .unwrap()
        + 4
}

pub fn part_2(input: &str) -> usize {
    // Definitely not optimal runtime complexity :/
    let input = input.as_bytes();
    (0..input.len())
        .zip(14..input.len())
        .map(|(i, j)| &input[i..j])
        .position(|slice| slice.iter().all_unique())
        .unwrap()
        + 14
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_06.txt");

    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    #[test_case(INPUT, 1929)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    #[test_case(INPUT, 3298)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
