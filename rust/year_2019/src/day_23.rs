use std::{
    array,
    cell::RefCell,
    collections::VecDeque,
    iter::{self, successors},
    rc::Rc,
};

use itertools::Itertools as _;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> isize {
    iter_nats(input).flatten().next().unwrap()[1]
}

pub fn part_2(input: &str) -> isize {
    iter_nats(input)
        .flatten()
        .tuple_windows()
        .find(|(prev, curr)| prev == curr)
        .unwrap()
        .0[1]
}

fn iter_nats(input: &str) -> impl Iterator<Item = Option<[isize; 2]>> {
    let program = parse_program(input);
    let queues: [_; 50] = array::from_fn(|i| {
        let queue = VecDeque::from([i as isize]);
        Rc::new(RefCell::new(queue))
    });
    let mut computers: [_; 50] = queues
        .iter()
        .map(|queue| {
            let queue = Rc::clone(queue);
            let inputs = iter::from_fn(move || queue.borrow_mut().pop_front());
            inputs.intcode(program.clone())
        })
        .collect_array()
        .unwrap();
    successors(Some(None), move |nat| {
        if let Some(inputs) = nat {
            let mut queue = queues[0].borrow_mut();
            queue.extend(inputs);
        }
        let mut nat = None;
        while computers.iter_mut().enumerate().any(|(i, computer)| {
            if queues[i].borrow().is_empty() {
                queues[i].borrow_mut().push_back(-1);
            }
            let Some(addr) = computer.next() else {
                return false;
            };
            let inputs = computer.next_chunk().unwrap();
            if addr == 255 {
                nat = Some(inputs);
            } else {
                let addr: usize = addr.try_into().unwrap();
                let mut queue = queues[addr].borrow_mut();
                queue.extend(inputs);
            }
            true
        }) {}
        Some(nat)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_23.txt");

    #[test_case(INPUT => 17740)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(INPUT => 12567)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
