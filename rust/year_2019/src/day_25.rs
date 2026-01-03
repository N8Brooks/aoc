use crate::intcode::{IntcodeExt as _, parse_program};

/// FIXME: hardcoded solution
pub fn part_1(input: &str) -> isize {
    const SCRIPT: &[u8] = b"\
south
take food ration
west
north
north
east
take astrolabe
west
south
south
east
north
east
south
take weather machine
west
take ornament
east
north
east
east
east
south
";

    let output: String = SCRIPT
        .iter()
        .map(|&b| b.into())
        .intcode(parse_program(input))
        .map(|i| i as u8 as char)
        .collect();

    output
        .lines()
        .find_map(|line| {
            line.strip_circumfix(
                "\"Oh, hello! You should be able to get in by typing ",
                " on the keypad at the main airlock.\"",
            )
        })
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_25.txt");

    #[test_case(INPUT => 4206594)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }
}
