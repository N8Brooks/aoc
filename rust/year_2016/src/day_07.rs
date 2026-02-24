use itertools::{Either, Itertools as _};
use num::Integer as _;

pub fn part_1(input: &str) -> usize {
    parse_input(input)
        .filter(|(supernets, hypernets)| {
            supernets.iter().any(|net| {
                net.as_bytes()
                    .array_windows()
                    .any(|[a1, b1, b2, a2]| a1 != b1 && a1 == a2 && b1 == b2)
            }) && !hypernets.iter().any(|part| {
                part.as_bytes()
                    .array_windows()
                    .any(|[a1, b1, b2, a2]| a1 != b1 && a1 == a2 && b1 == b2)
            })
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .filter(|(supernets, hypernets)| {
            supernets.iter().any(|net| {
                net.as_bytes()
                    .array_windows()
                    .filter(|[a1, b, a2]| a1 != b && a1 == a2)
                    .any(|&[a, b, _]| {
                        let bab = [b, a, b];
                        hypernets
                            .iter()
                            .any(|part| part.as_bytes().array_windows().contains(&bab))
                    })
            })
        })
        .count()
}

fn parse_input(input: &str) -> impl Iterator<Item = (Vec<&str>, Vec<&str>)> {
    input.lines().map(|line| {
        line.split(['[', ']'])
            .enumerate()
            .partition_map(|(i, net)| {
                if i.is_even() {
                    Either::Left(net)
                } else {
                    Either::Right(net)
                }
            })
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    #[test_case("abba[mnop]qrst" => 1)]
    #[test_case("abcd[bddb]xyyx" => 0)]
    #[test_case("aaaa[qwer]tyui" => 0)]
    #[test_case("ioxxoj[asdfgh]zxcvbn" => 1)]
    #[test_case(INPUT => 115)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case("aba[bab]xyz" => 1)]
    #[test_case("xyx[xyx]xyx" => 0)]
    #[test_case("aaa[kek]eke" => 1)]
    #[test_case("zazbz[bzb]cdb" => 1)]
    #[test_case(INPUT => 231)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
