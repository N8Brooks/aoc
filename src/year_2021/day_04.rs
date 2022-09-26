/// Bingo gamestate
struct Bingo {
    board: Vec<Vec<Option<u32>>>,
}

impl Bingo {
    fn new(input: &str) -> Bingo {
        Bingo {
            board: input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| Some(num.parse().unwrap()))
                        .collect()
                })
                .collect(),
        }
    }

    fn draw(&mut self, num: u32) {
        for (i, row) in self.board.iter().enumerate() {
            if let Some(j) = row.iter().position(|&val| val == Some(num)) {
                self.board[i][j] = None;
                return;
            }
        }
    }

    fn is_winner(&self) -> bool {
        let is_row_winner = self
            .board
            .iter()
            .any(|row| row.iter().all(|num| num.is_none()));
        let is_col_winner = (0..5).any(|j| self.board.iter().all(|num| num[j].is_none()));
        is_row_winner || is_col_winner
    }

    fn score(&self, called_num: u32) -> u32 {
        self.board.iter().flatten().flatten().sum::<u32>() * called_num
    }
}

pub fn part_1(input: &str) -> u32 {
    let (nums, boards) = input.split_once("\n\n").unwrap();
    let mut boards: Vec<Bingo> = boards.split("\n\n").map(Bingo::new).collect();
    for num in nums.split(',').map(|num| num.parse().unwrap()) {
        for board in boards.iter_mut() {
            board.draw(num);
            if board.is_winner() {
                return board.score(num);
            }
        }
    }
    panic!("no winning board");
}

pub fn part_2(input: &str) -> u32 {
    let (nums, boards) = input.split_once("\n\n").unwrap();
    let mut boards: Vec<Bingo> = boards.split("\n\n").map(Bingo::new).collect();
    for num in nums.split(',').map(|num| num.parse().unwrap()) {
        for board in boards.iter_mut() {
            board.draw(num);
        }
        if boards.len() == 1 && boards[0].is_winner() {
            return boards[0].score(num);
        }
        boards.retain_mut(|board| !board.is_winner());
    }
    panic!("no one loosing board")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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

    static INPUT: &str = include_str!("testdata/day_04.txt");

    #[test_case(EXAMPLE, 4512)]
    #[test_case(INPUT, 44088)]
    fn part_1_examples(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 1924)]
    #[test_case(INPUT, 23670)]
    fn part_2_examples(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
