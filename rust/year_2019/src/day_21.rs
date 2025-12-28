use crate::intcode::{Intcode, parse_program};

pub fn part_1(input: &str) -> usize {
    const CODE: &[u8] = b"NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
WALK
";
    hull_damage(input, CODE)
}

pub fn part_2(input: &str) -> usize {
    const CODE: &[u8] = b"NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN
";
    hull_damage(input, CODE)
}

fn hull_damage(input: &str, code: &[u8]) -> usize {
    let program = parse_program(input);
    let inputs = code.iter().map(|&b| b.into());
    Intcode::new(program, inputs)
        .last()
        .unwrap()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_21.txt");

    #[test_case(INPUT => 19360724)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 1140450681)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
