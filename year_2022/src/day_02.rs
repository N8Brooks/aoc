use itertools::Itertools;

#[derive(Copy, Clone)]
pub enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Shape {
    pub fn from_byte(byte: &u8) -> Shape {
        match byte {
            b'A' => Shape::Rock,
            b'B' => Shape::Paper,
            b'C' => Shape::Scissors,
            byte => panic!("invalid input: {byte}"),
        }
    }

    pub fn from_outcome(opponent: &Shape, outcome: &Outcome) -> Shape {
        let opponent = *opponent as u8;
        let outcome = *outcome as u8;
        let player = (outcome + opponent) % 3 + b'A';
        Shape::from_byte(&player)
    }

    pub fn get_score(&self) -> usize {
        *self as usize + 1
    }
}

#[derive(Copy, Clone)]
pub enum Outcome {
    Lose = 2, // -1 modulo 3
    Draw = 0,
    Win = 1,
}

impl Outcome {
    pub fn from_byte(byte: &u8) -> Outcome {
        match byte {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            byte => panic!("invalid input: {byte}"),
        }
    }

    pub fn from_round(opponent: &Shape, player: &Shape) -> Outcome {
        let player = *player as u8;
        let opponent = *opponent as u8;
        let outcome = (player + 3 - opponent) % 3;
        match outcome {
            2 => Outcome::Lose,
            0 => Outcome::Draw,
            1 => Outcome::Win,
            outcome => panic!("invalid outcome: {outcome}"),
        }
    }

    pub fn get_score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

pub fn get_total_score(opponent: &Shape, player: &Shape) -> usize {
    player.get_score() + Outcome::from_round(opponent, player).get_score()
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opponent, player) = line.split_once(' ').unwrap();
            let opponent = opponent.bytes().exactly_one().unwrap();
            let opponent = Shape::from_byte(&opponent);
            let player = player.bytes().exactly_one().unwrap() - b'X' + b'A';
            let player = Shape::from_byte(&player);
            get_total_score(&opponent, &player)
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opponent, outcome) = line.split_once(' ').unwrap();
            let opponent = opponent.bytes().exactly_one().unwrap();
            let opponent = Shape::from_byte(&opponent);
            let outcome = outcome.bytes().exactly_one().unwrap();
            let outcome = Outcome::from_byte(&outcome);
            let player = Shape::from_outcome(&opponent, &outcome);
            get_total_score(&opponent, &player)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "A Y
B X
C Z";

    const INPUT: &str = include_str!("testdata/day_02.txt");

    #[test_case("A X", 4)]
    #[test_case("A Y", 8)]
    #[test_case("A Z", 3)]
    #[test_case("B X", 1)]
    #[test_case("B Y", 5)]
    #[test_case("B Z", 9)]
    #[test_case("C X", 7)]
    #[test_case("C Y", 2)]
    #[test_case("C Z", 6)]
    #[test_case(EXAMPLE, 15)]
    #[test_case(INPUT, 12645)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 12)]
    #[test_case(INPUT, 11756)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
