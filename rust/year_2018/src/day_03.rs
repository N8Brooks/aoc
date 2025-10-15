use hashbrown::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl From<&str> for Claim {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }
        let caps = RE.captures(line).expect("line matches claim regex");
        Claim {
            id: caps[1].parse().unwrap(),
            left: caps[2].parse().unwrap(),
            top: caps[3].parse().unwrap(),
            width: caps[4].parse().unwrap(),
            height: caps[5].parse().unwrap(),
        }
    }
}

impl Claim {
    fn iter_locations(&self) -> impl Iterator<Item = (usize, usize)> {
        (self.left..self.left + self.width).cartesian_product(self.top..self.top + self.height)
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(Claim::from)
        .flat_map(|claim| claim.iter_locations())
        .duplicates()
        .unique()
        .count()
}

pub fn part_2(input: &str) -> usize {
    let claims = input.lines().map(Claim::from).collect_vec();
    let non_intact_locations: HashSet<_> = claims
        .iter()
        .flat_map(|claim| claim.iter_locations())
        .duplicates()
        .collect();
    claims
        .iter()
        .filter(|claim| {
            claim
                .iter_locations()
                .all(|location| !non_intact_locations.contains(&location))
        })
        .exactly_one()
        .expect("one instact claim")
        .id
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    const INPUT: &str = include_str!("../../../test_data/year_2018/day_03.txt");

    #[test_case(EXAMPLE, 4; "example")]
    #[test_case(INPUT, 121163; "input")]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 3; "example")]
    #[test_case(INPUT, 943; "input")]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
