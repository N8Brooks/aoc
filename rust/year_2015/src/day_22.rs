use std::{cmp::Reverse, collections::BinaryHeap, num::NonZeroU32};

use itertools::Itertools as _;

pub fn part_1(input: &str) -> u32 {
    min_mana(input, 0)
}

pub fn part_2(input: &str) -> u32 {
    min_mana(input, 1)
}

fn min_mana(input: &str, damage_1: u32) -> u32 {
    let (hp_2, damage_2) = parse_boss(input);
    let mut heap = BinaryHeap::from([State::new(hp_2)]);
    while let Some(state) = heap.pop() {
        if state.hp_2.is_none() {
            return state.spent.0;
        }
        if state.turn {
            heap.extend(state.player_turn(damage_1));
        } else {
            if let Some(state) = state.boss_turn(damage_1, damage_2) {
                heap.push(state);
            }
        }
    }
    panic!("no solution")
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct State {
    spent: Reverse<u32>,
    turn: bool,
    hp_1: NonZeroU32,
    hp_2: Option<NonZeroU32>,
    mana: u32,
    shield: u32,
    poison: u32,
    recharge: u32,
}

impl State {
    fn new(hp_2: u32) -> Self {
        Self {
            spent: Reverse(0),
            turn: true,
            hp_1: NonZeroU32::new(50).unwrap(),
            hp_2: NonZeroU32::new(hp_2),
            mana: 500,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }

    fn player_turn(&self, damage_1: u32) -> impl Iterator<Item = Self> {
        // Start of turn effects
        let spent = self.spent.0;
        let Some(hp_1) = NonZeroU32::new(self.hp_1.get().saturating_sub(damage_1)) else {
            return None.into_iter().flatten();
        };
        let hp_2 = self
            .hp_2
            .unwrap()
            .get()
            .saturating_sub(if self.poison > 0 { 3 } else { 0 });
        let mana = self.mana + if self.recharge > 0 { 101 } else { 0 };
        let shield_1 = self.shield.saturating_sub(1);
        let poison_1 = self.poison.saturating_sub(1);
        let recharge_1 = self.recharge.saturating_sub(1);

        // Spell effects
        let res = [
            (53, 4, 0, 0, 0, 0),
            (73, 2, 2, 0, 0, 0),
            (113, 0, 0, 6, 0, 0),
            (173, 0, 0, 0, 6, 0),
            (229, 0, 0, 0, 0, 5),
        ]
        .into_iter()
        .filter(move |&(cost, _, _, shield_2, poison_2, recharge_2)| {
            cost <= mana
                && (shield_1 == 0 || shield_2 == 0)
                && (poison_1 == 0 || poison_2 == 0)
                && (recharge_1 == 0 || recharge_2 == 0)
        })
        .map(
            move |(cost, damage, heal, shield_2, poison_2, recharge_2)| State {
                spent: Reverse(spent + cost),
                turn: false,
                hp_1: (hp_1.get() + heal).try_into().unwrap(),
                hp_2: hp_2.saturating_sub(damage).try_into().ok(),
                mana: mana - cost,
                shield: shield_1 | shield_2,
                poison: poison_1 | poison_2,
                recharge: recharge_1 | recharge_2,
            },
        );

        Some(res).into_iter().flatten()
    }

    fn boss_turn(&self, damage_1: u32, damage_2: u32) -> Option<Self> {
        let hp_2 = self
            .hp_2
            .unwrap()
            .get()
            .saturating_sub(if self.poison > 0 { 3 } else { 0 })
            .try_into()
            .ok();
        let attack = if hp_2.is_some() {
            let armor = if self.shield > 0 { 7 } else { 0 };
            damage_2.saturating_sub(armor).max(1)
        } else {
            0
        };
        Some(State {
            spent: self.spent,
            turn: true,
            hp_1: self
                .hp_1
                .get()
                .checked_sub(damage_1 + attack)?
                .try_into()
                .ok()?,
            hp_2,
            mana: (self.mana + if self.recharge > 0 { 101 } else { 0 }),
            shield: self.shield.saturating_sub(1),
            poison: self.poison.saturating_sub(1),
            recharge: self.recharge.saturating_sub(1),
        })
    }
}

fn parse_boss(input: &str) -> (u32, u32) {
    let (hp, damage) = input.lines().collect_tuple().unwrap();
    let hp = hp.strip_prefix("Hit Points: ").unwrap().parse().unwrap();
    let damage = damage.strip_prefix("Damage: ").unwrap().parse().unwrap();
    (hp, damage)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    #[test_case(INPUT => 1269)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(INPUT => 1309)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
