use hashbrown::{DefaultHashBuilder, HashMap};
use itertools::Itertools as _;

type State = (usize, u64, bool);

pub fn part_1(input: &str) -> usize {
    solve(input, false)
}

pub fn part_2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, allow_twice: bool) -> usize {
    let edges = input.lines().map(|line| line.split_once('-').unwrap());

    let ids: HashMap<_, _> = edges
        .clone()
        .flat_map(|(a, b)| [a, b])
        .unique_with_hasher(DefaultHashBuilder::default())
        .enumerate()
        .map(|(id, name)| (name, id))
        .collect();

    assert!(ids.len() <= u64::BITS as usize);

    let start = ids["start"];
    let end = ids["end"];

    let small = ids
        .iter()
        .filter(|(name, _)| name.as_bytes()[0].is_ascii_lowercase())
        .fold(0u64, |mask, (_, &id)| mask | (1 << id));

    let mut graph = vec![Vec::new(); ids.len()];

    edges
        .flat_map(|(a, b)| {
            let (a, b) = (ids[a], ids[b]);
            [(a, b), (b, a)]
        })
        .filter(|&(a, b)| a != end && b != start)
        .for_each(|(a, b)| graph[a].push(b));

    count_paths(
        &graph,
        small,
        end,
        (start, 0, allow_twice),
        &mut HashMap::new(),
    )
}

fn count_paths(
    graph: &[Vec<usize>],
    small: u64,
    end: usize,
    state @ (u, visited, allow_twice): State,
    cache: &mut HashMap<State, usize>,
) -> usize {
    if u == end {
        return 1;
    }

    if let Some(&count) = cache.get(&state) {
        return count;
    }

    let count = graph[u]
        .iter()
        .filter_map(|&v| {
            let bit = (1u64 << v) & small;
            let revisited = visited & bit != 0;

            (!revisited || allow_twice).then_some((v, visited | bit, allow_twice && !revisited))
        })
        .map(|next| count_paths(graph, small, end, next, cache))
        .sum();

    cache.insert(state, count);
    count
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    const EXAMPLE_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EXAMPLE_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EXAMPLE_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test_case(EXAMPLE_1 => 10)]
    #[test_case(EXAMPLE_2 => 19)]
    #[test_case(EXAMPLE_3 => 226)]
    #[test_case(INPUT => 3495)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 36)]
    #[test_case(EXAMPLE_2 => 103)]
    #[test_case(EXAMPLE_3 => 3509)]
    #[test_case(INPUT => 94849)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
