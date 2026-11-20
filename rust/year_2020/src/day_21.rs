use hashbrown::{DefaultHashBuilder, HashSet};
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let foods = parse_foods(input);
    let allergenic: HashSet<_> = allergenic_ingredients(&foods)
        .into_values()
        .flatten()
        .collect();

    foods
        .into_iter()
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|&ingredient| !allergenic.contains(ingredient))
        .count()
}

pub fn part_2(input: &str) -> String {
    let foods = parse_foods(input);
    let mut possible: Vec<_> = allergenic_ingredients(&foods).into_iter().collect();

    (0..possible.len())
        .map(|_| {
            let i = possible
                .iter()
                .position(|(_, ingredients)| ingredients.len() == 1)
                .unwrap();

            let (allergen, ingredients) = possible.swap_remove(i);
            let ingredient = ingredients.into_iter().next().unwrap();

            for (_, ingredients) in &mut possible {
                ingredients.remove(ingredient);
            }

            (allergen, ingredient)
        })
        .sorted_unstable_by_key(|&(allergen, _)| allergen)
        .map(|(_, ingredient)| ingredient)
        .join(",")
}

fn parse_foods(input: &str) -> Vec<(HashSet<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line.split_once(" (contains ").unwrap();
            (
                ingredients.split(' ').collect(),
                allergens.strip_suffix(')').unwrap().split(", ").collect(),
            )
        })
        .collect()
}

fn allergenic_ingredients<'a>(
    foods: &[(HashSet<&'a str>, Vec<&'a str>)],
) -> std::collections::HashMap<&'a str, HashSet<&'a str>, DefaultHashBuilder> {
    foods
        .iter()
        .flat_map(|(ingredients, allergens)| {
            allergens
                .iter()
                .copied()
                .map(move |allergen| (allergen, ingredients))
        })
        .into_grouping_map_with_hasher(DefaultHashBuilder::default())
        .aggregate(|acc: Option<HashSet<_>>, _, ingredients| {
            if let Some(mut acc) = acc {
                acc.retain(|ingredient| ingredients.contains(ingredient));
                Some(acc)
            } else {
                Some(ingredients.iter().copied().collect())
            }
        })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_21.txt");

    const EXAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test_case(EXAMPLE => 5)]
    #[test_case(INPUT => 2786)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => "mxmxvkd,sqjhc,fvjkl")]
    #[test_case(INPUT => "prxmdlz,ncjv,knprxg,lxjtns,vzzz,clg,cxfz,qdfpq")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
