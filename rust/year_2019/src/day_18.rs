use std::{cmp::Reverse, collections::BinaryHeap, iter::successors};

use hashbrown::{HashMap, HashSet};

pub fn part_1(input: &str) -> usize {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let (i, j) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &b)| (b == b'@').then_some((i, j)))
        })
        .unwrap();
    let num_keys = map
        .iter()
        .copied()
        .flatten()
        .filter(|&&b| b.is_ascii_lowercase())
        .count();
    let all_keys: usize = (1 << num_keys) - 1;
    let mut seen = HashSet::new();

    successors(Some(vec![(i, j, 0_usize)]), |q| {
        let q: Vec<_> = q
            .iter()
            .flat_map(|&(i, j, keys)| {
                [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
                    .into_iter()
                    .map(move |(i, j)| (i, j, keys))
                    .filter_map(|(i, j, keys)| match map[i][j] {
                        b'#' => None,
                        b'.' | b'@' => Some((i, j, keys)),
                        b @ b'a'..=b'z' => {
                            let key_bit: usize = 1 << (b - b'a');
                            Some((i, j, keys | key_bit))
                        }
                        b @ b'A'..=b'Z' => {
                            let key_bit: usize = 1 << (b - b'A');
                            (keys & key_bit > 0).then_some((i, j, keys))
                        }
                        _ => None,
                    })
            })
            .filter(|&state| seen.insert(state))
            .collect();
        (!q.is_empty()).then_some(q)
    })
    .enumerate()
    .find_map(|(steps, q)| {
        q.into_iter()
            .find_map(|(_, _, keys)| (keys == all_keys).then_some(steps))
    })
    .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let map = {
        let mut map: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        let (i, j) = map
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, &b)| (b == b'@').then_some((i, j)))
            })
            .unwrap();
        [b"1#2", b"###", b"3#4"]
            .into_iter()
            .enumerate()
            .for_each(|(di, row)| {
                map[i + di - 1][j - 1..=j + 1].copy_from_slice(row);
            });
        map
    };

    let graph: HashMap<_, _> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &b)| b.is_ascii_alphanumeric().then_some((b, (i, j))))
        })
        .map(|(b0, (i, j))| {
            let mut stack = vec![(0_usize, (i, j), (i, j))];
            let mut results = Vec::new();
            while let Some((dist, u, v @ (i, j))) = stack.pop() {
                for w in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                    if u == w {
                        continue;
                    }
                    let dist = dist + 1;
                    match map[w.0][w.1] {
                        b'#' => {}
                        b'.' => stack.push((dist, v, w)),
                        b @ b'1'..=b'4' | b @ b'A'..=b'Z' | b @ b'a'..=b'z' => {
                            results.push((b, dist))
                        }
                        b => panic!("unexpected tile {b}"),
                    }
                }
            }
            (b0, results)
        })
        .collect();

    let num_keys = map
        .iter()
        .flatten()
        .filter(|&b| b.is_ascii_lowercase())
        .count();
    let all_keys = (1 << num_keys) - 1;

    let p0 = [b'1', b'2', b'3', b'4'];
    let mut heap = BinaryHeap::from([(Reverse(0), 0, p0)]);
    let mut seen: HashSet<(u32, [u8; 4])> = HashSet::from([(0, p0)]);

    while let Some((Reverse(steps), keys, positions)) = heap.pop() {
        if keys == all_keys {
            return steps;
        }
        let update = positions
            .into_iter()
            .enumerate()
            .flat_map(|(idx, u)| graph[&u].iter().map(move |&(v, dist)| (idx, v, dist)))
            .filter_map(move |(idx, v, dist)| match v {
                b'a'..=b'z' => {
                    let key_bit = 1 << (v - b'a');
                    let mut positions = positions;
                    positions[idx] = v;
                    Some((Reverse(steps + dist), keys | key_bit, positions))
                }
                b'A'..=b'Z' => {
                    let key_bit = 1 << (v - b'A');
                    (keys & key_bit > 0).then(|| {
                        let mut positions = positions;
                        positions[idx] = v;
                        (Reverse(steps + dist), keys, positions)
                    })
                }
                b'1'..=b'4' => {
                    let mut positions = positions;
                    positions[idx] = v;
                    Some((Reverse(steps + dist), keys, positions))
                }
                _ => panic!("unexpected node {v}"),
            })
            .filter(|&(_, k, p)| seen.insert((k, p)));
        heap.extend(update)
    }
    panic!("no solution found");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    const EXAMPLE_1: &str = "\
#########
#b.A.@.a#
#########";

    const EXAMPLE_2: &str = "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const EXAMPLE_3: &str = "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const EXAMPLE_4: &str = "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const EXAMPLE_5: &str = "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    #[test_case(EXAMPLE_1 => 8)]
    #[test_case(EXAMPLE_2 => 86)]
    #[test_case(EXAMPLE_3 => 132)]
    #[test_case(EXAMPLE_4 => 136)]
    #[test_case(EXAMPLE_5 => 81)]
    #[test_case(INPUT => 4042)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_6: &str = "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#.b#
#######";

    const EXAMPLE_7: &str = "\
###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############";

    const EXAMPLE_8: &str = "\
#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############";

    const EXAMPLE_9: &str = "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

    #[test_case(EXAMPLE_6 => 8)]
    #[test_case(EXAMPLE_7 => 24)]
    #[test_case(EXAMPLE_8 => 32)]
    #[test_case(EXAMPLE_9 => 72)]
    #[test_case(INPUT => 2014)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
