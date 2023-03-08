pub fn part_1(input: &str) -> i64 {
    part_1::Monkeys::from(input).operate("root")
}

mod part_1 {
    use std::collections::HashMap;

    #[derive(Debug)]
    enum Operation<'a> {
        Number(i64),
        Add(&'a str, &'a str),
        Sub(&'a str, &'a str),
        Div(&'a str, &'a str),
        Mul(&'a str, &'a str),
    }

    impl<'a> From<&'a str> for Operation<'a> {
        fn from(input: &'a str) -> Self {
            if let Ok(num) = input.parse() {
                return Operation::Number(num);
            }
            let mut tokens = input.split_whitespace();
            let a = tokens.next().unwrap();
            let operator = tokens.next().unwrap();
            let b = tokens.next().unwrap();
            match operator {
                "+" => Operation::Add(a, b),
                "-" => Operation::Sub(a, b),
                "/" => Operation::Div(a, b),
                "*" => Operation::Mul(a, b),
                _ => panic!("unknown operator {operator}"),
            }
        }
    }

    #[derive(Debug)]
    pub struct Monkeys<'a>(HashMap<&'a str, Operation<'a>>);

    impl<'a> Monkeys<'a> {
        pub fn operate(&self, name: &str) -> i64 {
            let operation = &self.0[name];
            match operation {
                Operation::Number(num) => *num,
                Operation::Add(a, b) => self.operate(a) + self.operate(b),
                Operation::Sub(a, b) => self.operate(a) - self.operate(b),
                Operation::Div(a, b) => self.operate(a) / self.operate(b),
                Operation::Mul(a, b) => self.operate(a) * self.operate(b),
            }
        }
    }

    impl<'a> From<&'a str> for Monkeys<'a> {
        fn from(input: &'a str) -> Self {
            Monkeys(
                input
                    .lines()
                    .map(|line| {
                        let (name, operation) = line.split_once(": ").unwrap();
                        let operation = Operation::from(operation);
                        (name, operation)
                    })
                    .collect(),
            )
        }
    }
}

pub fn part_2(input: &str) -> i64 {
    use part_2_context::*;
    let monkeys = Monkeys::from(input);
    let equation = monkeys.operate("root");
    let (x, c) = match equation {
        Equation::Left(x, Operator::Eql, c) => (x, c),
        Equation::Right(c, Operator::Eql, x) => (x, c),
        _ => panic!("expected equality {equation:?}"),
    };
    Monkeys::un_operate(&x, c)
}

mod part_2_context {
    use std::collections::HashMap;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Operator {
        Add,
        Sub,
        Div,
        Mul,
        Eql,
    }

    #[derive(Debug)]
    enum Expression<'a> {
        Input,
        Number(i64),
        Operation(&'a str, Operator, &'a str),
    }

    impl<'a> Expression<'a> {
        fn parse(line: &'a str) -> (&'a str, Expression<'a>) {
            let (name, operation) = line.split_once(": ").unwrap();
            if let Ok(num) = operation.parse() {
                let operation = if name == "humn" {
                    Expression::Input
                } else {
                    Expression::Number(num)
                };
                (name, operation)
            } else {
                let mut tokens = operation.split_whitespace();
                let a = tokens.next().unwrap();
                let operator = tokens.next().unwrap();
                let b = tokens.next().unwrap();
                let operation = match operator {
                    _ if name == "root" => Expression::Operation(a, Operator::Eql, b),
                    "+" => Expression::Operation(a, Operator::Add, b),
                    "-" => Expression::Operation(a, Operator::Sub, b),
                    "/" => Expression::Operation(a, Operator::Div, b),
                    "*" => Expression::Operation(a, Operator::Mul, b),
                    _ => panic!("unknown operator {operator}"),
                };
                (name, operation)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Equation {
        Input,
        Number(i64),
        Left(Box<Equation>, Operator, i64),
        Right(i64, Operator, Box<Equation>),
    }

    #[derive(Debug)]
    pub struct Monkeys<'a>(HashMap<&'a str, Expression<'a>>);

    impl<'a> Monkeys<'a> {
        pub fn operate(&self, name: &str) -> Equation {
            let expression = &self.0[name];
            match expression {
                Expression::Input => Equation::Input,
                Expression::Number(num) => Equation::Number(*num),
                Expression::Operation(a, op, b) => self.sub_operate(a, *op, b),
            }
        }

        fn sub_operate(&self, a: &'a str, operator: Operator, b: &'a str) -> Equation {
            match (self.operate(a), self.operate(b)) {
                (
                    a @ Equation::Left(..) | a @ Equation::Right(..) | a @ Equation::Input,
                    Equation::Number(b),
                ) => Equation::Left(Box::new(a), operator, b),
                (
                    Equation::Number(a),
                    b @ Equation::Left(..) | b @ Equation::Right(..) | b @ Equation::Input,
                ) => Equation::Right(a, operator, Box::new(b)),
                (Equation::Number(a), Equation::Number(b)) => {
                    let number = match operator {
                        Operator::Add => a + b,
                        Operator::Sub => a - b,
                        Operator::Div => a / b,
                        Operator::Mul => a * b,
                        Operator::Eql => panic!("unexpected {a:?} {operator:?} {b:?}"),
                    };
                    Equation::Number(number)
                }
                (a, b) => panic!("{a:?} {operator:?} {b:?}"),
            }
        }

        pub fn un_operate(equation: &Equation, c: i64) -> i64 {
            match equation {
                Equation::Input => c,
                Equation::Left(x, operator, b) => match operator {
                    Operator::Add => Monkeys::un_operate(x, c - b),
                    Operator::Sub => Monkeys::un_operate(x, c + b),
                    Operator::Div => Monkeys::un_operate(x, c * b),
                    Operator::Mul => Monkeys::un_operate(x, c / b),
                    Operator::Eql => panic!("unexpected {x:?} {operator:?} {b:?}"),
                },
                Equation::Right(a, operator, x) => match operator {
                    Operator::Add => Monkeys::un_operate(x, c - a),
                    Operator::Sub => Monkeys::un_operate(x, a - c),
                    Operator::Div => Monkeys::un_operate(x, a / c),
                    Operator::Mul => Monkeys::un_operate(x, c / a),
                    Operator::Eql => panic!("unexpected {a:?} {operator:?} {x:?}"),
                },
                Equation::Number(num) => panic!("invariant number {num}"),
            }
        }
    }

    impl<'a> From<&'a str> for Monkeys<'a> {
        fn from(input: &'a str) -> Self {
            Monkeys(input.lines().map(Expression::parse).collect())
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
