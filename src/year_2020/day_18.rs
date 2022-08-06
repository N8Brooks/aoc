use num::Num;

fn add<T: Num>(a: T, b: T) -> T {
    a + b
}

fn mul<T: Num>(a: T, b: T) -> T {
    a * b
}

fn evaluate_part_1(expression: &str) -> i64 {
    let mut operations = Vec::new();
    let mut operand = 0;
    let mut operate: fn(i64, i64) -> i64 = add;
    expression.bytes().for_each(|token| match token {
        b' ' => (),
        b'+' => operate = add,
        b'*' => operate = mul,
        b'(' => {
            operations.push((operand, operate));
            operand = 0;
            operate = add;
        }
        b')' => {
            let (result, operate) = operations.pop().unwrap();
            operand = operate(result, operand);
        }
        digit @ b'0'..=b'9' => {
            operand = operate(operand, (digit - b'0') as i64);
        }
        _ => panic!("Unknown token encountered"),
    });
    operand
}

pub fn part_1(input: &str) -> i64 {
    input.lines().map(evaluate_part_1).sum()
}

fn evaluate_part_2(expression: &str) -> i64 {
    let mut operations = Vec::new();
    let mut operand = 0;
    let mut operate = b'+';
    expression.bytes().for_each(|token| match token {
        b' ' => (),
        b'+' => operate = b'+',
        b'*' => {
            operations.push(Some((operand, b'*')));
            operand = 0;
            operate = b'+';
        }
        b'(' => {
            operations.push(Some((operand, operate)));
            operations.push(None);
            operand = 0;
            operate = b'+';
        }
        b')' => {
            while let Some((result, _)) = operations.pop().unwrap() {
                operand *= result;
            }
            if operations.last().unwrap().unwrap().1 == b'+' {
                operand += operations.pop().unwrap().unwrap().0;
            }
        }
        digit @ b'0'..=b'9' => {
            operand += (digit - b'0') as i64;
        }
        _ => panic!("Unknown token encountered"),
    });
    operations
        .iter()
        .map(|operation| operation.unwrap())
        .map(|(operand, _)| operand)
        .product::<i64>()
        * operand
}

pub fn part_2(input: &str) -> i64 {
    input.lines().map(evaluate_part_2).sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_18.txt").unwrap();
    }

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 71)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 26)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)]
    #[test_case(&INPUT, 8929569623593)]
    fn part_1_tests(input: &str, expected: i64) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 231)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 46)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)]
    #[test_case(&INPUT, 231235959382961)]
    fn part_2_tests(input: &str, expected: i64) {
        assert_eq!(part_2(input), expected);
    }
}
