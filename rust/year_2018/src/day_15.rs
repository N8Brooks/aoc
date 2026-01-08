use std::{iter::successors, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Goblin,
    Elf,
}

pub fn part_1(input: &str) -> usize {
    let mut map: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut units: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &b)| (i, j, b)))
        .filter_map(|(i, j, b)| match b {
            b'G' => Some(((i, j), Unit::Goblin, 200_usize)),
            b'E' => Some(((i, j), Unit::Elf, 200_usize)),
            _ => None,
        })
        .collect();
    for round in 0.. {
        for k in 0..units.len() {
            let ((i, j), unit_type, hp) = units[k];
            if hp == 0 {
                continue;
            }
            let targets: Vec<_> = units
                .iter()
                .enumerate()
                .filter_map(|(j, &(other_pos, other_type, other_hp))| {
                    if unit_type != other_type && other_hp > 0 {
                        Some((j, other_pos, other_hp))
                    } else {
                        None
                    }
                })
                .collect();
            if targets.is_empty() {
                // battle ends
                let total_hp: usize = units.iter().map(|(_, _, hp)| *hp).sum();
                return round * total_hp;
            }
            if !targets
                .iter()
                .any(|(_, (ti, tj), _)| ti.abs_diff(i) + tj.abs_diff(j) == 1)
            {
                let open_squares: Vec<_> = targets
                    .iter()
                    .flat_map(|&(_, (ti, tj), _)| {
                        [(ti - 1, tj), (ti, tj - 1), (ti, tj + 1), (ti + 1, tj)]
                    })
                    .filter(|&(i, j)| map[i][j] == b'.')
                    .collect();
                if open_squares.is_empty() {
                    continue;
                }
                let mut new = vec![vec![true; map[0].len()]; map.len()];
                let frontier: Vec<_> = [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
                    .into_iter()
                    .filter(|&(i, j)| map[i][j] == b'.')
                    .inspect(|&(i, j)| new[i][j] = false)
                    .map(|pos| (pos, pos))
                    .collect();
                let mut frontiers = successors(Some(frontier), |frontier| {
                    let frontier: Vec<_> = frontier
                        .iter()
                        .flat_map(|&((i, j), first_step)| {
                            [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
                                .into_iter()
                                .filter(|&(i, j)| map[i][j] == b'.')
                                .map(move |pos| (pos, first_step))
                        })
                        .filter(|&((i, j), _)| mem::replace(&mut new[i][j], false))
                        .collect();
                    (!frontier.is_empty()).then_some(frontier)
                });
                if let Some((_, first_step @ (ni, nj))) = frontiers.find_map(|frontier| {
                    frontier
                        .into_iter()
                        .filter(|(pos, _)| open_squares.contains(pos))
                        .min()
                }) {
                    map[i][j] = b'.';
                    map[ni][nj] = b'X';
                    units[k].0 = first_step;
                }
            }
            let (i, j) = units[k].0;
            if let Some((k, _, _)) = targets
                .into_iter()
                .filter(|&(_, (ti, tj), _)| ti.abs_diff(i) + tj.abs_diff(j) == 1)
                .min_by_key(|&(_, pos, hp)| (hp, pos))
            {
                let damage = 3;
                let target_hp = &mut units[k].2;
                *target_hp = target_hp.saturating_sub(damage);
                if *target_hp == 0 {
                    let (ti, tj) = units[k].0;
                    map[ti][tj] = b'.';
                }
            }
        }

        units.retain(|&(_, _, hp)| hp > 0);
        units.sort_by_key(|(pos, _, _)| *pos);
    }
    unreachable!()
}

pub fn part_2(input: &str) -> usize {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let units: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &b)| (i, j, b)))
        .filter_map(|(i, j, b)| match b {
            b'G' => Some(((i, j), Unit::Goblin, 200_usize)),
            b'E' => Some(((i, j), Unit::Elf, 200_usize)),
            _ => None,
        })
        .collect();
    let mut lo = 4;
    let mut hi = 4;
    let mut best = loop {
        if let Some(outcome) = elf_outcome(map.clone(), units.clone(), hi) {
            break outcome;
        }
        lo = hi + 1;
        hi *= 2;
    };
    while lo <= hi {
        let mi = (lo + hi) / 2;
        if let Some(outcome) = elf_outcome(map.clone(), units.clone(), mi) {
            best = outcome;
            hi = mi - 1;
        } else {
            lo = mi + 1;
        }
    }
    best
}

