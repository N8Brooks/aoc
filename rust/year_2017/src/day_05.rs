pub fn part_1(input: &str) -> usize {
    let mut jumps = parse_jumps(input);
    let mut pos = 0;
    let mut steps = 0;
    while let Some(jump) = jumps.get_mut(pos) {
        pos = pos.strict_add_signed(*jump);
        *jump += 1;
        steps += 1;
    }
    steps
}

pub fn part_2(input: &str) -> usize {
    let mut jumps = parse_jumps(input);
    let mut pos = 0;
    let mut steps = 0;
    while let Some(jump) = jumps.get_mut(pos) {
        pos = pos.strict_add_signed(*jump);
        *jump += if *jump >= 3 { -1 } else { 1 };
        steps += 1;
    }
    steps
}

fn parse_jumps(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    const EXAMPLE: &str = "0
3
0
1
-3";

    #[test_case(EXAMPLE => 5)]
    #[test_case(INPUT => 325922)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 10)]
    #[test_case(INPUT => 24490906)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
