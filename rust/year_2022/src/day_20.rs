pub fn part_1(input: &str) -> i64 {
    decrypt_file(input, 1, 1)
}

pub fn part_2(input: &str) -> i64 {
    decrypt_file(input, 811_589_153, 10)
}

fn decrypt_file(input: &str, decryption_key: i64, n_mixes: usize) -> i64 {
    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * decryption_key)
        .collect();
    let mut indexes: Vec<_> = (0..numbers.len()).collect();
    for _ in 0..n_mixes {
        for (i_0, num) in numbers.iter().enumerate() {
            let i_1 = indexes.iter().position(|&i| i == i_0).unwrap();
            indexes.remove(i_1);
            let i_2 = (i_1 as i64 + num).rem_euclid(indexes.len() as i64) as usize;
            indexes.insert(i_2, i_0);
        }
    }
    let zero_i = indexes.iter().position(|&i| numbers[i] == 0).unwrap();
    (1000..=3000)
        .step_by(1000)
        .map(|i| numbers[indexes[(zero_i + i) % numbers.len()]])
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "1
2
-3
3
-2
0
4";

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_20.txt");

    #[test_case(EXAMPLE, 3)]
    #[test_case(INPUT, 5904)]
    fn part_1(input: &str, expected: i64) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 1623178306)]
    #[test_case(INPUT, 8332585833851)]
    fn part_2(input: &str, expected: i64) {
        assert_eq!(super::part_2(input), expected);
    }
}