fn elf_outcome(
    mut map: Vec<Vec<u8>>,
    mut units: Vec<((usize, usize), Unit, usize)>,
    elf_damage: usize,
) -> Option<usize> {
    for round in 0.. {
        for k in 0..units.len() {
            let ((i, j), unit_type, hp) = units[k];
            if hp == 0 {
                continue;
            }
            let targets: Vec<_> = units
                .iter()
                .enumerate()
                .filter_map(|(j, &(other_pos, other_type, other_hp))| {
                    if unit_type != other_type && other_hp > 0 {
                        Some((j, other_pos, other_hp))
                    } else {
                        None
                    }
                })
                .collect();
            if targets.is_empty() {
                // battle ends - assumes non zero elf units
                let total_hp: usize = units.iter().map(|(_, _, hp)| *hp).sum();
                return Some(round * total_hp);
            }
            if !targets
                .iter()
                .any(|(_, (ti, tj), _)| ti.abs_diff(i) + tj.abs_diff(j) == 1)
            {
                let open_squares: Vec<_> = targets
                    .iter()
                    .flat_map(|&(_, (ti, tj), _)| {
                        [(ti - 1, tj), (ti, tj - 1), (ti, tj + 1), (ti + 1, tj)]
                    })
                    .filter(|&(i, j)| map[i][j] == b'.')
                    .collect();
                if open_squares.is_empty() {
                    continue;
                }
                let mut new = vec![vec![true; map[0].len()]; map.len()];
                let frontier: Vec<_> = [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
                    .into_iter()
                    .filter(|&(i, j)| map[i][j] == b'.')
                    .inspect(|&(i, j)| new[i][j] = false)
                    .map(|pos| (pos, pos))
                    .collect();
                let mut frontiers = successors(Some(frontier), |frontier| {
                    let frontier: Vec<_> = frontier
                        .iter()
                        .flat_map(|&((i, j), first_step)| {
                            [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
                                .into_iter()
                                .filter(|&(i, j)| map[i][j] == b'.')
                                .map(move |pos| (pos, first_step))
                        })
                        .filter(|&((i, j), _)| mem::replace(&mut new[i][j], false))
                        .collect();
                    (!frontier.is_empty()).then_some(frontier)
                });
                if let Some((_, first_step @ (ni, nj))) = frontiers.find_map(|frontier| {
                    frontier
                        .into_iter()
                        .filter(|(pos, _)| open_squares.contains(pos))
                        .min()
                }) {
                    map[i][j] = b'.';
                    map[ni][nj] = b'X';
                    units[k].0 = first_step;
                }
            }
            let (i, j) = units[k].0;
            if let Some((k, _, _)) = targets
                .into_iter()
                .filter(|&(_, (ti, tj), _)| ti.abs_diff(i) + tj.abs_diff(j) == 1)
                .min_by_key(|&(_, pos, hp)| (hp, pos))
            {
                let damage = if unit_type == Unit::Elf {
                    elf_damage
                } else {
                    3
                };
                let target_hp = &mut units[k].2;
                *target_hp = target_hp.saturating_sub(damage);
                if *target_hp == 0 {
                    if units[k].1 == Unit::Elf {
                        return None;
                    }
                    let (ti, tj) = units[k].0;
                    map[ti][tj] = b'.';
                }
            }
        }

        units.retain(|&(_, _, hp)| hp > 0);
        units.sort_by_key(|(pos, _, _)| *pos);
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    const EXAMPLE_1: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const EXAMPLE_2: &str = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    const EXAMPLE_3: &str = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const EXAMPLE_4: &str = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const EXAMPLE_5: &str = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const EXAMPLE_6: &str = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test_case(EXAMPLE_1 => 27730)]
    #[test_case(EXAMPLE_2 => 36334)]
    #[test_case(EXAMPLE_3 => 39514)]
    #[test_case(EXAMPLE_4 => 27755)]
    #[test_case(EXAMPLE_5 => 28944)]
    #[test_case(EXAMPLE_6 => 18740)]
    #[test_case(INPUT => 179968)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 4988)]
    #[test_case(EXAMPLE_2 => 29064)]
    #[test_case(EXAMPLE_3 => 31284)]
    #[test_case(EXAMPLE_4 => 3478)]
    #[test_case(EXAMPLE_5 => 6474)]
    #[test_case(EXAMPLE_6 => 1140)]
    #[test_case(INPUT => 42098)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
