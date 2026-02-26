use std::{cell::Cell, iter::repeat_with, rc::Rc};

use hashbrown::HashMap;
use itertools::Itertools as _;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> usize {
    let (mut di, mut dj) = (-1, 0);
    let (mut i, mut j) = (0, 0);
    let mut painted = HashMap::new();

    let color = Rc::new(Cell::new(false));
    let inputs = {
        let color = Rc::clone(&color);
        repeat_with(move || color.get().into())
    };

    inputs
        .intcode(parse_program(input))
        .map(|output| output != 0)
        .tuples()
        .for_each(|(new_color, right)| {
            painted.insert((i, j), new_color);
            (di, dj) = if right { (dj, -di) } else { (-dj, di) };
            i += di;
            j += dj;
            color.set(painted.get(&(i, j)).is_some_and(|&b| b));
        });

    painted.len()
}

pub fn part_2(input: &str) -> String {
    let (mut di, mut dj) = (-1, 0);
    let (mut i, mut j) = (0, 0);
    let mut painted = [[b' '; 43]; 6];
    painted[i][j] = b'#';

    let color = Rc::new(Cell::new(true));
    let inputs = {
        let color = Rc::clone(&color);
        repeat_with(move || color.get().into())
    };

    inputs
        .intcode(parse_program(input))
        .map(|output| output != 0)
        .tuples()
        .for_each(|(new_color, right)| {
            painted[i][j] = if new_color { b'#' } else { b' ' };
            (di, dj) = if right { (dj, -di) } else { (-dj, di) };
            i = i.strict_add_signed(di);
            j = j.strict_add_signed(dj);
            color.set(painted[i][j] == b'#');
        });

    transpose(painted)
        .as_chunks()
        .0
        .iter()
        .map(|&cols| match transpose(cols).each_ref() {
            #[rustfmt::skip]
            [
                b" ### ",
                b" #  #",
                b" ### ",
                b" #  #",
                b" #  #",
                b" ### ",
            ] => 'B',
            #[rustfmt::skip]
            [
                b"  ## ",
                b" #  #",
                b" #   ",
                b" #   ",
                b" #  #",
                b"  ## ",
            ] => 'C',
            #[rustfmt::skip]
            [
                b" ####",
                b" #   ",
                b" ### ",
                b" #   ",
                b" #   ",
                b" #   ",
            ] => 'F',
            #[rustfmt::skip]
            [
                b" ### ",
                b" #  #",
                b" #  #",
                b" ### ",
                b" #   ",
                b" #   ",
            ] => 'P',
            #[rustfmt::skip]
            [
                b" #  #",
                b" #  #",
                b" #  #",
                b" #  #",
                b" #  #",
                b"  ## ",
            ] => 'U',
            #[rustfmt::skip]
            [
                b" ####",
                b"    #",
                b"   # ",
                b"  #  ",
                b" #   ",
                b" ####",
            ] => 'Z',
            rows => panic!(
                "unknown letter:\n{}",
                rows.map(|row| str::from_utf8(row).unwrap()).join("\n")
            ),
        })
        .collect()
}

/// Transposes a 2D array of size MxN into one of size NxM.
fn transpose<const M: usize, const N: usize, T>(m: [[T; N]; M]) -> [[T; M]; N] {
    use std::array::from_fn;
    let mut iters = m.map(|r| r.into_iter());
    from_fn(|_| from_fn(|i| iters[i].next().unwrap()))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    #[test_case(INPUT => 2415)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => "BFPUZUPC")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
