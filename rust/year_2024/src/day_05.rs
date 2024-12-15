use hashbrown::{HashMap, HashSet};

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (rules, pages) = parse_input(input);
    let nexts = rules.into_iter().into_group_map();
    pages
        .into_iter()
        .filter(|update| {
            let indexes: HashMap<usize, usize> =
                update.iter().enumerate().map(|(i, &x)| (x, i)).collect();
            update.iter().enumerate().all(|(i, x)| {
                nexts
                    .get(x)
                    .is_none_or(|ys| ys.iter().all(|y| indexes.get(y).is_none_or(|&j| i < j)))
            })
        })
        .map(|update| {
            let i = update.len() / 2;
            update[i]
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let (rules, pages) = parse_input(input);
    pages
        .into_iter()
        .filter_map(|update| {
            let set: HashSet<usize> = update.iter().copied().collect();
            let rules = rules
                .iter()
                .filter(|(x, y)| set.contains(x) & set.contains(y))
                .copied()
                .collect_vec();
            let mut indegree = rules.iter().map(|(_, y)| *y).counts();
            let graph = rules.into_iter().into_group_map();

            let mut frontier = graph
                .keys()
                .filter(|x| !indegree.contains_key(x))
                .copied()
                .collect_vec();
            let mut order = HashMap::new();

            let mut index = 0;
            while !frontier.is_empty() {
                let mut frontier_2 = Vec::new();
                for x in frontier {
                    order.insert(x, index);
                    if let Some(ys) = graph.get(&x) {
                        for y in ys {
                            *indegree.get_mut(y).unwrap() -= 1;
                            if indegree[y] == 0 {
                                frontier_2.push(*y);
                            }
                        }
                    }
                }
                frontier = frontier_2;
                index += 1;
            }

            let mut sorted = update.clone();
            sorted.sort_unstable_by_key(|x| order[x]);
            (update != sorted).then(|| {
                let i = sorted.len() / 2;
                sorted[i]
            })
        })
        .sum()
}

pub fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (x, y) = line.split_once('|').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect_vec();
    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (rules, pages)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test_case(EXAMPLE, 143)]
    #[test_case(INPUT, 4281)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 123)]
    #[test_case(INPUT, 5466)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
