use std::mem;

pub fn part_1(input: &str) -> usize {
    UnionFind::from_input(input).component_size(0)
}

pub fn part_2(input: &str) -> usize {
    UnionFind::from_input(input).components()
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn from_input(input: &str) -> Self {
        let n = input.lines().count();

        input
            .lines()
            .map(|line| {
                let (i, js) = line.split_once(" <-> ").unwrap();
                let js = js.split(", ").map(|j| j.parse().unwrap());
                (i.parse().unwrap(), js)
            })
            .fold(UnionFind::new(n), |mut uf, (i, js)| {
                js.for_each(|j| uf.unite(i, j));
                uf
            })
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.count -= 1;
    }

    fn component_size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        self.size[root]
    }

    fn components(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    #[test_case(EXAMPLE => 6)]
    #[test_case(INPUT => 306)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 200)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
