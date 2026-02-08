use std::iter;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> u32 {
    let ingredients = parse_ingredients(input);
    allocations(100, ingredients.len())
        .filter_map(|allocation| {
            let properties = ingredients.iter().zip(allocation).fold(
                [0; 4],
                |mut properties, (ingredient, amount)| {
                    for (prop, &value) in properties.iter_mut().zip(ingredient) {
                        *prop += value * amount as i32;
                    }
                    properties
                },
            );
            properties
                .into_iter()
                .map(|prop| u32::try_from(prop).ok())
                .product()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> u32 {
    let ingredients = parse_ingredients(input);
    allocations(100, ingredients.len())
        .filter_map(|allocation| {
            let properties = ingredients.iter().zip(allocation).fold(
                [0; 5],
                |mut properties, (ingredient, amount)| {
                    for (prop, &value) in properties.iter_mut().zip(ingredient) {
                        *prop += value * amount as i32;
                    }
                    properties
                },
            );
            if properties[4] != 500 {
                return None;
            }
            properties[..4]
                .iter()
                .map(|&prop| u32::try_from(prop).ok())
                .product()
        })
        .max()
        .unwrap()
}

fn allocations(r: u32, n: usize) -> impl Iterator<Item = Vec<u32>> {
    debug_assert!(n > 1);
    let mut indexes = vec![0; n];
    indexes[n - 1] = r;
    let mut state = None;
    iter::from_fn(move || {
        if let Some(i) = state {
            if indexes[i + 1] > 0 {
                indexes[i] += 1;
                indexes[i + 1] -= 1;
                return Some(indexes.clone());
            } else {
                if i == 0 {
                    return None;
                }
                for j in i..n - 1 {
                    indexes[n - 1] += indexes[j];
                    indexes[j] = 0;
                }
                indexes[i - 1] += 1;
                indexes[n - 1] -= 1;
            }
        }
        state = indexes.array_windows().rposition(|[a, b]| a != b);
        Some(indexes.clone())
    })
}

fn parse_ingredients(input: &str) -> Vec<[i32; 5]> {
    input
        .lines()
        .map(|line| {
            let (_, properties) = line.split_once(": ").unwrap();
            properties
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
