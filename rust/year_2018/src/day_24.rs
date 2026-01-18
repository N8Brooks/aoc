use std::cmp::Reverse;

use itertools::Itertools as _;
use num::Zero as _;

pub fn part_1(input: &str) -> u32 {
    let groups = parse_armies(input);
    total_units(groups).unsigned_abs()
}

pub fn part_2(input: &str) -> u32 {
    let groups = parse_armies(input);
    let copy = |boost: u32| -> Vec<_> {
        groups
            .iter()
            .copied()
            .map(|mut g| {
                if g.side == IMMUNE {
                    g.attack_damage += boost;
                }
                g
            })
            .collect()
    };
    let (mut lo, mut hi) = (0, 1);
    let mut min_units = loop {
        let groups = copy(hi);
        let units = total_units(groups);
        if units.is_positive() {
            break units;
        }
        lo = hi + 1;
        hi *= 2;
    };
    while lo <= hi {
        let mi = (lo + hi) / 2;
        let groups = copy(mi);
        let units = total_units(groups);
        if units.is_positive() {
            min_units = units;
            hi = mi - 1;
        } else {
            lo = mi + 1;
        }
    }
    min_units.try_into().unwrap()
}

fn total_units(mut groups: Vec<Group>) -> i32 {
    while !groups.iter().map(|g| g.side).all_equal() {
        // Target selection phase
        let mut targeted = vec![false; groups.len()];
        let attacks = (0..groups.len())
            .sorted_by_cached_key(|&i| {
                let g = &groups[i];
                Reverse((g.units * g.attack_damage, g.initiative))
            })
            .filter_map(|i| {
                let a = &groups[i];
                groups
                    .iter()
                    .enumerate()
                    .filter(|&(j, d)| {
                        d.side != a.side
                            && !targeted[j]
                            && !(a.attack_damage * a.units * d.modifiers[a.attack_kind]).is_zero()
                    })
                    .max_by_key(|(_, d)| {
                        (
                            a.attack_damage * a.units * d.modifiers[a.attack_kind],
                            d.units * d.attack_damage,
                            d.initiative,
                        )
                    })
                    .map(|(j, _)| {
                        targeted[j] = true;
                        (i, j)
                    })
            });

        // Attack phase
        if attacks
            .sorted_unstable_by_key(|&(i, _)| Reverse(groups[i].initiative))
            .fold(true, |stalemate, (i, j)| {
                let [a, d] = groups.get_disjoint_mut([i, j]).unwrap();
                let damage = a.attack_damage * a.units * d.modifiers[a.attack_kind];
                let killed = damage / d.hit_points;
                d.units = d.units.saturating_sub(killed);
                stalemate && killed.is_zero()
            })
        {
            return 0;
        }

        groups.retain(|g| g.units > 0);
    }

    groups
        .iter()
        .map(|g| i32::try_from(g.units).unwrap() * g.side)
        .sum()
}

fn parse_armies(input: &str) -> Vec<Group> {
    let (immune, infection) = input.split_once("\n\n").unwrap();
    let immune = immune.strip_prefix("Immune System:\n").unwrap();
    let immune = parse_army(immune, IMMUNE);
    let infection = infection.strip_prefix("Infection:\n").unwrap();
    let infection = parse_army(infection, INFECTION);
    immune.chain(infection).collect()
}

fn parse_army(input: &str, side: Side) -> impl Iterator<Item = Group> {
    input.lines().map(move |line| {
        let (units, line) = line.split_once(" units each with ").unwrap();
        let units: u32 = units.parse().unwrap();
        let (hit_points, line) = line.split_once(" hit points ").unwrap();
        let hit_points: u32 = hit_points.parse().unwrap();
        let (modifiers, line) = line.split_once("with an attack that does ").unwrap();

        let (immune, weak) = if let Some(modifiers) = modifiers.strip_circumfix('(', ") ") {
            modifiers
                .split("; ")
                .fold((None, None), |(immune, weak), part| {
                    if let Some(immune_str) = part.strip_prefix("immune to ") {
                        assert!(immune.is_none());
                        (Some(immune_str), weak)
                    } else if let Some(weak_str) = part.strip_prefix("weak to ") {
                        assert!(weak.is_none());
                        (immune, Some(weak_str))
                    } else {
                        panic!("invalid modifiers: {}", modifiers);
                    }
                })
        } else {
            (None, None)
        };

        let mut modifiers = [1; 5];
        if let Some(immune) = immune {
            for attack_type in immune.split(", ") {
                let i = attack_index(attack_type);
                modifiers[i] = 0;
            }
        }
        if let Some(weak) = weak {
            for attack_type in weak.split(", ") {
                let i = attack_index(attack_type);
                modifiers[i] = 2;
            }
        }

        let (attack, initiative) = line.split_once(" damage at initiative ").unwrap();
        let (attack_damage, attack_kind) = attack.split_once(' ').unwrap();
        let attack_damage: u32 = attack_damage.parse().unwrap();
        let attack_kind = attack_index(attack_kind);
        let initiative: u32 = initiative.parse().unwrap();

        Group {
            side,
            units,
            hit_points,
            modifiers,
            attack_damage,
            attack_kind,
            initiative,
        }
    })
}

type Side = i32;
const IMMUNE: Side = 1;
const INFECTION: Side = -1;

#[derive(Copy, Clone)]
struct Group {
    side: Side,
    units: u32,
    hit_points: u32,
    modifiers: [u32; 5],
    attack_damage: u32,
    attack_kind: usize,
    initiative: u32,
}

fn attack_index(attack_type: &str) -> usize {
    match attack_type {
        "fire" => 0,
        "cold" => 1,
        "slashing" => 2,
        "bludgeoning" => 3,
        "radiation" => 4,
        _ => panic!("Unknown attack type: {attack_type}"),
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_24.txt");

    const EXAMPLE_1: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test_case(EXAMPLE_1 => 5216)]
    #[test_case(INPUT => 15470)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 51)]
    #[test_case(INPUT => 5742)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
