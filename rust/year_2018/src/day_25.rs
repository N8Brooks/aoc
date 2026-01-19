use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    const N: usize = 17;
    let mut space = [[[[None; N]; N]; N]; N];

    let points = parse_input(input);

    let mins = points
        .iter()
        .copied()
        .reduce(|mut mins, p| {
            mins.iter_mut().zip(p).for_each(|(a, b)| *a = (*a).min(b));
            mins
        })
        .unwrap();

    let manhatten_sphere: Vec<_> = (-3isize..=3)
        .map(|dx| (dx, dx.abs()))
        .flat_map(|(dx, r)| (-3 + r..=3 - r).map(move |dy| ([dx, dy], r + dy.abs())))
        .flat_map(|([dx, dy], r)| (-3 + r..=3 - r).map(move |dz| ([dx, dy, dz], r + dz.abs())))
        .flat_map(|([dx, dy, dz], r)| (-3 + r..=3 - r).map(move |dw| [dx, dy, dz, dw]))
        .collect();

    let mut parent: Vec<_> = (0..points.len()).collect();
    let mut count = points.len();

    for (i, point) in points.iter().enumerate() {
        let [x, y, z, w] = point
            .iter()
            .zip(&mins)
            .map(|(&a, &b)| usize::try_from(a - b).unwrap())
            .collect_array()
            .unwrap();
        for &[dx, dy, dz, dw] in &manhatten_sphere {
            if let Some(&j) = Some(&space)
                .and_then(|a| a.get(x.checked_add_signed(dx)?))
                .and_then(|b| b.get(y.checked_add_signed(dy)?))
                .and_then(|c| c.get(z.checked_add_signed(dz)?))
                .and_then(|d| d.get(w.checked_add_signed(dw)?))
                .flatten_ref()
            {
                count -= unite(&mut parent, i, j) as usize;
            }
        }
        space[x][y][z][w] = Some(i);
    }

    count
}

fn parse_input(input: &str) -> Vec<[isize; 4]> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect_array()
                .unwrap()
        })
        .collect()
}

fn find(parent: &mut [usize], i: usize) -> usize {
    if parent[i] != i {
        let p = parent[i];
        parent[i] = find(parent, p);
    }
    parent[i]
}

fn unite(parent: &mut [usize], i: usize, j: usize) -> bool {
    let root_x = find(parent, i);
    let root_y = find(parent, j);
    if root_x != root_y {
        parent[root_y] = root_x;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_25.txt");

    const EXAMPLE_1: &str = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";

    const EXAMPLE_2: &str = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";

    const EXAMPLE_3: &str = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";

    const EXAMPLE_4: &str = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";

    #[test_case(EXAMPLE_1 => 2)]
    #[test_case(EXAMPLE_2 => 4)]
    #[test_case(EXAMPLE_3 => 3)]
    #[test_case(EXAMPLE_4 => 8)]
    #[test_case(INPUT => 367)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }
}
