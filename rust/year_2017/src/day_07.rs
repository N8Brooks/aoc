use hashbrown::{DefaultHashBuilder, HashMap, HashSet};
use itertools::Itertools;

pub fn part_1(input: &str) -> &str {
    let (parents, children): (HashSet<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (parent, children) = line.split_once(" -> ").unzip();
            let (parent, _weight) = parent.unwrap_or(line).split_once(' ').unwrap();
            (parent, children.into_iter().flat_map(|b| b.split(", ")))
        })
        .unzip();
    let b = children.into_iter().flatten().collect();
    parents.difference(&b).exactly_one().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let graph: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (parent, children) = line.split_once(" -> ").unzip();
            let (parent, weight) = parent.unwrap_or(line).split_once(' ').unwrap();
            let weight: usize = weight.strip_circumfix("(", ")").unwrap().parse().unwrap();
            let children = children
                .into_iter()
                .flat_map(|b| b.split(", "))
                .collect_vec();
            (parent, (weight, children))
        })
        .collect();

    let child_names: HashSet<_> = graph
        .values()
        .flat_map(|(_, children)| children.iter().copied())
        .collect();

    let root = graph
        .keys()
        .copied()
        .find(|name| !child_names.contains(name))
        .unwrap();

    let mut stack = vec![(root, false)];
    let mut totals = HashMap::<&str, usize>::new();

    while let Some((node, expanded)) = stack.pop() {
        let (weight, children) = &graph[node];

        if !expanded {
            stack.push((node, true));
            stack.extend(children.iter().map(|&child| (child, false)));
            continue;
        }

        let counts = children
            .iter()
            .map(|&child| totals[child])
            .counts_with_hasher(DefaultHashBuilder::default());

        if counts.len() > 1 {
            let (&correct, &count) = counts.iter().max_by_key(|&(_, &count)| count).unwrap();

            assert!(
                count > 1 && count + 1 == children.len(),
                "ambiguous imbalance"
            );

            let incorrect = children
                .iter()
                .copied()
                .find(|&child| totals[child] != correct)
                .unwrap();

            let diff = correct.checked_signed_diff(totals[incorrect]).unwrap();
            return graph[incorrect].0.strict_add_signed(diff);
        }

        totals.insert(
            node,
            weight + children.iter().map(|&child| totals[child]).sum::<usize>(),
        );
    }

    unreachable!("expected an unbalanced tower")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    const EXAMPLE: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test_case(EXAMPLE => "tknk")]
    #[test_case(INPUT => "ahnofa")]
    fn part_1(input: &str) -> &str {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 60)]
    #[test_case(INPUT => 802)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
