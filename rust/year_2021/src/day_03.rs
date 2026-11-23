pub fn part_1(input: &str) -> usize {
    let nums: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let (n, m) = (nums.len(), nums[0].len());

    let gamma = (0..m).fold(0, |gamma, i| {
        (gamma << 1) | usize::from(2 * nums.iter().filter(|bytes| bytes[i] == b'1').count() > n)
    });

    let mask = (1 << m) - 1;
    let epsilon = mask ^ gamma;

    gamma * epsilon
}

pub fn part_2(input: &str) -> usize {
    rating(input, true) * rating(input, false)
}

fn rating(input: &str, most_common: bool) -> usize {
    let mut nums: Vec<_> = input.lines().collect();
    let m = nums[0].len();

    for i in 0..m {
        if nums.len() == 1 {
            break;
        }

        let ones = nums.iter().filter(|num| num.as_bytes()[i] == b'1').count();

        let keep_one = (2 * ones >= nums.len()) == most_common;
        let bit = b'0' + u8::from(keep_one);

        nums.retain(|num| num.as_bytes()[i] == bit);
    }

    usize::from_str_radix(nums[0], 2).unwrap()
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

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    #[test_case(EXAMPLE => 198)]
    #[test_case(INPUT => 3242606)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 230)]
    #[test_case(INPUT => 4856080)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
