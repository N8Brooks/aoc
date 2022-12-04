use std::str;

pub fn part_1(input: &str) -> usize {
    let bit_length = input.find('\n').unwrap();
    let mut bit_counts = vec![0; bit_length];
    for line in input.lines() {
        for (i, bit) in line.bytes().enumerate() {
            let bit = bit - b'0';
            bit_counts[i] += bit as usize;
        }
    }

    let n_bin_nums = (input.len() + 1) / (bit_length + 1);
    let mode_thresh = n_bin_nums / 2;
    let gamma = bit_counts.iter().fold(0, |gamma, &bit_count| {
        let mode_bit = bit_count > mode_thresh;
        gamma << 1 | mode_bit as usize
    });
    let epsilon = ((1 << bit_length) - 1) ^ gamma;

    gamma * epsilon
}

pub fn part_2(input: &str) -> usize {
    let mut i = 0;
    let mut bin_nums: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    while bin_nums.len() > 1 {
        let bit_count = bin_nums.iter().filter(|bin_num| bin_num[i] == b'1').count();
        let mode = b'0' + (2 * bit_count >= bin_nums.len()) as u8;
        bin_nums.retain_mut(|bin_num| bin_num[i] == mode);
        i += 1;
    }
    let oxygen = str::from_utf8(bin_nums[0]).unwrap();
    let oxygen = usize::from_str_radix(oxygen, 2).unwrap();

    let mut i = 0;
    let mut bin_nums: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    while bin_nums.len() > 1 {
        let bit_count = bin_nums.iter().filter(|bin_num| bin_num[i] == b'1').count();
        let mode = b'0' + (2 * bit_count < bin_nums.len()) as u8;
        bin_nums.retain_mut(|bin_num| bin_num[i] == mode);
        i += 1;
    }
    let co2 = str::from_utf8(bin_nums[0]).unwrap();
    let co2 = usize::from_str_radix(co2, 2).unwrap();

    oxygen * co2
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    const INPUT: &str = include_str!("../../../testdata/year_2021/day_03.txt");

    #[test_case(EXAMPLE, 198)]
    #[test_case(INPUT, 3242606)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 230)]
    #[test_case(INPUT, 4856080)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
