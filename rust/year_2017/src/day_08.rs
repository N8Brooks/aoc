use hashbrown::HashMap;
use itertools::Itertools;

pub fn part_1(input: &str) -> isize {
    let mut registers = HashMap::new();

    for (target, delta, condition_register, comparison, threshold) in parse_instructions(input) {
        let current = *registers.entry(condition_register).or_default();
        if !match comparison {
            ">" => current > threshold,
            "<" => current < threshold,
            ">=" => current >= threshold,
            "<=" => current <= threshold,
            "==" => current == threshold,
            "!=" => current != threshold,
            _ => panic!("Unknown comparison: {comparison}"),
        } {
            continue;
        }

        *registers.entry(target).or_default() += delta;
    }

    registers.into_values().max().unwrap_or(0)
}

pub fn part_2(input: &str) -> isize {
    let mut registers = HashMap::new();

    parse_instructions(input)
        .filter_map(
            |(target, delta, condition_register, comparison, threshold)| {
                let current = registers.get(condition_register).copied().unwrap_or(0);
                if !match comparison {
                    ">" => current > threshold,
                    "<" => current < threshold,
                    ">=" => current >= threshold,
                    "<=" => current <= threshold,
                    "==" => current == threshold,
                    "!=" => current != threshold,
                    _ => panic!("Unknown comparison: {comparison}"),
                } {
                    return None;
                }

                let value = registers.entry(target).or_default();
                *value += delta;
                Some(*value)
            },
        )
        .fold(0, isize::max)
}

fn parse_instructions(input: &str) -> impl Iterator<Item = (&str, isize, &str, &str, isize)> {
    input.lines().map(|line| {
        let [target, operation, amount, _, cond_reg, comp, threshold] =
            line.split(' ').collect_array().unwrap();
        let amount: isize = amount.parse().unwrap();
        let threshold = threshold.parse().unwrap();
        let delta = match operation {
            "inc" => amount,
            "dec" => -amount,
            _ => panic!("Unknown operation: {}", operation),
        };
        (target, delta, cond_reg, comp, threshold)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    const EXAMPLE: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test_case(EXAMPLE => 1)]
    #[test_case(INPUT => 3880)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 10)]
    #[test_case(INPUT => 5035)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
