use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    scores(input).min_by_key(|&(turn, _)| turn).unwrap().1
}

pub fn part_2(input: &str) -> usize {
    scores(input).max_by_key(|&(turn, _)| turn).unwrap().1
}

fn scores(input: &str) -> impl Iterator<Item = (usize, usize)> {
    let (numbers, boards) = input.trim().split_once("\n\n").unwrap();
    let numbers: Vec<usize> = numbers.split(',').map(|n| n.parse().unwrap()).collect();

    let mut turns = [None; 100];
    for (i, &num) in numbers.iter().enumerate() {
        turns[num] = Some(i);
    }

    boards.split("\n\n").map(move |board| {
        let board: [[usize; 5]; 5] = board
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_array()
                    .unwrap()
            })
            .collect_array()
            .unwrap();

        let times = board.map(|row| row.map(|n| turns[n]));

        let turn = times
            .iter()
            .map(|row| *row.iter().max().unwrap())
            .chain((0..5).map(|j| times.iter().map(|row| row[j]).max().unwrap()))
            .min()
            .unwrap()
            .unwrap();

        let score: usize = board
            .iter()
            .flatten()
            .filter(|&&n| turns[n] > Some(turn))
            .sum();

        (turn, score * numbers[turn])
    })
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    static INPUT: &str = include_str!("../test_data/day_04.txt");

    static EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test_case(EXAMPLE => 4512)]
    #[test_case(INPUT => 44088)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 1924)]
    #[test_case(INPUT => 23670)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
