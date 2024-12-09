fn from_snafu(input: &str) -> isize {
    input
        .bytes()
        .rev()
        .enumerate()
        .map(|(i, byte)| {
            let num = match byte {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                _ => panic!("unknown byte {byte}"),
            };
            isize::pow(5, i as u32) * num
        })
        .sum()
}

fn to_snafu(num: &isize) -> String {
    let mut num = *num;
    let mut output = String::new();
    while num != 0 {
        let place = match num % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                num += 5;
                '='
            }
            4 => {
                num += 5;
                '-'
            }
            n => panic!("unknown n {n}"),
        };
        output.push(place);
        num /= 5;
    }
    output.chars().rev().collect()
}

pub fn part_1(input: &str) -> String {
    to_snafu(&input.lines().map(from_snafu).sum())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_25.txt");

    #[test_case("1", 1)]
    #[test_case("2", 2)]
    #[test_case("1=", 3)]
    #[test_case("1-", 4)]
    #[test_case("10", 5)]
    #[test_case("11", 6)]
    #[test_case("12", 7)]
    #[test_case("2=", 8)]
    #[test_case("2-", 9)]
    #[test_case("20", 10)]
    #[test_case("1=0", 15)]
    #[test_case("1-0", 20)]
    #[test_case("1=11-2", 2022)]
    #[test_case("1-0---0", 12345)]
    #[test_case("1121-1110-1=0", 314159265)]
    fn from_snafu(input: &str, expected: isize) {
        assert_eq!(super::from_snafu(input), expected);
    }

    #[test_case(1, "1")]
    #[test_case(2, "2")]
    #[test_case(3, "1=")]
    #[test_case(4, "1-")]
    #[test_case(5, "10")]
    #[test_case(6, "11")]
    #[test_case(7, "12")]
    #[test_case(8, "2=")]
    #[test_case(9, "2-")]
    #[test_case(10, "20")]
    #[test_case(15, "1=0")]
    #[test_case(20, "1-0")]
    #[test_case(2022, "1=11-2")]
    #[test_case(12345, "1-0---0")]
    #[test_case(314159265, "1121-1110-1=0")]
    fn to_snafu(input: isize, expected: &str) {
        assert_eq!(super::to_snafu(&input), expected);
    }

    #[test_case(EXAMPLE, "2=-1=0")]
    #[test_case(INPUT, "2=0=02-0----2-=02-10")]
    fn part_1(input: &str, expected: &str) {
        assert_eq!(super::part_1(input), expected);
    }
}
