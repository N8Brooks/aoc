use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;

pub fn part_1(input: &str) -> isize {
    let mut ones: HashSet<isize> = HashSet::new();
    for line in input.lines() {
        let a = line.parse().unwrap();
        let b = 2020 - a;
        if ones.contains(&b) {
            return a * b;
        }
        ones.insert(a);
    }
    panic!("no two numbers sum to 2020")
}

pub fn part_2(input: &str) -> isize {
    let mut ones: HashSet<isize> = HashSet::new();
    let mut twos: HashMap<_, _> = HashMap::new();
    for c in input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted_unstable()
    {
        let complement = 2020 - c;
        if let Some((a, b)) = twos.get(&complement) {
            return a * b * c;
        }
        for &a in ones.iter() {
            let complement = a + c;
            twos.insert(complement, (a, c));
        }
        ones.insert(c);
    }
    panic!("no three numbers sum to 2020")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_01.txt");

    const EXAMPLE: &str = "1721
979
366
299
675
1456";

    #[test_case(EXAMPLE => 514579)]
    #[test_case(INPUT => 482811)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 241861950)]
    #[test_case(INPUT => 193171814)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
