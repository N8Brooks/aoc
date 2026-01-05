pub fn part_1(input: &str) -> usize {
    let root = Node::parse_tree(input);
    let mut stack = vec![root];
    let mut total = 0;
    while let Some(node) = stack.pop() {
        total += node.metadata.into_iter().sum::<usize>();
        stack.extend(node.children);
    }
    total
}

pub fn part_2(input: &str) -> usize {
    let root = Node::parse_tree(input);
    let mut stack = vec![&root];
    let mut total = 0;
    while let Some(node) = stack.pop() {
        if node.children.is_empty() {
            total += node.metadata.iter().sum::<usize>();
        } else {
            node.metadata
                .iter()
                .filter_map(|&idx| node.children.get(idx.checked_sub(1)?))
                .collect_into(&mut stack);
        }
    }
    total
}

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse_tree(input: &str) -> Node {
        let mut nums = input.split(' ').map(|s| s.parse().unwrap());
        fn parse_node(nums: &mut impl Iterator<Item = usize>) -> Node {
            let [n, m] = nums.next_chunk().unwrap();
            let children = (0..n).map(|_| parse_node(nums)).collect();
            let metadata = nums.take(m).collect();
            Node { children, metadata }
        }
        parse_node(&mut nums)
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    #[test_case(EXAMPLE => 138)]
    #[test_case(INPUT => 42472)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 66)]
    #[test_case(INPUT => 21810)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
