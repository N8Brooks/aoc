use std::{cell::Cell, iter::repeat_with, rc::Rc};

use hashbrown::HashMap;
use itertools::Itertools as _;
use num::Complex;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> usize {
    paint_panels(input, false).len()
}

pub fn part_2(input: &str) -> String {
    let painted = paint_panels(input, true);
    let (min_j, max_j) = painted.keys().map(|p| p.re).minmax().into_option().unwrap();
    let (min_i, max_i) = painted.keys().map(|p| p.im).minmax().into_option().unwrap();
    (min_i..=max_i)
        .rev()
        .map(|y| -> String {
            (min_j..=max_j)
                .map(|x| {
                    let p = Complex::new(x, y);
                    let c = painted.get(&p).copied().unwrap_or(false);
                    if c { '#' } else { ' ' }
                })
                .collect()
        })
        .join("\n")
}

fn paint_panels(input: &str, init: bool) -> HashMap<Complex<isize>, bool> {
    const R: Complex<isize> = Complex::new(0, -1);
    const L: Complex<isize> = Complex::new(0, 1);
    let mut painted = HashMap::new();
    let mut dir = Complex::new(0, 1);
    let mut loc = Complex::new(0, 0);
    painted.insert(loc, init);

    let color = Rc::new(Cell::new(init));
    let inputs = {
        let color = Rc::clone(&color);
        repeat_with(move || color.get() as isize)
    };

    inputs
        .intcode(parse_program(input))
        .map(|output| output != 0)
        .tuples()
        .for_each(|(new_color, right)| {
            painted.insert(loc, new_color);
            dir *= if right { R } else { L };
            loc += dir;
            color.set(painted.get(&loc).copied().unwrap_or(false));
        });

    painted
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
