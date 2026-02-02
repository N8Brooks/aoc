use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let bytes = line.as_bytes();
            let vowel_count = bytes
                .iter()
                .filter(|b| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u'))
                .count();
            let has_double = bytes.array_windows().any(|[a, b]| a == b);
            let has_forbidden = bytes
                .array_windows()
                .any(|pair| matches!(pair, b"ab" | b"cd" | b"pq" | b"xy"));
            vowel_count >= 3 && has_double && !has_forbidden
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let bytes = line.as_bytes();
            let has_pair = bytes
                .array_windows::<2>()
                .enumerate()
                .any(|(i, pair)| bytes[i + 2..].array_windows().contains(pair));
            let has_repeat = bytes.array_windows().any(|[a, _, c]| a == c);
            has_pair && has_repeat
        })
        .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    #[test_case("ugknbfddgicrmopn" => 1)]
    #[test_case("aaa" => 1)]
    #[test_case("jchzalrnumimnmhp" => 0)]
    #[test_case("haegwjzuvuyypxyu" => 0)]
    #[test_case("dvszwmarrgswjxmb" => 0)]
    #[test_case(INPUT => 238)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case("qjhvhtzxzqqjkmpb" => 1)]
    #[test_case("xxyxx" => 1)]
    #[test_case("uurcxstgmygtbstg" => 0)]
    #[test_case("ieodomkazucvgmuy" => 0)]
    #[test_case(INPUT => 69)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
