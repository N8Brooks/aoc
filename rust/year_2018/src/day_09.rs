use std::collections::VecDeque;

pub fn part_1(input: &str) -> usize {
    let (n_players, last_marble) = parse_input(input);
    winning_score(n_players, last_marble)
}

pub fn part_2(input: &str) -> usize {
    let (n_players, last_marble) = parse_input(input);
    winning_score(n_players, 100 * last_marble)
}

fn parse_input(input: &str) -> (usize, usize) {
    let (n_players, last_marble) = input.split_once("; ").unwrap();
    let n_players = n_players.strip_suffix(" players").unwrap();
    let last_marble = last_marble
        .strip_circumfix("last marble is worth ", " points")
        .unwrap();
    (n_players.parse().unwrap(), last_marble.parse().unwrap())
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
    player_scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    #[test_case("9 players; last marble is worth 25 points" => 32)]
    #[test_case("10 players; last marble is worth 1618 points" => 8317)]
    #[test_case("13 players; last marble is worth 7999 points" => 146373)]
    #[test_case("17 players; last marble is worth 1104 points" => 2764)]
    #[test_case("21 players; last marble is worth 6111 points" => 54718)]
    #[test_case("30 players; last marble is worth 5807 points" => 37305)]
    #[test_case(INPUT => 398371)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 3212830280)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
