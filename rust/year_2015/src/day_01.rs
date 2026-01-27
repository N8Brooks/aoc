pub fn part_1(input: &str) -> isize {
    parse_input(input).sum()
}

pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .scan(0, |floor, diff| {
            *floor += diff;
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .expect("reaches basement")
        + 1
}

fn parse_input(input: &str) -> impl Iterator<Item = isize> {
    input.bytes().map(|b| match b {
        b'(' => 1,
        b')' => -1,
        _ => panic!("unexpected byte {b}"),
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_01.txt");

    #[test_case("(())" => 0; "example 1")]
    #[test_case("()()" => 0; "example 2")]
    #[test_case("(((" => 3; "example 3")]
    #[test_case("(()(()(" => 3; "example 4")]
    #[test_case("))(((((" => 3; "example 5")]
    #[test_case("())" => -1; "example 6")]
    #[test_case("))(" => -1; "example 7")]
    #[test_case(")))" => -3; "example 8")]
    #[test_case(")())())" => -3; "example 9")]
    #[test_case(INPUT => 232)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(")" => 1)]
    #[test_case("()())" => 5)]
    #[test_case(INPUT => 1783)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
