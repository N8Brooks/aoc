use std::iter;

pub fn part_1(input: &str) -> usize {
    bridges(input).map(|(_, strength)| strength).max().unwrap()
}

pub fn part_2(input: &str) -> usize {
    bridges(input).max().unwrap().1
}

fn bridges(input: &str) -> impl Iterator<Item = (usize, usize)> {
    let mut by_port = vec![Vec::new()];
    for (id, line) in input.lines().enumerate() {
        assert!(
            id < u64::BITS as usize,
            "at most 64 components are supported"
        );
        let (a, b) = line.split_once('/').unwrap();
        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
        let bit = 1u64 << id;
        by_port.resize_with(by_port.len().max(a.max(b) + 1), Vec::new);
        by_port[a].push((b, bit));
        if a != b {
            by_port[b].push((a, bit));
        }
    }

    let mut used = 0;
    let mut bridge = Vec::new();
    let mut strength = 0;
    let mut port = 0;
    let mut start = 0;

    iter::from_fn(move || {
        loop {
            if let Some((i, &(next, bit))) = by_port[port]
                .iter()
                .enumerate()
                .skip(start)
                .find(|&(_, &(_, bit))| used & bit == 0)
            {
                used ^= bit;
                bridge.push((port, i));
                strength += port + next;
                port = next;
                start = 0;

                return Some((bridge.len(), strength));
            }

            // Backtrack.
            let (previous, i) = bridge.pop()?;
            let (next, bit) = by_port[previous][i];

            used ^= bit;
            strength -= previous + next;
            port = previous;
            start = i + 1;
        }
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_24.txt");

    const EXAMPLE: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test_case(EXAMPLE => 31)]
    #[test_case(INPUT => 1940)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 19)]
    #[test_case(INPUT => 1928)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
