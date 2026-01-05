use hashbrown::HashMap;

pub fn part_1(input: &str) -> usize {
    let graph = parse_graph(input);
    let mut stack = vec!["you"];
    let mut total = 0;
    while let Some(u) = stack.pop() {
        if u == "out" {
            total += 1;
            continue;
        }
        stack.extend_from_slice(&graph[u]);
    }
    total
}

pub fn part_2(input: &str) -> usize {
    let graph = parse_graph(input);

    let mut indegree = HashMap::with_capacity(graph.len());
    for outputs in graph.values() {
        for &v in outputs {
            *indegree.entry(v).or_insert(0) += 1;
        }
    }

    let mut counts = HashMap::with_capacity(graph.len());
    counts.insert("svr", (0b00u8, 1));

    let mut stack = vec![("svr")];
    while let Some(u) = stack.pop() {
        let (mark, count_1) = counts[u];
        if u == "out" {
            return if mark == 0b11 { count_1 } else { 0 };
        }
        let update = graph[u]
            .iter()
            .inspect(|&&v| {
                let (mark_a, count_2) = counts.entry(v).or_default();
                let mark_b = match v {
                    "dac" => mark | 0b01,
                    "fft" => mark | 0b10,
                    _ => mark,
                };
                use std::cmp::Ordering::*;
                match mark_b.cmp(mark_a) {
                    Greater => {
                        *mark_a = mark_b;
                        *count_2 = count_1;
                    }
                    Equal => *count_2 += count_1,
                    Less => (),
                }
            })
            .filter(|&v| {
                let deg = indegree.get_mut(v).unwrap();
                *deg -= 1;
                *deg == 0
            });
        stack.extend(update);
    }
    panic!("no output found");
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (input, outputs) = line.split_once(": ").unwrap();
            let outputs = outputs.split(' ').collect();
            (input, outputs)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test_case(EXAMPLE => 5)]
    #[test_case(INPUT => 497)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test_case(EXAMPLE_2 => 2)]
    #[test_case(INPUT => 358564784931864)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
