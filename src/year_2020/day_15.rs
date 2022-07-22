fn nth_spoken_number(input: &str, n: usize) -> usize {
    let mut last_time_spoken: Vec<Option<usize>> = vec![None; n];
    let starting_numbers: Vec<usize> = input.split(',').map(|num| num.parse().unwrap()).collect();
    for (time, &num) in starting_numbers.iter().enumerate() {
        last_time_spoken[num] = Some(time + 1);
    }
    let mut previous_number = *starting_numbers.last().unwrap();
    for time in starting_numbers.len()..n {
        let current_number = last_time_spoken[previous_number];
        last_time_spoken[previous_number] = Some(time);
        previous_number = time - current_number.unwrap_or(time);
    }
    previous_number
}

pub fn part_1(input: &str) -> usize {
    nth_spoken_number(input, 2020)
}

pub fn part_2(input: &str) -> usize {
    nth_spoken_number(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_15.txt").unwrap();
    }

    #[test_case("1,3,2", 1)]
    #[test_case("2,1,3", 10)]
    #[test_case("1,2,3", 27)]
    #[test_case("2,3,1", 78)]
    #[test_case("3,2,1", 438)]
    #[test_case("3,1,2", 1836)]
    #[test_case(&INPUT, 1111)]
    fn part_1_examples(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case("0,3,6", 175594)]
    #[test_case("1,3,2", 2578)]
    #[test_case("2,1,3", 3544142)]
    #[test_case("1,2,3", 261214)]
    #[test_case("2,3,1", 6895259)]
    #[test_case("3,2,1", 18)]
    #[test_case("3,1,2", 362)]
    #[test_case(&INPUT, 48568)]
    fn part_2_examples(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected);
    }
}
