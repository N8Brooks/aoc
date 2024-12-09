use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    let (n_players, last_marble) = parse_input(input);
    winning_score(n_players, last_marble)
}

pub fn part_2(input: &str) -> usize {
    let (n_players, last_marble) = parse_input(input);
    winning_score(n_players, 100 * last_marble)
}

fn parse_input(input: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    let n_players = caps[1].parse().unwrap();
    let last_marble = caps[2].parse().unwrap();
    (n_players, last_marble)
}

fn winning_score(n_players: usize, last_marble: usize) -> usize {
    let mut player_scores = vec![0; n_players];
    let mut marbles = VecDeque::from([0]);
    for marble in 1..=last_marble {
        if marble.is_multiple_of(23) {
            marbles.rotate_right(7);
            let player_index = (marble - 1) % n_players;
            player_scores[player_index] += marble + marbles.pop_back().unwrap();
            marbles.rotate_left(1);
        } else {
            marbles.rotate_left(1);
            marbles.push_back(marble);
        }
    }
    *player_scores.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../test_data/year_2018/day_09.txt");

    #[test_case("9 players; last marble is worth 25 points", 32; "example_1")]
    #[test_case("10 players; last marble is worth 1618 points", 8317; "example_2")]
    #[test_case("13 players; last marble is worth 7999 points", 146373; "example_3")]
    #[test_case("17 players; last marble is worth 1104 points", 2764; "example_4")]
    #[test_case("21 players; last marble is worth 6111 points", 54718; "example_5")]
    #[test_case("30 players; last marble is worth 5807 points", 37305; "example_6")]
    #[test_case(INPUT, 398371; "input")]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, 3212830280; "input")]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
