use std::iter::{self, successors};

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_1(input: &str) -> usize {
    fn step(seats: &[Vec<Option<bool>>], [i, j]: [usize; 2]) -> Option<bool> {
        seats[i][j].map(|occupied| {
            let threshold = if occupied { 4 } else { 1 };
            DIRS.into_iter()
                .filter_map(|(di, dj)| {
                    let i = i.checked_add_signed(di)?;
                    let j = j.checked_add_signed(dj)?;
                    seats.get(i)?.get(j)
                })
                .filter_map(|&seat| seat)
                .filter(|&occupied| occupied)
                .nth(threshold - 1)
                .is_none()
        })
    }

    solve(input, step)
}

pub fn part_2(input: &str) -> usize {
    fn step(seats: &[Vec<Option<bool>>], [i, j]: [usize; 2]) -> Option<bool> {
        seats[i][j].map(|occupied| {
            let threshold = if occupied { 5 } else { 1 };
            DIRS.into_iter()
                .filter_map(|(di, dj)| {
                    let (mut i, mut j) = (i, j);
                    iter::from_fn(|| {
                        i = i.checked_add_signed(di)?;
                        j = j.checked_add_signed(dj)?;
                        seats.get(i)?.get(j)
                    })
                    .find_map(|&seat| seat)
                })
                .filter(|&occupied| occupied)
                .nth(threshold - 1)
                .is_none()
        })
    }

    solve(input, step)
}

pub fn solve(
    input: &str,
    step: impl Fn(&[Vec<Option<bool>>], [usize; 2]) -> Option<bool>,
) -> usize {
    let seats_0 = parse_seats(input);
    let seats_n = successors(Some(seats_0), |seats_1| {
        let seats_2 = seats_1
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, _)| step(seats_1, [i, j]))
                    .collect()
            })
            .collect();
        (seats_1 != &seats_2).then_some(seats_2)
    })
    .last()
    .unwrap();
    seats_n
        .into_iter()
        .flatten()
        .flatten()
        .filter(|&occupied| occupied)
        .count()
}

fn parse_seats(input: &str) -> Vec<Vec<Option<bool>>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'L' => Some(false),
                    b'#' => Some(true),
                    b'.' => None,
                    _ => panic!("invalid seat {}", b as char),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test_case(EXAMPLE => 37)]
    #[test_case(INPUT => 2178)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 26)]
    #[test_case(INPUT => 1978)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
