use itertools::Itertools;
use num::Integer;

pub fn part_1(input: &str) -> String {
    let n: usize = input.parse().unwrap();
    let recipes = gen_recipes_while(|recipes| recipes.len() < n + 10);
    format!("{:0>10}", recipes[recipes.len() - 10..].iter().join(""))
}

pub fn part_2(input: &str) -> usize {
    let digits = input.bytes().map(|byte| byte - b'0').collect_vec();
    let recipes = gen_recipes_while(|recipes| !recipes.ends_with(&digits));
    recipes.len() - digits.len()
}

fn gen_recipes_while(mut predicate: impl FnMut(&[u8]) -> bool) -> Vec<u8> {
    let (mut i, mut j) = (0, 1);
    let mut recipes = vec![3u8, 7u8];
    while predicate(&recipes) {
        let (a, b) = (recipes[i], recipes[j]);
        match (a + b).div_rem(&10) {
            (0, r) => recipes.push(r),
            (q @ 1..=9, r) => {
                recipes.push(q);
                if !predicate(&recipes) {
                    return recipes;
                }
                recipes.push(r);
            }
            _ => panic!("multi-digit quotient"),
        };
        i = (i + 1 + a as usize) % recipes.len();
        j = (j + 1 + b as usize) % recipes.len();
    }
    recipes
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    #[test_case("9" => "5158916779")]
    #[test_case("5" => "0124515891")]
    #[test_case("18" => "9251071085")]
    #[test_case("2018" => "5941429882")]
    #[test_case(INPUT => "6289129761")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case("51589" => 9)]
    #[test_case("01245" => 5)]
    #[test_case("92510" => 18)]
    #[test_case("59414" => 2018)]
    #[test_case(INPUT => 20207075)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
