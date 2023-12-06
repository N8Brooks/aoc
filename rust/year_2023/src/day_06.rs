pub fn part_1(input: &str) -> isize {
    let (times, distances) = input.trim_end().split_once("\n").unwrap();
    let times = times
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap());
    let distances = distances
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap());
    times
        .zip(distances)
        .map(|(t, d)| count_ways_to_beat(t, d))
        .product()
}

pub fn part_2(input: &str) -> isize {
    let (time, distance) = input.trim_end().split_once("\n").unwrap();
    let time = time
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse::<isize>()
        .unwrap();
    let distance = distance
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse::<isize>()
        .unwrap();
    count_ways_to_beat(time, distance)
}

fn count_ways_to_beat(time: isize, distance: isize) -> isize {
    let sqrt_discriminant = (time * time - 4 * distance - 1).isqrt() + 1;
    let i = (-time + sqrt_discriminant) / -2 + 1;
    let j = (-time - sqrt_discriminant).div_ceil(-2);
    return j - i;
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../testdata/year_2023/day_06.txt");

    const EXAMPLE_1: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test_case(EXAMPLE_1, 288)]
    #[test_case(INPUT, 625968)]
    fn part_1(input: &str, expected: isize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE_1, 71503)]
    #[test_case(INPUT, 43663323)]
    fn part_2(input: &str, expected: isize) {
        assert_eq!(super::part_2(input), expected);
    }
}
