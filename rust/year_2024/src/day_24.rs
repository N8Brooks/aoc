use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (values, gates) = input.split_once("\n\n").unwrap();
    let mut values: HashMap<&str, bool> = values
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").unwrap();
            let value = match value {
                "1" => true,
                "0" => false,
                _ => panic!("Unexpected value: {value}"),
            };
            (wire, value)
        })
        .collect();
    let mut gates: VecDeque<(&str, &str, &str, &str)> = gates
        .lines()
        .map(|line| {
            let (lhs, out) = line.split_once(" -> ").unwrap();
            let (in_a, gate, in_b) = lhs.split(" ").collect_tuple().unwrap();
            (in_a, gate, in_b, out)
        })
        .collect();
    while let Some((in_a, gate, in_b, out)) = gates.pop_front() {
        if let (Some(in_a), Some(in_b)) = (values.get(in_a), values.get(in_b)) {
            let value = match gate {
                "AND" => in_a & in_b,
                "OR" => in_a | in_b,
                "XOR" => in_a ^ in_b,
                _ => panic!("Unexpected gate: {gate}"),
            };
            values.insert(out, value);
        } else {
            gates.push_back((in_a, gate, in_b, out));
        }
    }
    values
        .into_iter()
        .filter(|(wire, _)| wire.starts_with('z'))
        .sorted_unstable_by_key(|(wire, _)| *wire)
        .rev()
        .fold(0, |acc, (_, value)| (acc << 1) | value as usize)
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Wire<'a> {
    X(usize),
    Y(usize),
    Z(usize),
    Other(&'a str),
}

impl<'a> Wire<'a> {
    fn parse(s: &'a str) -> Self {
        s[1..].parse().map_or_else(
            |_| Wire::Other(s),
            |i| match s.chars().next().unwrap() {
                'x' => Wire::X(i),
                'y' => Wire::Y(i),
                'z' => Wire::Z(i),
                _ => unreachable!(),
            },
        )
    }
}

impl Display for Wire<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Wire::X(i) => write!(f, "x{i:02}"),
            Wire::Y(i) => write!(f, "y{i:02}"),
            Wire::Z(i) => write!(f, "z{i:02}"),
            Wire::Other(s) => write!(f, "{s}"),
        }
    }
}

pub fn part_2(input: &str) -> String {
    let (_, gates) = input.split_once("\n\n").unwrap();
    let gates = gates
        .lines()
        .map(|line| {
            let (lhs, out) = line.split_once(" -> ").unwrap();
            let (in_a, gate, in_b) = lhs.split(" ").collect_tuple().unwrap();
            (Wire::parse(in_a), gate, Wire::parse(in_b), Wire::parse(out))
        })
        .collect_vec();

    let z_not_xor = gates
        .iter()
        .filter(|(_, gate, _, out)| matches!(*out, Wire::Z(0..45)) && gate != &"XOR")
        .map(|(_, _, _, out)| out);

    let invalid_xor = gates
        .iter()
        .filter(|(_, gate, _, _)| *gate == "XOR")
        .filter_map(|(in_a, _, in_b, out)| match (in_a, in_b, out) {
            (Wire::X(0), Wire::Y(0), Wire::Z(0)) => None,
            (Wire::X(_) | Wire::Y(_), Wire::X(_) | Wire::Y(_), Wire::Other(_)) => None,
            (Wire::Other(_), Wire::Other(_), Wire::Z(_)) => None,
            _ => Some(out),
        });

    let and_outputs: HashSet<_> = gates
        .iter()
        .filter(|&(in_a, gate, in_b, _)| {
            gate == &"AND" && in_a != &Wire::X(0) && in_b != &Wire::X(0)
        })
        .map(|(_, _, _, out)| out)
        .collect();

    let or_inputs: HashSet<_> = gates
        .iter()
        .filter(|(_, gate, _, _)| *gate == "OR")
        .flat_map(|(in_a, _, in_b, _)| [in_a, in_b])
        .collect();

    let invalid_and_or = and_outputs.symmetric_difference(&or_inputs).copied();

    z_not_xor
        .chain(invalid_xor)
        .chain(invalid_and_or)
        .unique()
        .map(|x| x.to_string())
        .sorted_unstable()
        .join(",")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    const INPUT: &str = include_str!("../test_data/day_24.txt");

    #[test_case(EXAMPLE_1, 4)]
    #[test_case(EXAMPLE_2, 2024)]
    #[test_case(INPUT, 41324968993486)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, "bmn,jss,mvb,rds,wss,z08,z18,z23")]
    fn part_2(input: &str, expected: &str) {
        assert_eq!(super::part_2(input), expected);
    }
}
