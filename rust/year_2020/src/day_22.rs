use std::collections::VecDeque;

use hashbrown::HashSet;

pub fn part_1(input: &str) -> usize {
    let [mut cards_1, mut cards_2] = parse_decks(input);

    let winner = if combat(&mut cards_1, &mut cards_2) {
        &cards_1
    } else {
        &cards_2
    };

    score(winner)
}

fn combat(cards_1: &mut VecDeque<usize>, cards_2: &mut VecDeque<usize>) -> bool {
    while !cards_1.is_empty() && !cards_2.is_empty() {
        let card_1 = cards_1.pop_front().unwrap();
        let card_2 = cards_2.pop_front().unwrap();

        if card_1 > card_2 {
            cards_1.extend([card_1, card_2]);
        } else {
            cards_2.extend([card_2, card_1]);
        }
    }

    !cards_1.is_empty()
}

pub fn part_2(input: &str) -> usize {
    let [mut cards_1, mut cards_2] = parse_decks(input);

    let winner = if recursive_combat(&mut cards_1, &mut cards_2) {
        &cards_1
    } else {
        &cards_2
    };

    score(winner)
}

fn recursive_combat(cards_1: &mut VecDeque<usize>, cards_2: &mut VecDeque<usize>) -> bool {
    let mut seen = HashSet::new();

    while !cards_1.is_empty() && !cards_2.is_empty() {
        if !seen.insert((cards_1.clone(), cards_2.clone())) {
            return true;
        }

        let card_1 = cards_1.pop_front().unwrap();
        let card_2 = cards_2.pop_front().unwrap();

        let player_1_wins = if card_1 <= cards_1.len() && card_2 <= cards_2.len() {
            let mut sub_1: VecDeque<_> = cards_1.iter().copied().take(card_1).collect();
            let mut sub_2: VecDeque<_> = cards_2.iter().copied().take(card_2).collect();

            sub_1.iter().max() > sub_2.iter().max() || recursive_combat(&mut sub_1, &mut sub_2)
        } else {
            card_1 > card_2
        };

        if player_1_wins {
            cards_1.extend([card_1, card_2]);
        } else {
            cards_2.extend([card_2, card_1]);
        }
    }

    !cards_1.is_empty()
}
fn parse_decks(input: &str) -> [VecDeque<usize>; 2] {
    let (cards_1, cards_2) = input.split_once("\n\n").unwrap();
    [cards_1, cards_2].map(|cards| {
        cards
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect()
    })
}

fn score(cards: &VecDeque<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i + 1) * card)
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    const EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test_case(EXAMPLE => 306)]
    #[test_case(INPUT => 32783)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 291)]
    #[test_case(INPUT => 33455)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
