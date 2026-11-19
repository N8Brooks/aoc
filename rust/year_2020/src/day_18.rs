pub fn part_1(input: &str) -> u64 {
    use crate::day_18::Operator::*;
    let mut stack = Vec::new();
    input
        .lines()
        .map(|line| {
            let (_, acc) = line.bytes().fold((Add, 0), |(op, acc), b| match b {
                b'0'..=b'9' => (Add, op.apply(acc, (b - b'0').into())),
                b' ' => (op, acc),
                b'+' => (Add, acc),
                b'*' => (Mul, acc),
                b'(' => {
                    stack.push((op, acc));
                    (Add, 0)
                }
                b')' => {
                    let (op, outer) = stack.pop().expect("unexpected close");
                    (Add, op.apply(outer, acc))
                }
                _ => panic!("invalid token {}", b as char),
            });
            assert!(stack.is_empty(), "expected close");
            acc
        })
        .sum()
}

enum Operator {
    Add,
    Mul,
}

impl Operator {
    #[inline(always)]
    fn apply(self, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Mul => left * right,
        }
    }
}

pub fn part_2(input: &str) -> u64 {
    let mut stack = Vec::new();
    input
        .lines()
        .map(|line| {
            let (prod, sum) = line.bytes().fold((1, 0), |(prod, sum), b| match b {
                b'0'..=b'9' => (prod, sum + u64::from(b - b'0')),
                b'+' | b' ' => (prod, sum),
                b'*' => (prod * sum, 0),
                b'(' => {
                    stack.push((prod, sum));
                    (1, 0)
                }
                b')' => {
                    let (outer_prod, outer_sum) = stack.pop().expect("unexpected close");
                    (outer_prod, outer_sum + prod * sum)
                }
                _ => panic!("invalid token {}", b as char),
            });
            assert!(stack.is_empty(), "expected close");
            prod * sum
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    #[test_case("1 + 2 * 3 + 4 * 5 + 6" => 71)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))" => 51)]
    #[test_case("2 * 3 + (4 * 5)" => 26)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)" => 437)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 12240)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 13632)]
    #[test_case(INPUT => 8929569623593)]
    fn part_1(input: &str) -> u64 {
        super::part_1(input)
    }

    #[test_case("1 + 2 * 3 + 4 * 5 + 6" => 231)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))" => 51)]
    #[test_case("2 * 3 + (4 * 5)" => 46)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)" => 1445)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 669060)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 23340)]
    #[test_case(INPUT => 231235959382961)]
    fn part_2(input: &str) -> u64 {
        super::part_2(input)
    }
}
