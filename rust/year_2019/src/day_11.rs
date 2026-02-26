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
        repeat_with(move || color.get() as isize)
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

    let color = Rc::new(Cell::new(b'#'));
    let inputs = {
        let color = Rc::clone(&color);
        repeat_with(move || (color.get() == b'#') as isize)
    };

    inputs
        .intcode(parse_program(input))
        .map(|output| output != 0)
        .tuples()
        .for_each(|(new_color, right)| {
            painted[i][j] = if new_color { b'#' } else { b' ' };
            (di, dj) = if right { (dj, -di) } else { (-dj, di) };
            (i, j) = (i.strict_add_signed(di), j.strict_add_signed(dj));
            color.set(painted[i][j]);
        });

    painted
        .each_ref()
        .into_iter()
        .map(|row| row.as_slice())
        .intersperse(b"\n")
        .flatten()
        .map(|&b| b as char)
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    #[test_case(INPUT => 2415)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXPECTED_2: &str = " \
 ###  #### ###  #  # #### #  # ###   ##    
 #  # #    #  # #  #    # #  # #  # #  #   
 ###  ###  #  # #  #   #  #  # #  # #      
 #  # #    ###  #  #  #   #  # ###  #      
 #  # #    #    #  # #    #  # #    #  #   
 ###  #    #     ##  ####  ##  #     ##    ";

    #[test_case(INPUT => EXPECTED_2)]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
