use itertools::Itertools as _;

pub fn part_1(input: &str, shift: u64) -> String {
    let program = parse_program(input);
    let mut a = parse_a(input);
    let mut outputs = Vec::new();
    while a > 0 {
        outputs.push(next_output(a, &program));
        a >>= shift;
    }
    outputs.iter().join(",")
}

pub fn part_2(input: &str) -> u64 {
    let program = parse_program(input);
    program.iter().rev().fold(0, |a, &x| {
        let a = a << 3; // assume octal
        (a..=a | 255)
            .find(|&a| next_output(a, &program) == x)
            .unwrap()
    })
}

fn parse_a(input: &str) -> u64 {
    input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_program(input: &str) -> Vec<u64> {
    input
        .lines()
        .last()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn next_output(mut a: u64, program: &[u64]) -> u64 {
    // this makes several assumptions about the program
    let mut b = 0;
    let mut c = 0;
    for (instruction, literal) in program.iter().tuples() {
        let combo = || match literal {
            0..=3 => *literal,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("unknown combo: {}", literal),
        };
        match instruction {
            0 => a /= 2u64.pow(combo() as u32),
            1 => b ^= literal,
            2 => b = combo() % 8,
            3 => panic!("hit next iteration"),
            4 => b ^= c,
            5 => return combo() % 8,
            6 => b = a / 2u64.pow(combo() as u32),
            7 => c = a / 2u64.pow(combo() as u32),
            _ => panic!("unknown instruction: {}", instruction),
        }
    }
    panic!("no output found")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_17.txt");

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test_case(EXAMPLE, 1, "4,6,3,5,6,3,5,2,1,0")]
    #[test_case(INPUT, 3, "1,2,3,1,3,2,5,3,1")]
    fn part_1(input: &str, shift: u64, expected: &str) {
        assert_eq!(super::part_1(input, shift), expected);
    }

    const EXAMPLE_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test_case(EXAMPLE_2, 117440)]
    #[test_case(INPUT, 105706277661082)]
    fn part_2(input: &str, expected: u64) {
        assert_eq!(super::part_2(input), expected);
    }
}
