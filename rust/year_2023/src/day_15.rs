use std::array;

pub fn part_1(input: &str) -> usize {
    input.trim_end().split(',').map(hash).sum()
}

pub fn part_2(input: &str) -> usize {
    let mut hash_map = HashMap::new();
    for operation in input.trim_end().split(',') {
        if let Some(label) = operation.strip_suffix('-') {
            hash_map.remove(label);
        } else if let Some((label, focal_length)) = operation.split_once('=') {
            hash_map.insert(label, focal_length.parse().unwrap());
        } else {
            panic!("Invalid operation: {}", operation);
        }
    }
    return hash_map.focussing_power();
}

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0u8, |value, code| value.wrapping_add(code).wrapping_mul(17))
        .into()
}

#[derive(Debug)]
struct HashMap<'a> {
    buckets: [Vec<(&'a str, usize)>; 256],
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        Self {
            buckets: array::from_fn(|_| Vec::new()),
        }
    }

    fn insert(&mut self, key: &'a str, value: usize) {
        let i = hash(key);
        let bucket = &mut self.buckets[i];
        if let Some(j) = bucket.iter().position(|&(k, _)| k == key) {
            bucket[j] = (key, value);
        } else {
            bucket.push((key, value));
        }
    }

    fn remove(&mut self, key: &'a str) {
        let i = hash(key);
        let bucket = &mut self.buckets[i];
        if let Some(j) = bucket.iter().position(|&(k, _)| k == key) {
            bucket.remove(j);
        }
    }

    fn focussing_power(&self) -> usize {
        (1..)
            .zip(&self.buckets)
            .flat_map(|(i, bucket)| {
                (1..)
                    .zip(bucket)
                    .map(move |(j, (_, focal_length))| i * j * focal_length)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    const INPUT: &str = include_str!("../../../testdata/year_2023/day_15.txt");

    #[test_case(EXAMPLE_1, 1320)]
    #[test_case(INPUT, 517315)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE_1, 145)]
    #[test_case(INPUT, 247763)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }

    #[test_case("HASH", 52)]
    fn hash(input: &str, expected: usize) {
        assert_eq!(super::hash(input), expected);
    }
}
