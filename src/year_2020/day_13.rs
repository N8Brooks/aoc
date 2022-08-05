use crate::util::crt;

pub fn part_1(input: &str) -> i32 {
    let (t0, schedule) = input.split_once('\n').unwrap();
    let t0: i32 = t0.parse().unwrap();
    let id: i32 = schedule
        .split(',')
        .flat_map(|id| id.parse())
        .min_by_key(|id| t0 + id - t0 % id)
        .unwrap();
    id * (id - t0 % id)
}

pub fn part_2(input: &str) -> i64 {
    let (_, schedule) = input.split_once('\n').unwrap();
    let (rs, ms): (Vec<_>, Vec<_>) = schedule
        .split(',')
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(i, id)| {
            let id: i64 = id.parse().unwrap();
            (id - i as i64, id)
        })
        .unzip();
    crt(&rs, &ms).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_13.txt").unwrap();
    }

    #[test_case(EXAMPLE, 295)]
    #[test_case(&INPUT, 4722)]
    fn part_1_tests(input: &str, expected: i32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 1068781)]
    #[test_case("-1\n17,x,13,19", 3417)]
    #[test_case("-1\n67,7,59,61", 754018)]
    #[test_case("-1\n67,x,7,59,61", 779210)]
    #[test_case("-1\n67,7,x,59,61", 1261476)]
    #[test_case("-1\n1789,37,47,1889", 1202161486)]
    #[test_case(&INPUT, 825305207525452)]
    fn part_2_tests(input: &str, expected: i64) {
        assert_eq!(part_2(input), expected);
    }
}
