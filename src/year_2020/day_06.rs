use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            let mut iter = group
                .split('\n')
                .map(|person| HashSet::from_iter(person.chars()));
            iter.next()
                .map(|set: HashSet<char>| iter.fold(set, |a, b| &a | &b).len())
                .unwrap() as u32
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            let mut iter = group
                .split('\n')
                .map(|person| HashSet::from_iter(person.chars()));
            iter.next()
                .map(|set: HashSet<char>| iter.fold(set, |a, b| &a & &b).len())
                .unwrap() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_06.txt").unwrap();
    }

    #[test_case(EXAMPLE, 11)]
    #[test_case(&INPUT, 6532)]
    fn part_1_examples(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 6)]
    #[test_case(&INPUT, 3427)]
    fn part_2_examples(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
