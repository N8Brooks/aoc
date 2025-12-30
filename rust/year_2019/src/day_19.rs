use num::{One as _, Zero as _};

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> usize {
    let program = parse_program(input);
    (0..50)
        .flat_map(|y| (0..50).map(move |x| [x, y]))
        .map(|inputs| inputs.into_iter().intcode(program.clone()).next().unwrap())
        .filter(|&output| output == 1)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let program = parse_program(input);
    let mut y1 = 0;
    (99..)
        .find_map(|x2| {
            while [x2, y1]
                .into_iter()
                .intcode(program.clone())
                .next()
                .unwrap()
                .is_zero()
            {
                y1 += 1;
            }

            let x1 = x2 - 99;
            let y2 = y1 + 99;
            [x1, y2]
                .into_iter()
                .intcode(program.clone())
                .next()
                .unwrap()
                .is_one()
                .then(|| x1 * 10_000 + y1)
        })
        .unwrap()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_19.txt");

    #[test_case(INPUT => 226)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 7900946)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
