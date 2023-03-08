pub use part_1_context::part_1;
pub use part_2_context::part_2;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(char: &str) -> Result<Self, Self::Error> {
        match char {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "/" => Ok(Operator::Div),
            "*" => Ok(Operator::Mul),
            _ => Err(format!("unknown operator {char}")),
        }
    }
}

impl Operator {
    fn operate(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Div => a / b,
            Operator::Mul => a * b,
        }
    }
}

mod part_1_context {
    use std::collections::HashMap;

    use super::Operator;

    pub fn part_1(input: &str) -> i64 {
        let monkeys: HashMap<&str, Expression> = input
            .lines()
            .map(|line| {
                let (name, operation) = line.split_once(": ").unwrap();
                let operation = Expression::from(operation);
                (name, operation)
            })
            .collect();
        monkeys["root"].evaluate(&monkeys)
    }

    #[derive(Debug)]
    enum Expression<'a> {
        Number(i64),
        Expressions(&'a str, Operator, &'a str),
    }

    impl<'a> From<&'a str> for Expression<'a> {
        fn from(input: &'a str) -> Self {
            if let Ok(num) = input.parse() {
                Expression::Number(num)
            } else {
                let mut tokens = input.split(' ');
                Expression::Expressions(
                    tokens.next().expect("first token"),
                    Operator::try_from(tokens.next().expect("second token")).unwrap(),
                    tokens.next().expect("third token"),
                )
            }
        }
    }

    impl<'a> Expression<'a> {
        fn evaluate(&self, monkeys: &HashMap<&'a str, Expression<'a>>) -> i64 {
            match self {
                Expression::Number(num) => *num,
                Expression::Expressions(a, operator, b) => {
                    let a = monkeys[a].evaluate(monkeys);
                    let b = monkeys[b].evaluate(monkeys);
                    operator.operate(a, b)
                }
            }
        }
    }
}

mod part_2_context {
    use std::collections::HashMap;

    use itertools::{Either, Itertools};

    use super::Operator;

    pub fn part_2(input: &str) -> i64 {
        let (equalities, monkeys) = parse_input(input);
        let (a, b) = equalities.iter().exactly_one().expect("one equality");
        let a = monkeys[a].evaluate(&monkeys);
        let b = monkeys[b].evaluate(&monkeys);
        let (x, c) = match (a, b) {
            (Term::Number(c), Term::Equation(x)) => (x, c),
            (Term::Equation(x), Term::Number(c)) => (x, c),
            _ => panic!("expected exact one Number and one Equation"),
        };
        x.solve(c)
    }

    fn parse_input(input: &str) -> (Vec<(&str, &str)>, HashMap<&str, Expression>) {
        input.lines().partition_map(|line| {
            let (name, operation) = line.split_once(": ").unwrap();
            if let Ok(num) = operation.parse() {
                let term = if name == "humn" {
                    Expression::Input
                } else {
                    Expression::Number(num)
                };
                Either::Right((name, term))
            } else {
                let (a, operator, b) = operation.split(' ').collect_tuple().unwrap();
                if name == "root" {
                    Either::Left((a, b))
                } else {
                    let operator = Operator::try_from(operator).unwrap();
                    Either::Right((name, Expression::Expressions(a, operator, b)))
                }
            }
        })
    }

    #[derive(Debug)]
    enum Expression<'a> {
        Input,
        Number(i64),
        Expressions(&'a str, Operator, &'a str),
    }

    impl<'a> Expression<'a> {
        fn evaluate(&self, monkeys: &HashMap<&'a str, Expression>) -> Term {
            match self {
                Expression::Input => Term::Equation(Equation::Input),
                Expression::Number(number) => Term::Number(*number),
                Expression::Expressions(a, operator, b) => {
                    let a = monkeys[a].evaluate(monkeys);
                    let b = monkeys[b].evaluate(monkeys);
                    match (a, b) {
                        (Term::Number(a), Term::Number(b)) => {
                            let number = operator.operate(a, b);
                            Term::Number(number)
                        }
                        (Term::Number(a), Term::Equation(b)) => {
                            let equation = Equation::Right(a, *operator, Box::new(b));
                            Term::Equation(equation)
                        }
                        (Term::Equation(a), Term::Number(b)) => {
                            let equation = Equation::Left(Box::new(a), *operator, b);
                            Term::Equation(equation)
                        }
                        (Term::Equation(a), Term::Equation(b)) => {
                            panic!("expected at most one equation {a:?} {b:?}")
                        }
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    enum Term {
        Number(i64),
        Equation(Equation),
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Equation {
        Input,
        Left(Box<Equation>, Operator, i64),
        Right(i64, Operator, Box<Equation>),
    }

    impl Equation {
        fn solve(&self, c: i64) -> i64 {
            match self {
                Equation::Input => c,
                Equation::Left(x, operator, b) => match operator {
                    Operator::Add => x.solve(c - b),
                    Operator::Sub => x.solve(c + b),
                    Operator::Div => x.solve(c * b),
                    Operator::Mul => x.solve(c / b),
                },
                Equation::Right(a, operator, x) => match operator {
                    Operator::Add => x.solve(c - a),
                    Operator::Sub => x.solve(a - c),
                    Operator::Div => x.solve(a / c),
                    Operator::Mul => x.solve(c / a),
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_21.txt");

    #[test_case(EXAMPLE, 152)]
    #[test_case(INPUT, 93813115694560)]
    fn part_1(input: &str, expected: i64) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 301)]
    #[test_case(INPUT, 3910938071092)]
    fn part_2(input: &str, expected: i64) {
        assert_eq!(super::part_2(input), expected);
    }
}
