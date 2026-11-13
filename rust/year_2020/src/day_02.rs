pub fn part_1(input: &str) -> usize {
    parse_input(input)
        .filter(|&(min, max, letter, password)| {
            let count = password.bytes().filter(|&c| c == letter).count();
            (min..=max).contains(&count)
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .filter(|&(i, j, letter, password)| {
            let bytes = password.as_bytes();
            let first = bytes[i - 1] == letter;
            let second = bytes[j - 1] == letter;
            first ^ second
        })
        .count()
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize, u8, &str)> {
    input.lines().map(|line| {
        let (policy, password) = line.split_once(": ").unwrap();
        let (range, letter) = policy.split_once(' ').unwrap();
        let &[letter] = letter.as_bytes() else {
            panic!("expected single letter")
        };
        let (a, b) = range.split_once('-').unwrap();
        (a.parse().unwrap(), b.parse().unwrap(), letter, password)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    const EXAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test_case(EXAMPLE => 2)]
    #[test_case(INPUT => 591)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 1)]
    #[test_case(INPUT => 335)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
