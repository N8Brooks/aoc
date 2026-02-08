pub fn part_1(input: &str) -> i32 {
    let n = num_people(input);
    let diffs = parse_diffs(input, n);
    max_happiness(&diffs)
}

pub fn part_2(input: &str) -> i32 {
    let n = num_people(input) + 1;
    let diffs = parse_diffs(input, n);
    max_happiness(&diffs)
}

fn num_people(input: &str) -> usize {
    let m = input.lines().count();
    (4 * m + 1).isqrt().div_ceil(2)
}

fn parse_diffs(input: &str, n: usize) -> Vec<Vec<i32>> {
    let mut people = Vec::with_capacity(n);
    let mut happiness = vec![vec![0; n]; n];
    for line in input.lines() {
        let (left, rest) = line.split_once(" would ").unwrap();
        let (points, rest) = rest
            .split_once(" happiness units by sitting next to ")
            .unwrap();
        let right = rest.strip_suffix('.').unwrap();
        let i = people.iter().position(|&c| c == left).unwrap_or_else(|| {
            people.push(left);
            people.len() - 1
        });
        let j = people.iter().position(|&c| c == right).unwrap_or_else(|| {
            people.push(right);
            people.len() - 1
        });
        let points = if let Some(points) = points.strip_prefix("gain ") {
            points.parse().unwrap()
        } else if let Some(points) = points.strip_prefix("lose ") {
            -points.parse::<i32>().unwrap()
        } else {
            panic!("invalid points: {points}");
        };
        happiness[i][j] += points;
        happiness[j][i] += points;
    }
    happiness
}

fn max_happiness(diffs: &[Vec<i32>]) -> i32 {
    fn tsp(dp: &mut [Vec<Option<i32>>], diffs: &[Vec<i32>], mask: usize, pos: usize) -> i32 {
        if mask == (1 << diffs.len()) - 1 {
            return diffs[pos][0];
        }
        if let Some(res) = dp[mask][pos] {
            return res;
        }
        let res = diffs[pos]
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) == 0)
            .map(|(i, &dist)| dist + tsp(dp, diffs, mask | (1 << i), i))
            .max()
            .unwrap();
        dp[mask][pos] = Some(res);
        res
    }

    let n = diffs.len();
    let mut dp = vec![vec![None; n]; 1 << n];
    tsp(&mut dp, diffs, 1, 0)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    const EXAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test_case(EXAMPLE => 330)]
    #[test_case(INPUT => 709)]
    fn part_1(input: &str) -> i32 {
        super::part_1(input)
    }

    #[test_case(INPUT => 668)]
    fn part_2(input: &str) -> i32 {
        super::part_2(input)
    }
}
