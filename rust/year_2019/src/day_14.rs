use std::{hint, iter::once};

use hashbrown::HashMap;

pub fn part_1(input: &str) -> usize {
    let recipes = parse_recipes(input);
    ore_required(&recipes, 1)
}

pub fn part_2(input: &str) -> usize {
    const ORE_AVAILABLE: usize = 1_000_000_000_000;
    let recipes = parse_recipes(input);
    let mut size = ORE_AVAILABLE; // assume up to a 1:1 ratio
    let mut base = 0;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let ore = ore_required(&recipes, mid);
        base = hint::select_unpredictable(ore > 1_000_000_000_000, base, mid);
        size -= half;
    }
    base
}

fn parse_recipes(input: &str) -> Vec<(Vec<(usize, usize)>, usize)> {
    let names: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, output) = line.split_once(" => ").unwrap();
            let (_, out_name) = output.split_once(' ').unwrap();
            out_name
        })
        .chain(once("ORE"))
        .collect();
    let get_id = |name: &str| names.iter().position(|&n| n == name).unwrap();

    let mut counts = vec![0; names.len()];
    let recipes: Vec<_> = input
        .lines()
        .map(|line| {
            let (inputs, output) = line.split_once(" => ").unwrap();
            let inputs: Vec<_> = inputs
                .split(", ")
                .map(|s| {
                    let (in_qty, in_name) = s.split_once(' ').unwrap();
                    let in_id = get_id(in_name);
                    let in_qty = in_qty.parse().unwrap();
                    counts[in_id] += 1;
                    (in_id, in_qty)
                })
                .collect();
            let (out_qty, _) = output.split_once(' ').unwrap();
            let out_qty: usize = out_qty.parse().unwrap();
            (inputs, out_qty)
        })
        .collect();

    let mut stack = vec![get_id("FUEL")];

    let mut ordered = Vec::with_capacity(recipes.len());
    while let Some(out_id) = stack.pop() {
        ordered.push(out_id);
        if let Some((inputs, _)) = recipes.get(out_id) {
            for &(in_id, _) in inputs {
                counts[in_id] -= 1;
                if counts[in_id] == 0 {
                    stack.push(in_id);
                }
            }
        }
    }

    let ids: HashMap<_, _> = ordered
        .iter()
        .enumerate()
        .map(|(i, &name)| (name, i))
        .collect();

    assert_eq!(ordered.pop(), Some(names.len() - 1), "ORE should be last");
    ordered
        .into_iter()
        .map(|name| {
            let (inputs, out_qty) = &recipes[name];
            let inputs = inputs
                .iter()
                .map(|(in_name, in_qty)| (ids[in_name], *in_qty))
                .collect();
            (inputs, *out_qty)
        })
        .collect()
}

fn ore_required(recipes: &[(Vec<(usize, usize)>, usize)], amount: usize) -> usize {
    const FUEL_INDEX: usize = 0;
    let ore_index = recipes.len(); // sentinel for ORE
    let mut required = vec![0; ore_index + 1];
    required[FUEL_INDEX] = amount;

    for (i, (inputs, out_qty)) in recipes.iter().enumerate() {
        let need = required[i];
        if need == 0 {
            continue;
        }
        let batches = need.div_ceil(*out_qty);
        for &(name, in_qty) in inputs {
            required[name] += in_qty * batches;
        }
    }

    required[ore_index]
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    const EXAMPLE_1: &str = "\
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    const EXAMPLE_2: &str = "\
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const EXAMPLE_3: &str = "\
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const EXAMPLE_4: &str = "\
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const EXAMPLE_5: &str = "\
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test_case(EXAMPLE_1, 31)]
    #[test_case(EXAMPLE_2, 165)]
    #[test_case(EXAMPLE_3, 13312)]
    #[test_case(EXAMPLE_4, 180697)]
    #[test_case(EXAMPLE_5, 2210736)]
    #[test_case(INPUT, 97422)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE_3, 82892753)]
    #[test_case(EXAMPLE_4, 5586022)]
    #[test_case(EXAMPLE_5, 460664)]
    #[test_case(INPUT, 13108426)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
