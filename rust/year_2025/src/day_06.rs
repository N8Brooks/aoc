use std::iter;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (operands, operators) = input.rsplit_once('\n').unwrap();
    let mut rows: Vec<_> = operands
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse().unwrap()))
        .collect();
    let lists = transpose::<usize>(&mut rows);
    operators
        .split_ascii_whitespace()
        .zip_eq(lists)
        .map(move |(op, list)| -> usize {
            match op {
                "*" => list.into_iter().product(),
                "+" => list.into_iter().sum(),
                _ => panic!("unknown op {op}"),
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let (operands, operators) = input.rsplit_once('\n').unwrap();
    let mut rows: Vec<_> = operands
        .lines()
        .map(|line| line.bytes().map(|b| b.checked_sub(b'0').map(usize::from)))
        .collect();
    let lists = iter::from_fn(|| {
        let list: Vec<_> = transpose(&mut rows)
            .map_while(|col| {
                col.into_iter()
                    .flatten()
                    .reduce(|num, digit| num * 10 + digit)
            })
            .collect();
        (!list.is_empty()).then_some(list)
    });
    operators
        .split_ascii_whitespace()
        .zip_eq(lists)
        .map(move |(op, list)| -> usize {
            match op {
                "*" => list.into_iter().product(),
                "+" => list.into_iter().sum(),
                _ => panic!("unknown op {op}"),
            }
        })
        .sum()
}

fn transpose<T>(rows: &mut [impl Iterator<Item = T>]) -> impl Iterator<Item = Vec<T>> {
    iter::from_fn(move || rows.iter_mut().map(|row| row.next()).collect())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test_case(EXAMPLE => 4277556)]
    #[test_case(INPUT => 5873191732773)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 3263827)]
    #[test_case(INPUT => 11386445308378)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
