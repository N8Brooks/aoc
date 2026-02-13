use std::iter;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> u32 {
    let ingredients = parse_ingredients(input);
    allocations(100, ingredients.len())
        .map(|allocation| {
            ingredients
                .iter()
                .zip(allocation)
                .fold([0; 4], |mut props, (ingredient, amount)| {
                    for (prop, &value) in props.iter_mut().zip(ingredient) {
                        *prop += value * amount as i32;
                    }
                    props
                })
                .into_iter()
                .map(|prop| u32::try_from(prop).unwrap_or(0))
                .product()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> u32 {
    let ingredients = parse_ingredients(input);
    allocations(100, ingredients.len())
        .map(|allocation| {
            ingredients
                .iter()
                .zip(allocation)
                .fold([0; 5], |mut props, (ingredient, amount)| {
                    for (prop, &value) in props.iter_mut().zip(ingredient) {
                        *prop += value * amount as i32;
                    }
                    props
                })
        })
        .filter(|[.., calories]| *calories == 500)
        .map(|[props @ .., _]| {
            props
                .into_iter()
                .map(|prop| u32::try_from(prop).unwrap_or(0))
                .product()
        })
        .max()
        .unwrap()
}

fn allocations(r: u32, n: usize) -> impl Iterator<Item = [u32; 4]> {
    assert!(r > 0, "must allocate at least one unit");
    assert!(n > 1, "must allocate to at least two n");
    assert!(n <= 4, "hardcoded to support up to four n");
    let mut indexes = [0; 4];
    indexes[n - 1] = r;
    let mut i = n - 2;
    iter::from_fn(move || {
        if indexes[i + 1] > 0 {
            indexes[i] += 1;
            indexes[i + 1] -= 1;
        } else if i == 0 {
            return None;
        } else {
            indexes[n - 1] += indexes[i..n - 1].iter().sum::<u32>();
            indexes[i..n - 1].fill(0);
            indexes[n - 1] -= 1;
            indexes[i - 1] += 1;
            i = if indexes[n - 1] > 0 { n - 2 } else { i - 1 };
        }
        Some(indexes)
    })
}

fn parse_ingredients(input: &str) -> Vec<[i32; 5]> {
    input
        .lines()
        .map(|line| {
            let (_, props) = line.split_once(": ").unwrap();
            props
                .split(", ")
                .map(|property| {
                    let (_, value) = property.split_once(' ').unwrap();
                    value.parse().unwrap()
                })
                .collect_array()
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    const EXAMPLE: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test_case(INPUT => 18965440)]
    #[test_case(EXAMPLE => 62842880)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 57600000)]
    #[test_case(INPUT => 15862900)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
