use std::iter::once;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> isize {
    intcode(input, 1)
}

pub fn part_2(input: &str) -> isize {
    intcode(input, 2)
}

fn intcode(input: &str, init: isize) -> isize {
    let program = parse_program(input);
    once(init).intcode(program).next().unwrap()
}

#[cfg(test)]
mod test {

    use test_case::test_case;

    const EXAMPLE_1: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

    #[test]
    fn example_1() {
        use crate::intcode::{IntcodeExt as _, parse_program};
        use std::iter::once;
        let expected = parse_program(EXAMPLE_1);
        let actual: Vec<_> = once(0).intcode(expected.clone()).collect();
        assert_eq!(actual, expected);
    }

    const EXAMPLE_2: &str = "1102,34915192,34915192,7,4,7,99,0";

    const EXAMPLE_3: &str = "104,1125899906842624,99";

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    #[test_case(EXAMPLE_2 => 1219070632396864)]
    #[test_case(EXAMPLE_3 => 1125899906842624)]
    #[test_case(INPUT => 2171728567)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(INPUT => 49815)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
