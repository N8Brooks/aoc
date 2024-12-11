use hashbrown::HashMap;
use num::Integer as _;

pub fn part_1(input: &str) -> usize {
    count_stones(input, 25)
}

pub fn part_2(input: &str) -> usize {
    count_stones(input, 75)
}

fn count_stones(input: &str, n: u32) -> usize {
    let mut knapsack_1 = input
        .trim()
        .split(' ')
        .fold(HashMap::new(), |mut knapsack, stone| {
            let stone = stone.parse::<u64>().unwrap();
            *knapsack.entry(stone).or_insert(0) += 1;
            knapsack
        });
    let mut knapsack_2 = HashMap::new();

    for _ in 0..n {
        knapsack_2 = knapsack_1
            .drain()
            .flat_map(|(stone, count)| {
                if stone == 0 {
                    return vec![(1, count)];
                }
                let n = stone.ilog10() + 1;
                if n & 1 == 0 {
                    let d = 10u64.pow(n / 2);
                    let (l, r) = stone.div_mod_floor(&d);
                    vec![(l, count), (r, count)]
                } else {
                    vec![(stone * 2024, count)]
                }
            })
            .fold(knapsack_2, |mut knapsack, (stone, count)| {
                *knapsack.entry(stone).or_insert(0) += count;
                knapsack
            });
        std::mem::swap(&mut knapsack_1, &mut knapsack_2);
    }

    knapsack_1.values().sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    const EXAMPLE: &str = "125 17";

    #[test_case(EXAMPLE, 55312)]
    #[test_case(INPUT, 217443)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 65601038650482)]
    #[test_case(INPUT, 257246536026785)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
