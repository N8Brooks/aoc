use std::collections::VecDeque;

pub fn part_1(input: &str) -> usize {
    let bytes = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let (max_i, max_j) = (bytes.len(), bytes[0].len());
    let mut cummulative_steps = vec![vec![None::<usize>; max_j]; max_i];
    let (start_i, start_j) = get_byte_position(&bytes, b'S');
    let mut queue = VecDeque::from([(start_i, start_j)]);
    cummulative_steps[start_i][start_j] = Some(0);
    let end = get_byte_position(&bytes, b'E');
    while let Some((i, j)) = queue.pop_front() {
        let steps = cummulative_steps[i][j].expect("some steps");
        if (i, j) == end {
            return steps;
        }
        let height = get_height(bytes[i][j]);
        let steps = Some(steps + 1);
        i.checked_sub(1)
            .iter()
            .map(|&i| (i, j))
            .chain(j.checked_sub(1).iter().map(|&j| (i, j)))
            .chain((i + 1 < max_i).then_some((i + 1, j)))
            .chain((j + 1 < max_j).then_some((i, j + 1)))
            .for_each(|(i, j)| {
                if cummulative_steps[i][j].is_some() {
                    return;
                }
                if get_height(bytes[i][j]) > height + 1 {
                    return;
                }
                cummulative_steps[i][j] = steps;
                queue.push_back((i, j));
            });
    }
    unreachable!()
}

pub fn part_2(input: &str) -> usize {
    let bytes = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let (max_i, max_j) = (bytes.len(), bytes[0].len());
    let mut cummulative_steps = vec![vec![None::<usize>; max_j]; max_i];
    let (start_i, start_j) = get_byte_position(&bytes, b'E');
    let mut queue = VecDeque::from([(start_i, start_j)]);
    cummulative_steps[start_i][start_j] = Some(0);
    while let Some((i, j)) = queue.pop_front() {
        let steps = cummulative_steps[i][j].expect("some steps");
        let height = get_height(bytes[i][j]);
        if height == b'a' {
            return steps;
        }
        let steps = Some(steps + 1);
        i.checked_sub(1)
            .iter()
            .map(|&i| (i, j))
            .chain(j.checked_sub(1).iter().map(|&j| (i, j)))
            .chain((i + 1 < max_i).then_some((i + 1, j)))
            .chain((j + 1 < max_j).then_some((i, j + 1)))
            .for_each(|(i, j)| {
                if cummulative_steps[i][j].is_some() {
                    return;
                }
                if height - 1 > get_height(bytes[i][j]) {
                    return;
                }
                cummulative_steps[i][j] = steps;
                queue.push_back((i, j));
            });
    }
    unreachable!()
}

fn get_height(byte: u8) -> u8 {
    match byte {
        b'a'..=b'z' => byte,
        b'S' => b'a',
        b'E' => b'z',
        _ => panic!("No height for {byte}"),
    }
}

fn get_byte_position(bytes: &[&[u8]], byte: u8) -> (usize, usize) {
    bytes
        .iter()
        .enumerate()
        .find_map(|(i, bytes)| {
            bytes
                .iter()
                .enumerate()
                .find_map(|(j, &x)| if x == byte { Some((i, j)) } else { None })
        })
        .unwrap_or_else(|| panic!("No position of {byte}"))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_12.txt");

    #[test]
    fn get_height_c() {
        assert_eq!(get_height(b'c'), b'c');
    }

    #[test]
    fn get_height_start() {
        assert_eq!(get_height(b'S'), b'a');
    }

    #[test]
    fn get_height_end() {
        assert_eq!(get_height(b'E'), b'z');
    }

    #[test]
    fn get_start_position() {
        let bytes = EXAMPLE
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<_>>();
        assert_eq!(get_byte_position(&bytes, b'S'), (0, 0));
    }

    #[test]
    fn get_end_position() {
        let bytes = EXAMPLE
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<_>>();
        assert_eq!(get_byte_position(&bytes, b'E'), (2, 5));
    }

    #[test_case(EXAMPLE, 31)]
    #[test_case(INPUT, 462)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 29)]
    #[test_case(INPUT, 451)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
