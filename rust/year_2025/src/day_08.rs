use std::{cmp::Reverse, collections::BinaryHeap, mem};

use itertools::Itertools as _;

pub fn part_1(input: &str, k: usize) -> usize {
    let points = parse_points(input);
    (0..points.len())
        .tuple_combinations()
        .k_smallest_relaxed_by_key(k, |&(i1, i2)| idistance(points[i1], points[i2]))
        .fold(UnionFind::new(points.len()), |mut uf, (i1, i2)| {
            uf.unite(i1, i2);
            uf
        })
        .component_sizes()
        .k_largest_relaxed(3)
        .product()
}

pub fn part_2(input: &str) -> usize {
    let points = parse_points(input);
    let (mut i1, mut i2) = (0, 0);
    let mut heap: BinaryHeap<_> = (0..points.len())
        .tuple_combinations()
        .map(|(i1, i2)| (Reverse(idistance(points[i1], points[i2])), i1, i2))
        .collect();
    let mut uf = UnionFind::new(points.len());
    while uf.count > 1 {
        (_, i1, i2) = heap.pop().unwrap();
        uf.unite(i1, i2);
    }
    let x1 = points[i1][0];
    let x2 = points[i2][0];
    x1 * x2
}

fn parse_points(input: &str) -> Vec<[usize; 3]> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect_array::<3>()
                .unwrap()
        })
        .collect()
}

/// Integer euclidean distance between two 3D points.
#[inline(always)]
fn idistance([x1, y1, z1]: [usize; 3], [x2, y2, z2]: [usize; 3]) -> usize {
    // Assumes integer is enough precision
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);
    let dz = z1.abs_diff(z2);
    (dx * dx + dy * dy + dz * dz).isqrt()
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

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
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

    /// Sizes of all distinct components.
    fn component_sizes(&mut self) -> impl Iterator<Item = usize> {
        let n = self.parent.len();
        let mut new = vec![true; n];
        (0..n).filter_map(move |r| {
            let r = self.find(r);
            mem::replace(&mut new[r], false).then(|| self.size[r])
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test_case(EXAMPLE, 10 => 40)]
    #[test_case(INPUT, 1_000 => 63920)]
    fn part_1(input: &str, k: usize) -> usize {
        super::part_1(input, k)
    }

    #[test_case(EXAMPLE => 25272)]
    #[test_case(INPUT => 1026594680)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
