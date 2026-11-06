use std::iter::repeat_with;

pub fn part_1(input: &str) -> usize {
    let (a, b) = parse_generators(input);
    a.zip(b)
        .take(40_000_000)
        .filter(|&(a, b)| a as u16 == b as u16)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let (a, b) = parse_generators(input);
    a.filter(|x| x.is_multiple_of(4))
        .zip(b.filter(|x| x.is_multiple_of(8)))
        .take(5_000_000)
        .filter(|&(a, b)| a as u16 == b as u16)
        .count()
}

fn parse_generators(input: &str) -> (impl Iterator<Item = u64>, impl Iterator<Item = u64>) {
    let (a, b) = input.trim_end().split_once('\n').unwrap();
    let a = a
        .trim_start_matches("Generator A starts with ")
        .parse()
        .unwrap();
    let b = b
        .trim_start_matches("Generator B starts with ")
        .parse()
        .unwrap();
    (generator(a, 16_807), generator(b, 48_271))
}

fn generator(mut x: u64, y: u64) -> impl Iterator<Item = u64> {
    const MOD: u64 = i32::MAX as u64;
    repeat_with(move || {
        x = (x * y) % MOD;
        x
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "Generator A starts with 65
Generator B starts with 8921";

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    #[test_case(EXAMPLE => 588)]
    #[test_case(INPUT => 626)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 309)]
    #[test_case(INPUT => 306)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
