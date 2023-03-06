use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Valve (?P<valve>[A-Z]+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<leads>[A-Z]+(?:, [A-Z]+)*)$").unwrap();
}

struct VolcanoState<'a> {
    memo: HashMap<(usize, usize, usize), usize>,
    valves: HashMap<&'a str, usize>,
    nodes: HashMap<usize, VolcanoNode>,
}

impl<'a> VolcanoState<'a> {
    fn from_input(input: &str) -> VolcanoState {
        let mut valves = HashMap::new();
        let nodes = HashMap::from_iter(
            input
                .lines()
                .map(|line| VolcanoNode::parse_line(line, &mut valves)),
        );
        VolcanoState {
            memo: HashMap::new(),
            // memo: vec![],
            valves,
            nodes,
        }
    }

    fn get_starting_index(&self) -> usize {
        *self.valves.get("AA").expect("starting index")
    }

    fn max_pressure(&mut self, t: usize, index: usize, open_set: usize) -> usize {
        if t == 30 {
            return 0;
        }

        if let Some(&p) = self.memo.get(&(t, open_set, index)) {
            return p;
        }

        let pressure = usize::max(
            self.open_valve(t, index, open_set),
            self.visit_neighbor(t, index, open_set),
        );

        self.memo.insert((t, open_set, index), pressure);

        pressure
    }

    fn open_valve(&mut self, t: usize, index: usize, open_set: usize) -> usize {
        let node = &self.nodes[&index];
        if node.flow_rate > 0 && open_set & (1 << index) == 0 {
            let open_set = open_set | (1 << index);
            let pressure = (30 - t - 1) * node.flow_rate;
            self.max_pressure(t + 1, index, open_set) + pressure
        } else {
            0
        }
    }

    fn visit_neighbor(&mut self, t: usize, index: usize, open_set: usize) -> usize {
        self.nodes[&index]
            .leads
            .clone()
            .iter()
            .map(|index| self.max_pressure(t + 1, *index, open_set))
            .max()
            .unwrap()
    }
}

#[derive(Debug)]
struct VolcanoNode {
    leads: Vec<usize>,
    flow_rate: usize,
}

impl VolcanoNode {
    fn parse_line<'a>(line: &'a str, valves: &mut HashMap<&'a str, usize>) -> (usize, VolcanoNode) {
        let cap = RE.captures(line).unwrap();
        let valve = &line[cap.name("valve").unwrap().range()];
        let new_index = valves.len();
        let index = *valves.entry(valve).or_insert(new_index);
        let node = VolcanoNode {
            flow_rate: cap["flow_rate"].parse().unwrap(),
            leads: line[cap.name("leads").unwrap().range()]
                .split(", ")
                .map(|valve| {
                    let new_index = valves.len();
                    *valves.entry(valve).or_insert(new_index)
                })
                .collect(),
        };
        (index, node)
    }
}

pub fn part_1(input: &str) -> usize {
    let mut state = VolcanoState::from_input(input);
    let index = state.get_starting_index();
    state.max_pressure(0, index, 0)
}

// pub fn part_2(input: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use test_case::test_case;

    const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_16.txt");

    #[test]
    fn volcano_node_from_line_singular() {
        let line = EXAMPLE.lines().last().unwrap();
        let mut valves = HashMap::from([("--", 0)]);
        let expected_index = valves.len();
        let (actual_index, node) = super::VolcanoNode::parse_line(line, &mut valves);
        assert_eq!(actual_index, expected_index);
        assert_eq!(
            valves,
            HashMap::from([("--", 0), ("JJ", actual_index), ("II", 2),])
        );
        assert_eq!(node.leads, &[valves["II"]]);
        assert_eq!(node.flow_rate, 21);
    }

    #[test]
    fn volcano_node_from_line_plural() {
        let line = EXAMPLE.lines().next().unwrap();
        let mut valves = HashMap::new();
        let expected_index = valves.len();
        let (actual_index, node) = super::VolcanoNode::parse_line(line, &mut valves);
        assert_eq!(actual_index, expected_index);
        assert_eq!(
            valves,
            HashMap::from([("AA", expected_index), ("DD", 1), ("II", 2), ("BB", 3),])
        );
        assert_eq!(node.leads, &[valves["DD"], valves["II"], valves["BB"]]);
        assert_eq!(node.flow_rate, 0);
    }

    #[test_case(EXAMPLE, 1651)]
    #[test_case(INPUT, 1376)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    // #[test_case(EXAMPLE, 0)]
    // #[test_case(INPUT, 0)]
    // fn part_2(input: &str, expected: usize) {
    //     assert_eq!(super::part_2(input), expected);
    // }
}
