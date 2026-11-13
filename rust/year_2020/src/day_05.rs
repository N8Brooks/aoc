use std::ops::BitXor;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> u16 {
    input.lines().map(parse_seat_id).max().unwrap()
}

pub fn part_2(input: &str) -> u16 {
    let seat_ids = input.lines().map(parse_seat_id);
    let (min, max) = seat_ids.clone().minmax().into_option().unwrap();
    let xor = seat_ids.fold(0, BitXor::bitxor);
    (min..=max).fold(xor, BitXor::bitxor)
}

fn parse_seat_id(seat: &str) -> u16 {
    seat.bytes()
        .fold(0, |acc, b| (acc << 1) | (b == b'B' || b == b'R') as u16)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    #[test_case("FBFBBFFRLR" => 357)]
    #[test_case("BFFFBBFRRR" => 567)]
    #[test_case("FFFBBBFRRR" => 119)]
    #[test_case("BBFFBBFRLL" => 820)]
    fn parse_seat_id_test(seat: &str) -> u16 {
        super::parse_seat_id(seat)
    }

    #[test_case(INPUT => 953)]
    fn part_1(input: &str) -> u16 {
        super::part_1(input)
    }

    #[test_case(INPUT => 615)]
    fn part_2(input: &str) -> u16 {
        super::part_2(input)
    }
}
