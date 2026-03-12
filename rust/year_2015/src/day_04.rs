use util::hash::Md5Hasher;

pub fn part_1(input: &str) -> usize {
    filter(input, 5)
}

pub fn part_2(input: &str) -> usize {
    filter(input, 6)
}

fn filter(input: &str, k: u32) -> usize {
    let prefix = input.as_bytes();
    let mut num_buf = [0u8; 20];
    let mut base_hasher = Md5Hasher::default();
    base_hasher.write(prefix);

    (0..)
        .find(|&i| {
            let digits = write_decimal(i, &mut num_buf);
            let mut hasher = base_hasher.clone();
            hasher.write(digits);
            hasher.finish_u128().to_be().leading_zeros() >= k * 4
        })
        .unwrap_or_else(|| unreachable!())
}

fn write_decimal(mut num: usize, buf: &mut [u8; 20]) -> &[u8] {
    if num == 0 {
        buf[19] = b'0';
        return &buf[19..20];
    }
    let mut idx = 20;
    while num > 0 {
        idx -= 1;
        buf[idx] = (num % 10) as u8 + b'0';
        num /= 10;
    }
    &buf[idx..20]
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    #[test_case("abcdef" => 609043)]
    #[test_case("pqrstuv" => 1048970)]
    #[test_case(INPUT => 346386)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 9958218)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
