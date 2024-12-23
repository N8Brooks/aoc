use hashbrown::{HashMap, HashSet};
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (u, v) = line.split_once('-').unwrap();
        network.entry(u).or_default().insert(v);
        network.entry(v).or_default().insert(u);
    }
    network
        .iter()
        .filter(|(u, _)| u.starts_with('t'))
        .flat_map(|(&u, vs)| {
            vs.iter()
                .tuple_combinations()
                .filter(|&(v, w)| network[v].contains(w))
                .map(move |(v, w)| {
                    let mut set = [u, v, w];
                    set.sort_unstable();
                    set
                })
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2(input: &str) -> String {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (u, v) = line.split_once('-').unwrap();
        network.entry(u).or_default().insert(v);
        network.entry(v).or_default().insert(u);
    }
    // Assume lan size is the size of connections minus one
    let n = network.iter().next().unwrap().1.len();
    let mut nw = network
        .iter()
        .map(|(_, vs)| {
            vs.iter()
                .flat_map(|v| network[v].iter())
                .chain(vs.iter())
                .counts()
                .into_iter()
                .filter(|&(_, count)| count >= n - 1)
                .map(|(v, _)| *v)
                .collect_vec()
        })
        .max_by_key(|x| x.len())
        .unwrap();
    nw.sort_unstable();
    nw.iter().join(",")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_23.txt");

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test_case(EXAMPLE, 7)]
    #[test_case(INPUT, 1366)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, "co,de,ka,ta")]
    #[test_case(INPUT, "bs,cf,cn,gb,gk,jf,mp,qk,qo,st,ti,uc,xw")]
    fn part_2(input: &str, expected: &str) {
        assert_eq!(super::part_2(input), expected);
    }
}
