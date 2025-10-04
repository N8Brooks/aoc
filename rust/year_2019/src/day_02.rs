use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let mut program = parse_program(input);
    compute(&mut program, 12, 2)
}

pub fn part_2(input: &str) -> usize {
    const TARGET: usize = 19690720;
    let program = parse_program(input);
    let mut tmp = vec![0; program.len()];
    let (noun, verb) = (0..=99)
        .cartesian_product(0..=99)
        .find(|&(noun, verb)| {
            tmp.copy_from_slice(&program);
            compute(&mut tmp, noun, verb) == TARGET
        })
        .unwrap();
    100 * noun + verb
}

fn parse_program(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn compute(program: &mut [usize], noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;
    intcode(program);
    program[0]
}

fn intcode(program: &mut [usize]) {
    let mut i = 0;
    loop {
        let mut next = || {
            let num = program[i];
            i += 1;
            num
        };
        let opcode = next();
        match opcode {
            1 | 2 => {
                let (l, r) = (program[next()], program[next()]);
                let i = next();
                program[i] = if opcode == 1 { l + r } else { l * r };
            }
            99 => return,
            _ => panic!("unexpected opcode"),
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test_case(EXAMPLE, &[3500,9,10,70,2,3,11,0,99,30,40,50])]
    #[test_case("1,0,0,0,99", &[2,0,0,0,99])]
    #[test_case("2,3,0,3,99", &[2,3,0,6,99])]
    #[test_case("2,4,4,5,99,0", &[2,4,4,5,99,9801])]
    #[test_case("1,1,1,4,99,5,6,0,99", &[30,1,1,4,2,5,6,0,99])]
    fn intcode(input: &str, expected: &[usize]) {
        let mut program = super::parse_program(input);
        super::intcode(&mut program);
        assert_eq!(&program, expected);
    }

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    #[test_case(INPUT, 3760627)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, 7195)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
