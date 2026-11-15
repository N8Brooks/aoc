use hashbrown::HashMap;

pub fn part_1(input: &str) -> usize {
    let mut parents: HashMap<_, Vec<_>> =
        parse_bags(input).fold(HashMap::new(), |mut parents, (u, contents)| {
            parents.entry(u).or_default();
            contents.for_each(|(v, _)| parents.entry(v).or_default().push(u));
            parents
        });

    let mut stack = vec![parents.remove("shiny gold").unwrap_or_default()];
    let mut count = 0;

    while let Some(bags) = stack.pop() {
        bags.into_iter()
            .filter_map(|bag| parents.remove(bag))
            .inspect(|_| count += 1)
            .collect_into(&mut stack);
    }

    count
}

pub fn part_2(input: &str) -> usize {
    let children: HashMap<_, Vec<_>> = parse_bags(input)
        .map(|(u, contents)| (u, contents.collect()))
        .collect();

    let mut stack = vec![("shiny gold", false)];
    let mut counts = HashMap::new();

    while let Some((u, ready)) = stack.pop() {
        if ready {
            let count = children[u].iter().map(|&(v, n)| (1 + counts[v]) * n).sum();
            counts.insert(u, count);
        } else if !counts.contains_key(u) {
            stack.push((u, true));
            children[u]
                .iter()
                .map(|&(v, _)| (v, false))
                .collect_into(&mut stack);
        }
    }

    counts["shiny gold"]
}

fn parse_bags(input: &str) -> impl Iterator<Item = (&str, impl Iterator<Item = (&str, usize)>)> {
    input.lines().map(|line| {
        let (u, contents) = line.split_once(" bags contain ").unwrap();
        let contents = contents
            .strip_suffix('.')
            .unwrap()
            .split(", ")
            .filter(|&s| s != "no other bags")
            .map(|s| {
                let (count, v) = s.split_once(' ').unwrap();
                let v = v.trim_suffix('s').strip_suffix(" bag").unwrap();
                let count = count.parse().unwrap();
                (v, count)
            });
        (u, contents)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    const EXAMPLE_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test_case(EXAMPLE_1 => 4)]
    #[test_case(INPUT => 205)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test_case(EXAMPLE_1 => 32)]
    #[test_case(EXAMPLE_2 => 126)]
    #[test_case(INPUT => 80902)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
