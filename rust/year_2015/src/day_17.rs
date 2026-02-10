pub fn part_1(input: &str, target: usize) -> usize {
    let mut dp = vec![0; target + 1];
    dp[0] = 1;
    for size in parse_containers(input) {
        let stop = (target + 1).saturating_sub(size);
        for j in (0..stop).rev() {
            dp[j + size] += dp[j];
        }
    }
    dp[target]
}

pub fn part_2(input: &str, target: usize) -> usize {
    let mut dp = vec![vec![0; target + 1]];
    dp[0][0] = 1;
    for size in parse_containers(input) {
        if dp.iter().all(|row| row[target] == 0) {
            dp.push(vec![0; target + 1]);
        }
        let stop = (target + 1).saturating_sub(size);
        for j in (0..stop).rev() {
            for i in 0..dp.len() - 1 {
                dp[i + 1][j + size] += dp[i][j];
            }
        }
    }
    dp.iter()
        .map(|row| row[target])
        .find(|&count| count > 0)
        .unwrap_or(0)
}

fn parse_containers(input: &str) -> impl Iterator<Item = usize> {
    input.lines().map(|line| line.parse().unwrap())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_17.txt");

    const EXAMPLE: &str = "20
15
10
5
5";

    #[test_case(EXAMPLE, 25 => 4)]
    #[test_case(INPUT, 150 => 654)]
    fn part_1(input: &str, target: usize) -> usize {
        super::part_1(input, target)
    }

    #[test_case(EXAMPLE, 25 => 3)]
    #[test_case(INPUT, 150 => 57)]
    fn part_2(input: &str, target: usize) -> usize {
        super::part_2(input, target)
    }
}
