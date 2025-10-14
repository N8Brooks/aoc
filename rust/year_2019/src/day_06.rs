use hashbrown::HashMap;
use std::iter::{self, successors};

pub fn part_1(input: &str) -> usize {
    let children: HashMap<_, Vec<_>> = input.lines().fold(HashMap::new(), |mut orbits, line| {
        let (a, b) = line.split_once(')').unwrap();
        orbits.entry(a).or_default().push(b);
        orbits
    });

    let mut stack = vec![("COM", 0)];
    iter::from_fn(move || {
        stack.pop().map(|(node, depth)| {
            if let Some(children) = children.get(node) {
                for &child in children {
                    stack.push((child, depth + 1));
                }
            }
            depth
        })
    })
    .sum()
}

pub fn part_2(input: &str) -> usize {
    let parent: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(')').unwrap();
            (b, a)
        })
        .collect();

    let ancestors = |start: &str| successors(parent.get(start), |&n| parent.get(n));

    let path_1: HashMap<_, _> = ancestors("YOU")
        .enumerate()
        .map(|(depth, ancestor)| (ancestor, depth))
        .collect();

    ancestors("SAN")
        .enumerate()
        .find_map(|(depth_san, ancestor)| {
            path_1.get(ancestor).map(|&depth_you| depth_you + depth_san)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    #[test_case(EXAMPLE_1, 42)]
    #[test_case(INPUT, 186597)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_2: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test_case(EXAMPLE_2, 4)]
    #[test_case(INPUT, 412)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
