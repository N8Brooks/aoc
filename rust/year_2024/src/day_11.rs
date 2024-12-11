use std::sync::{Arc, LazyLock, RwLock};

use hashbrown::HashMap;
use num::{BigUint, FromPrimitive, Integer as _};

pub fn part_1(input: &str) -> u64 {
    num_stones(input, 25)
}

pub fn part_2(input: &str) -> u64 {
    num_stones(input, 75)
}

fn num_stones(input: &str, n: u32) -> u64 {
    input
        .trim()
        .split(' ')
        .map(|stone| stone.parse::<u64>().unwrap())
        .map(|stone| num_stones_helper(stone, n))
        .sum()
}

fn num_stones_helper(stone: u64, left: u32) -> u64 {
    static CACHE: LazyLock<Arc<RwLock<HashMap<(u64, u32), u64>>>> =
        LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

    if left == 0 {
        return 1;
    }
    if stone == 0 {
        return num_stones_helper(1, left - 1);
    }

    if let Some(&x) = CACHE.read().unwrap().get(&(stone, left)) {
        return x;
    }

    let n = stone.ilog10() + 1;
    let x = if n & 1 == 0 {
        let (x, y) = stone.div_mod_floor(&10u64.pow(n / 2));
        num_stones_helper(x, left - 1) + num_stones_helper(y, left - 1)
    } else {
        num_stones_helper(stone * 2024, left - 1)
    };
    CACHE.write().unwrap().insert((stone, left), x);
    x
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    const EXAMPLE: &str = "125 17";

    #[test_case(EXAMPLE, 55312)]
    #[test_case(INPUT, 217443)]
    fn part_1(input: &str, expected: u64) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 65601038650482)]
    #[test_case(INPUT, 257246536026785)]
    fn part_2(input: &str, expected: u64) {
        assert_eq!(super::part_2(input), expected);
    }
}
