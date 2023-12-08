use std::collections::HashMap;

use num::Integer;

pub fn part_1(input: &str) -> usize {
    Network::parse(input, |node| node != "ZZZ").steps("AAA")
}

pub fn part_2(input: &str) -> usize {
    let network = Network::parse(input, |node| !node.ends_with('Z'));
    network
        .nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| network.steps(node))
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}

struct Network<'a> {
    nodes: HashMap<&'a str, [&'a str; 2]>,
    instructions: Vec<usize>,
    predicate: fn(&str) -> bool,
}

impl<'a> Network<'a> {
    fn parse(input: &'a str, predicate: fn(&str) -> bool) -> Network<'a> {
        let (instructions, network_input) = input.split_once("\n\n").unwrap();

        let instructions = instructions
            .bytes()
            .map(|byte| match byte {
                b'L' => 0,
                b'R' => 1,
                _ => panic!("Invalid instruction"),
            })
            .collect::<Vec<_>>();

        let network = HashMap::from_iter(network_input.lines().map(|line| {
            let (name, connections) = line.split_once(" = ").unwrap();
            let (left, right) = connections
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();

            (name, [left, right])
        }));

        Network {
            nodes: network,
            instructions,
            predicate,
        }
    }

    fn steps(&self, node: &str) -> usize {
        self.instructions
            .iter()
            .cycle()
            .scan(node, |node, &instruction| {
                let connection = self.nodes[node][instruction];
                *node = connection;
                Some(*node)
            })
            .take_while(|&node| (self.predicate)(node))
            .count()
            + 1
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case(EXAMPLE_1, 2)]
    #[test_case(EXAMPLE_2, 6)]
    #[test_case(INPUT, 20221)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test_case(EXAMPLE_3, 6)]
    #[test_case(INPUT, 14616363770447)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }

    const INPUT: &str = include_str!("../../../testdata/year_2023/day_08.txt");

    const EXAMPLE_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
}
