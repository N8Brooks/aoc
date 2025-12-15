pub fn part_1(input: &str) -> u32 {
    parse_input(input).map(|mass| mass / 3 - 2).sum()
}

pub fn part_2(input: &str) -> u32 {
    parse_input(input)
        .map(|mut fuel| {
            let mut total = 0;
            while fuel >= 9 {
                fuel = fuel / 3 - 2;
                total += fuel;
            }
            total
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = u32> {
    input.lines().map(|line| line.parse().unwrap())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_01.txt");

    #[test_case("12" => 2)]
    #[test_case("14" => 2)]
    #[test_case("1969" => 654)]
    #[test_case("100756" => 33583)]
    #[test_case(INPUT => 3408471)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case("14" => 2)]
    #[test_case("1969" => 966)]
    #[test_case("100756" => 50346)]
    #[test_case(INPUT => 5109803)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
