use std::{array, iter::once};

use itertools::{Itertools as _, iproduct};

pub fn part_1(input: &str) -> u32 {
    outcomes(input)
        .filter(|(_, win)| *win)
        .map(|(cost, _)| cost)
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> u32 {
    outcomes(input)
        .filter(|(_, win)| !win)
        .map(|(cost, _)| cost)
        .max()
        .unwrap()
}

fn outcomes(input: &str) -> impl Iterator<Item = (u32, bool)> {
    const HP_1: u32 = 100;
    let (hp_2, damage_2, armor_2) = parse_boss(input);
    item_combinations().map(move |[cost, damage_1, armor_1]| {
        let attack_1 = damage_1.saturating_sub(armor_2).max(1);
        let attack_2 = damage_2.saturating_sub(armor_1).max(1);
        let turns_1 = hp_2.div_ceil(attack_1);
        let turns_2 = HP_1.div_ceil(attack_2);
        (cost, turns_1 <= turns_2)
    })
}

fn parse_boss(input: &str) -> (u32, u32, u32) {
    let (hp, damage, armor) = input.lines().collect_tuple().unwrap();
    let hp = hp.strip_prefix("Hit Points: ").unwrap().parse().unwrap();
    let damage = damage.strip_prefix("Damage: ").unwrap().parse().unwrap();
    let armor = armor.strip_prefix("Armor: ").unwrap().parse().unwrap();
    (hp, damage, armor)
}

fn item_combinations() -> impl Iterator<Item = [u32; 3]> {
    const NONE: [u32; 3] = [0, 0, 0];
    const WEAPONS: [[u32; 3]; 5] = [
        [8, 4, 0],  // Dagger
        [10, 5, 0], // Shortsword
        [25, 6, 0], // Warhammer
        [40, 7, 0], // Longsword
        [74, 8, 0], // Greataxe
    ];
    const ARMORS: [[u32; 3]; 5] = [
        [13, 0, 1],  // Leather
        [31, 0, 2],  // Chainmail
        [53, 0, 3],  // Splintmail
        [75, 0, 4],  // Bandedmail
        [102, 0, 5], // Platemail
    ];
    const RINGS: [[u32; 3]; 6] = [
        [25, 1, 0],  // Damage +1
        [50, 2, 0],  // Damage +2
        [100, 3, 0], // Damage +3
        [20, 0, 1],  // Defense +1
        [40, 0, 2],  // Defense +2
        [80, 0, 3],  // Defense +3
    ];
    let weapons = WEAPONS.into_iter();
    let armors = {
        let armors_0 = once(NONE);
        let armors_1 = ARMORS.into_iter();
        armors_0.chain(armors_1)
    };
    let rings = {
        let rings_0 = once(NONE);
        let rings_1 = RINGS.into_iter();
        let rings_2 = RINGS.into_iter().array_combinations::<2>().map(add_items);
        rings_0.chain(rings_1).chain(rings_2)
    };
    iproduct!(weapons, armors, rings).map(|(weapon, armor, ring)| add_items([weapon, armor, ring]))
}

fn add_items<const M: usize>(items: [[u32; 3]; M]) -> [u32; 3] {
    items
        .into_iter()
        .reduce(|a, b| array::from_fn(|i| a[i] + b[i]))
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_21.txt");

    #[test_case(INPUT => 111)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(INPUT => 188)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
