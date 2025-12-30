use std::{
    cell::Cell,
    iter::{empty, repeat_with},
    rc::Rc,
};

use hashbrown::HashSet;
use itertools::Itertools as _;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> usize {
    let program = parse_program(input);
    empty()
        .intcode(program)
        .tuples()
        .fold(HashSet::new(), |mut screen, (x, y, tile_id)| {
            if tile_id == 2 {
                screen.insert((x, y));
            }
            screen
        })
        .len()
}

pub fn part_2(input: &str) -> isize {
    let mut program = parse_program(input);
    program[0] = 2; // play for free

    let ball_x = Rc::new(Cell::new(0));
    let paddle_x = Rc::new(Cell::new(0));

    let inputs = {
        let bx = Rc::clone(&ball_x);
        let px = Rc::clone(&paddle_x);
        repeat_with(move || bx.cmp(&px) as isize)
    };

    inputs
        .intcode(program)
        .tuples()
        .filter_map(|(x, y, tile_id)| {
            if x == -1 && y == 0 {
                return Some(tile_id);
            }
            match tile_id {
                3 => paddle_x.set(x),
                4 => ball_x.set(x),
                _ => {}
            }
            None
        })
        .last()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    #[test_case(INPUT => 326)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 15988)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
