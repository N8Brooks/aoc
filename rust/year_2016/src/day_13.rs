use std::{iter::successors, mem};

use num::Integer;

pub fn part_1(input: &str, target: (usize, usize)) -> usize {
    bfs(input)
        .position(|frontier| frontier.contains(&target))
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    bfs(input).take(51).map(|frontier| frontier.len()).sum()
}

fn bfs(input: &str) -> impl Iterator<Item = Vec<(usize, usize)>> {
    let number: usize = input.parse().unwrap();
    let frontier = vec![(1, 1)];
    let mut new = [[true; 64]; 64];
    new[1][1] = false;
    successors(Some(frontier), move |frontier| {
        let frontier: Vec<_> = frontier
            .iter()
            .flat_map(|&(x, y): &(usize, usize)| {
                [
                    x.checked_sub(1).map(|x| (x, y)),
                    Some((x + 1, y)),
                    y.checked_sub(1).map(|y| (x, y)),
                    Some((x, y + 1)),
                ]
            })
            .flatten()
            .filter(|&(x, y)| mem::replace(&mut new[y][x], false))
            .filter(|(x, y)| {
                (x * x + 3 * x + 2 * x * y + y + y * y + number)
                    .count_ones()
                    .is_even()
            })
            .collect();
        (!frontier.is_empty()).then_some(frontier)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    const EXAMPLE: &str = "10";

    #[test_case(EXAMPLE, (7, 4) => 11)]
    #[test_case(INPUT, (31, 39) => 90)]
    fn part_1(input: &str, target: (usize, usize)) -> usize {
        super::part_1(input, target)
    }

    #[test_case(INPUT => 135)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
