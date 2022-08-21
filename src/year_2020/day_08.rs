#[derive(Copy, Clone)]
enum Operation {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

fn parse_input(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| -> (&str, i32) {
            let (op, val) = line.split_once(' ').unwrap();
            (op, val.parse().unwrap())
        })
        .map(|(op, val)| {
            match op {
                "acc" => Operation::Acc(val),
                "jmp" => Operation::Jmp(val),
                "nop" => Operation::Nop(val),
                _ => panic!("Unknown operation")
            }
        })
        .collect()
}

fn run_program(instructions: &Vec<Operation>) -> (usize, i32) {
    let mut unvisited = vec![true; instructions.len()];
    let (mut i, mut acc) = (0, 0);
    while i < instructions.len() && unvisited[i] {
        unvisited[i] = false;
        (i, acc) = match instructions[i] {
            Operation::Jmp(val) => ((i as i32 + val) as usize, acc),
            Operation::Acc(val) => (i + 1, acc + val),
            Operation::Nop(_) => (i + 1, acc),
        };
    }
    (i, acc)
}

pub fn part_1(input: &str) -> i32 {
    let instructions = parse_input(input);
    run_program(&instructions).1
}

pub fn part_2<'a>(input: &'a str) -> i32 {
    let mut instructions = parse_input(input);
    for i in 0..instructions.len() {
        let corrupted_instruction = instructions[i];
        instructions[i] = match corrupted_instruction {
            Operation::Jmp(val) => Operation::Nop(val),
            Operation::Acc(_) => continue,
            Operation::Nop(val) => Operation::Jmp(val),
        };
        let (j, acc) = run_program(&instructions);
        if j == instructions.len() {
            return acc;
        }
        instructions[i] = corrupted_instruction;
    }
    panic!("Corrupted instruction not found");
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_08.txt").unwrap();
    }

    #[test_case(EXAMPLE, 5)]
    #[test_case(&INPUT, 1766)]
    fn part_1_examples(input: &str, expected: i32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 8)]
    #[test_case(&INPUT, 1639)]
    fn part_2_examples(input: &str, expected: i32) {
        assert_eq!(part_2(input), expected);
    }
}
