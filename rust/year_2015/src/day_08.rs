pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let num_bytes = line.len();
            let mut it = line.strip_circumfix('"', '"').unwrap().bytes();
            let mut mem_bytes = 0;
            while let Some(b) = it.next() {
                if b == b'\\' && it.next().unwrap() == b'x' {
                    it.next_chunk::<2>().expect("invalid hex escape");
                }
                mem_bytes += 1;
            }
            num_bytes - mem_bytes
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let enc_bytes: usize = line
                .bytes()
                .map(|b| match b {
                    b'"' | b'\\' => 2,
                    _ => 1,
                })
                .sum();
            let num_bytes = line.len();
            enc_bytes + 2 - num_bytes
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    const EXAMPLE: &str = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";

    #[test_case(EXAMPLE => 12)]
    #[test_case(INPUT => 1333)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 19)]
    #[test_case(INPUT => 2046)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
