use lazy_static::lazy_static;
use regex::Regex;

struct CorpPolicy {
    lo: usize,
    hi: usize,
    letter: char,
    password: String,
}

fn iter_input(input: &str) -> impl Iterator<Item = CorpPolicy> + '_ {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }

    RE.captures_iter(input).map(|groups| CorpPolicy {
        lo: groups[1].parse().unwrap(),
        hi: groups[2].parse().unwrap(),
        letter: groups[3].chars().next().unwrap(),
        password: groups[4].to_string(),
    })
}

pub fn part_1(input: &str) -> usize {
    iter_input(input)
        .filter(|policy| {
            let range = policy.lo..=policy.hi;
            let count = policy.password.matches(policy.letter).count();
            range.contains(&&count)
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    iter_input(input)
        .filter(|policy| {
            let a = policy.password.chars().nth(policy.lo - 1).unwrap();
            let b = policy.password.chars().nth(policy.hi - 1).unwrap();
            (a == policy.letter) ^ (b == policy.letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;

    static EXAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_02.txt").unwrap();
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE), 2);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(&INPUT), 591);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE), 1);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(part_2(&INPUT), 335);
    }
}
