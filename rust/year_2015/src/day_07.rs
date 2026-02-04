use Instruction::*;
use Operand::*;
use hashbrown::HashMap;

pub fn part_1(input: &str) -> u16 {
    let instructions = parse_instructions(input);
    evaluate_a(&instructions)
}

pub fn part_2(input: &str) -> u16 {
    let mut instructions = parse_instructions(input);
    let a = evaluate_a(&instructions);
    instructions.insert("b", Assign(Value(a)));
    evaluate_a(&instructions)
}

fn parse_instructions(input: &str) -> HashMap<&str, Instruction<'_>> {
    input
        .lines()
        .map(|line| {
            let (expr, wire) = line.split_once(" -> ").unwrap();
            let instruction = if let Some((a, b)) = expr.split_once(" AND ") {
                And(Operand::parse(a), b)
            } else if let Some((a, b)) = expr.split_once(" OR ") {
                Or(Operand::parse(a), b)
            } else if let Some((a, b)) = expr.split_once(" LSHIFT ") {
                Lshift(a, b.parse().unwrap())
            } else if let Some((a, b)) = expr.split_once(" RSHIFT ") {
                Rshift(a, b.parse().unwrap())
            } else if let Some(a) = expr.strip_prefix("NOT ") {
                Not(a)
            } else {
                Assign(Operand::parse(expr))
            };
            (wire, instruction)
        })
        .collect()
}

#[derive(Debug)]
enum Instruction<'a> {
    Assign(Operand<'a>),
    And(Operand<'a>, &'a str),
    Or(Operand<'a>, &'a str),
    Lshift(&'a str, u8),
    Rshift(&'a str, u8),
    Not(&'a str),
}

impl Instruction<'_> {
    fn inputs(&self) -> [Option<&'_ str>; 2] {
        match self {
            Assign(a) => [a.input(), None],
            Lshift(a, _) | Rshift(a, _) | Not(a) => [Some(*a), None],
            And(a, b) | Or(a, b) => [a.input(), Some(*b)],
        }
    }
}

#[derive(Debug)]
enum Operand<'a> {
    Value(u16),
    Wire(&'a str),
}

impl Operand<'_> {
    fn parse(s: &str) -> Operand<'_> {
        if let Ok(n) = s.parse() {
            Value(n)
        } else {
            debug_assert!(
                s.bytes().all(|b| b.is_ascii_lowercase()),
                "invalid wire: {s}"
            );
            Wire(s)
        }
    }

    fn input(&self) -> Option<&str> {
        match self {
            Value(_) => None,
            Wire(w) => Some(w),
        }
    }
}

fn evaluate_a(instructions: &HashMap<&str, Instruction>) -> u16 {
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();
    let mut indegree: HashMap<_, usize> = HashMap::new();
    for (wire, instr) in instructions {
        let count = indegree.entry(*wire).or_default();
        for input in instr.inputs().into_iter().flatten() {
            graph.entry(input).or_default().push(*wire);
            *count += 1;
        }
    }
    let mut stack: Vec<_> = indegree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&wire, _)| wire)
        .collect();
    let mut signals = HashMap::new();
    while let Some(wire) = stack.pop() {
        let instr = &instructions[wire];
        let signal = match instr {
            Assign(n) => match n {
                Value(n) => *n,
                Wire(w) => signals[w],
            },
            And(a, b) => {
                let a = match a {
                    Value(n) => *n,
                    Wire(w) => signals[w],
                };
                let b = signals[b];
                a & b
            }
            Or(a, b) => {
                let a = match a {
                    Value(n) => *n,
                    Wire(w) => signals[w],
                };
                let b = signals[b];
                a | b
            }
            Lshift(a, n) => signals[a] << n,
            Rshift(a, n) => signals[a] >> n,
            Not(a) => !signals[a],
        };
        signals.insert(wire, signal);
        graph
            .get(wire)
            .into_iter()
            .flatten()
            .filter(|&x| {
                let count = indegree.get_mut(x).unwrap();
                *count -= 1;
                *count == 0
            })
            .collect_into(&mut stack);
    }
    signals["a"]
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    #[test_case(INPUT => 16076)]
    fn part_1(input: &str) -> u16 {
        super::part_1(input)
    }

    #[test_case(INPUT => 2797)]
    fn part_2(input: &str) -> u16 {
        super::part_2(input)
    }
}
