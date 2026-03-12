use std::{collections::VecDeque, iter};

use util::hash::Md5Hasher;

pub fn part_1(input: &str) -> Option<String> {
    let mut hasher = Md5Hasher::default();
    hasher.write(input.as_bytes());
    let mut queue = VecDeque::from([(hasher, 0u8, 0u8, String::new())]);
    while let Some((hasher, i, j, path)) = queue.pop_front() {
        if (i, j) == (3, 3) {
            return Some(path);
        }
        [
            i.checked_sub(1).map(|i| (i, j, b'U')),
            (i < 3).then(|| (i + 1, j, b'D')),
            j.checked_sub(1).map(|j| (i, j, b'L')),
            (j < 3).then(|| (i, j + 1, b'R')),
        ]
        .into_iter()
        .zip(hasher.finish_hex())
        .filter_map(|(pos, b)| pos.filter(|_| (b'b'..=b'f').contains(&b)))
        .map(|(i, j, b)| {
            let mut hasher = hasher.clone();
            hasher.write_u8(b);
            let mut path = path.clone();
            path.push(b as char);
            (hasher, i, j, path)
        })
        .collect_into(&mut queue);
    }
    None
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut hasher = Md5Hasher::default();
    hasher.write(input.as_bytes());
    let mut stack = vec![(hasher, 0u8, 0u8, 0)];
    iter::from_fn(|| {
        while let Some((hasher, i, j, len)) = stack.pop() {
            if (i, j) == (3, 3) {
                return Some(len);
            }
            [
                i.checked_sub(1).map(|i| (i, j, b'U')),
                (i < 3).then(|| (i + 1, j, b'D')),
                j.checked_sub(1).map(|j| (i, j, b'L')),
                (j < 3).then(|| (i, j + 1, b'R')),
            ]
            .into_iter()
            .zip(hasher.finish_hex())
            .filter_map(|(pos, b)| pos.filter(|_| (b'b'..=b'f').contains(&b)))
            .map(|(i, j, b)| {
                let mut hasher = hasher.clone();
                hasher.write_u8(b);
                (hasher, i, j, len + 1)
            })
            .collect_into(&mut stack);
        }
        None
    })
    .max()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_17.txt");

    #[test_case("ihgpwlah" => Some("DDRRRD".into()))]
    #[test_case("kglvqrro" => Some("DDUDRLRRUDRD".into()))]
    #[test_case("ulqzkmiv" => Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".into()))]
    #[test_case(INPUT => Some("DDURRLRRDD".into()))]
    fn part_1(input: &str) -> Option<String> {
        super::part_1(input)
    }

    #[test_case("ihgpwlah" => Some(370))]
    #[test_case("kglvqrro" => Some(492))]
    #[test_case("ulqzkmiv" => Some(830))]
    #[test_case(INPUT => Some(436))]
    fn part_2(input: &str) -> Option<usize> {
        super::part_2(input)
    }
}
