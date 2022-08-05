use std::collections::HashMap;

pub fn part_1(input: &str) -> i64 {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut mem: HashMap<u32, i64> = HashMap::new();
    input
        .lines()
        .filter_map(|line| {
            let (address, value) = line.split_once(" = ").unwrap();
            if address == "mask" {
                and_mask = i64::from_str_radix(&value.replace('X', "1"), 2).unwrap();
                or_mask = i64::from_str_radix(&value.replace('X', "0"), 2).unwrap();
                None
            } else {
                let address = address[4..address.len() - 1].parse().unwrap();
                let value = value.parse::<i64>().unwrap() & and_mask | or_mask;
                Some(value - mem.insert(address, value).unwrap_or(0))
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> i64 {
    let mut mem: HashMap<i64, i64> = HashMap::new();
    let mut masks = vec![];
    for line in input.lines() {
        let (address, value) = line.split_once(" = ").unwrap();
        if address == "mask" {
            masks = vec![(
                0xfffffffff,
                i64::from_str_radix(&value.replace('X', "0"), 2).unwrap(),
            )];
            value
                .bytes()
                .rev()
                .enumerate()
                .filter(|(_, bit)| *bit == b'X')
                .for_each(|(i, _)| {
                    let i = 1 << i as i64;
                    let zeros = masks
                        .iter()
                        .map(|(and_mask, or_mask)| (and_mask ^ i, *or_mask));
                    let ones = masks
                        .iter()
                        .map(|(and_mask, or_mask)| (*and_mask, or_mask | i));
                    masks = Vec::from_iter(zeros.chain(ones));
                });
        } else {
            let address: i64 = address[4..address.len() - 1].parse().unwrap();
            let value = value.parse().unwrap();
            masks
                .iter()
                .map(|(and_mask, or_mask)| address & and_mask | or_mask)
                .for_each(|address| {
                    mem.insert(address, value);
                })
        }
    }
    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static EXAMPLE_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_14.txt").unwrap();
    }

    #[test_case(EXAMPLE_1, 165)]
    #[test_case(&INPUT, 14722016054794)]
    fn part_1_tests(input: &str, expected: i64) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE_2, 208)]
    #[test_case(&INPUT, 3618217244644)]
    fn part_2_tests(input: &str, expected: i64) {
        assert_eq!(part_2(input), expected);
    }
}
