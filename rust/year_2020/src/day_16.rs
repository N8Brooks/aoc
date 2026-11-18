use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let (rules, _, nearby_tickets) = parse_input(input);
    let mut valid = [false; 1000];
    for range in rules.flat_map(|(_, ranges)| ranges) {
        valid[range].fill(true);
    }
    nearby_tickets.flatten().filter(|&n| !valid[n]).sum()
}

pub fn part_2(input: &str) -> usize {
    let (rules, your_ticket, nearby_tickets) = parse_input(input);

    let mut masks = [0; 1000];
    let mut departures = 0;
    for (index, (name, ranges)) in rules.enumerate() {
        let field = 1u32.checked_shl(index as u32).expect("at most 64 fields");
        if name.starts_with("departure ") {
            departures |= field;
        }
        for range in ranges {
            masks[range].iter_mut().for_each(|mask| *mask |= field);
        }
    }

    let your_ticket: Vec<_> = your_ticket.collect();
    let mut columns: Vec<_> = your_ticket.iter().map(|&n| masks[n]).collect();
    let mut row = Vec::with_capacity(columns.len());

    for ticket in nearby_tickets {
        row.clear();
        row.extend(ticket.map(|n| masks[n]));
        if !row.contains(&0) {
            for (column, &fields) in columns.iter_mut().zip(&row) {
                *column &= fields;
            }
        }
    }

    let mut assigned = 0;
    (0..columns.len())
        .filter_map(|_| {
            let (column, field) = columns
                .iter()
                .map(|mask| mask & !assigned)
                .enumerate()
                .find(|&(_, mask)| mask.count_ones() == 1)
                .expect("ticket fields must have a unique assignment");
            assigned |= field;
            (field & departures != 0).then(|| your_ticket[column])
        })
        .product()
}

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = (&str, [RangeInclusive<usize>; 2])>,
    impl Iterator<Item = usize>,
    impl Iterator<Item = impl Iterator<Item = usize>>,
) {
    let [rules, your_ticket, nearby_tickets] = input.split("\n\n").collect_array().unwrap();

    let rules = rules.lines().map(|line| {
        let (name, ranges) = line.split_once(": ").unwrap();
        let (first, second) = ranges.split_once(" or ").unwrap();
        let ranges = [first, second].map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        });
        (name, ranges)
    });

    let your_ticket = parse_ticket(your_ticket.lines().nth(1).unwrap());
    let nearby_tickets = nearby_tickets.lines().skip(1).map(parse_ticket);
    (rules, your_ticket, nearby_tickets)
}

fn parse_ticket(ticket: &str) -> impl Iterator<Item = usize> {
    ticket.split(',').map(|n| n.parse().unwrap())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    const EXAMPLE_1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test_case(EXAMPLE_1 => 71)]
    #[test_case(INPUT => 32842)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "row: 0-5 or 8-19
departure class: 0-1 or 4-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
18,99,18";

    #[test_case(EXAMPLE_2 => 156)]
    #[test_case(INPUT => 2628667251989)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
