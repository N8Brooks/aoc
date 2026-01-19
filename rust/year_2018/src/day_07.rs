use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    iter::{self},
};

pub fn part_1(input: &str) -> String {
    let (graph, mut indegree) = parse_graph(input);
    let mut queue: VecDeque<_> = indegree
        .iter()
        .enumerate()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(u, _)| u)
        .collect();
    iter::from_fn(|| {
        let u = queue.pop_front()?;
        for &v in &graph[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                let i = queue.partition_point(|&x| x < v);
                queue.insert(i, v);
            }
        }
        Some((b'A' + u as u8) as char)
    })
    .collect()
}

pub fn part_2(input: &str) -> usize {
    completion_time(input, 5, 60)
}

fn completion_time(input: &str, workers: usize, base: usize) -> usize {
    let (graph, mut indegree) = parse_graph(input);
    let mut it = indegree
        .iter()
        .enumerate()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(u, _)| u);
    let mut heap: BinaryHeap<_> = it
        .by_ref()
        .take(workers)
        .map(|u| (Reverse(base + u + 1), u))
        .collect();
    let mut stack: Vec<_> = it.collect();

    iter::from_fn(|| {
        let (Reverse(t), u) = heap.pop()?;
        for &v in &graph[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                stack.push(v);
            }
        }

        iter::from_fn(|| stack.pop())
            .take(workers - heap.len())
            .map(|w| (Reverse(t + (base + w + 1)), w))
            .collect_into(&mut heap);

        Some(t)
    })
    .last()
    .unwrap()
}

fn parse_graph(input: &str) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut graph = Vec::with_capacity(26);
    let mut indegree = Vec::with_capacity(26);
    for line in input.lines() {
        let bytes = line.as_bytes();
        let u = (bytes[5] - b'A') as usize;
        let v = (bytes[36] - b'A') as usize;
        let n = u.max(v) + 1;
        if graph.len() < n {
            graph.resize_with(n, Vec::new);
            indegree.resize(n, 0);
        }
        graph[u].push(v);
        indegree[v] += 1;
    }
    (graph, indegree)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    #[test_case(EXAMPLE => "CABDFE")]
    #[test_case(INPUT => "HEGMPOAWBFCDITVXYZRKUQNSLJ")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(INPUT => 1226)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }

    #[test_case(EXAMPLE => 15)]
    fn completion_time(input: &str) -> usize {
        super::completion_time(input, 2, 0)
    }
}
