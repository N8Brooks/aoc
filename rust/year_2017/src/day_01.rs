pub fn part_1(input: &str) -> u32 {
    solve(input, 1)
}

pub fn part_2(input: &str) -> u32 {
    solve(input, input.trim().len() / 2)
}

fn solve(input: &str, offset: usize) -> u32 {
    let digits = input.trim().bytes();
    digits
        .clone()
        .zip(digits.cycle().skip(offset))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| u32::from(a - b'0'))
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_01.txt");

    #[test_case("1122" => 3)]
    #[test_case("1111" => 4)]
    #[test_case("1234" => 0)]
    #[test_case("91212129" => 9)]
    #[test_case(INPUT => 1136)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case("1212" => 6)]
    #[test_case("1221" => 0)]
    #[test_case("123425" => 4)]
    #[test_case("123123" => 12)]
    #[test_case("12131415" => 4)]
    #[test_case(INPUT => 1092)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
