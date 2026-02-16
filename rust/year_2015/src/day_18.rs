use itertools::Itertools as _;

pub fn part_1<const N: usize>(input: &str, steps: usize) -> u32 {
    let grid_0: [_; N] = parse_grid(input);
    (0..steps)
        .fold(grid_0, |mut grid, _| {
            step(&mut grid);
            grid
        })
        .into_iter()
        .map(|row| row.count_ones())
        .sum()
}

pub fn part_2<const N: usize>(input: &str, steps: usize) -> u32 {
    let or_mask = 1 | (1 << (N - 1));
    let mut grid_0: [_; N] = parse_grid(input);
    grid_0[0] |= or_mask;
    grid_0[N - 1] |= or_mask;
    (0..steps)
        .fold(grid_0, |mut grid, _| {
            step(&mut grid);
            grid[0] |= or_mask;
            grid[N - 1] |= or_mask;
            grid
        })
        .into_iter()
        .map(|row| row.count_ones())
        .sum()
}

fn parse_grid<const N: usize>(input: &str) -> [u128; N] {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| u128::from(b == b'#'))
                .reduce(|row, b| (row << 1) | b)
                .unwrap()
        })
        .collect_array()
        .unwrap()
}

fn step<const N: usize>(grid: &mut [u128; N]) {
    let and_mask: u128 = (1 << N) - 1;
    let mut a = 0;
    for i in 0..grid.len() {
        let b = grid[i];
        let c = grid.get(i + 1).copied().unwrap_or(0);
        grid[i] = next_row(a, b, c) & and_mask;
        a = b;
    }
}

#[inline(always)]
pub fn next_row(a: u128, b: u128, c: u128) -> u128 {
    let (a_l, a_c, a_r) = (a << 1, a, a >> 1);
    let (b_l, b_r) = (b << 1, b >> 1);
    let (c_l, c_c, c_r) = (c << 1, c, c >> 1);

    let [b1, b2, b3, b4] = cs8(a_l, a_c, a_r, b_l, b_r, c_l, c_c, c_r);
    let eq2 = !b1 & b2 & !b3 & !b4;
    let eq3 = b1 & b2 & !b3 & !b4;

    eq3 | b & eq2
}

#[allow(clippy::too_many_arguments)]
fn cs8(a: u128, b: u128, c: u128, d: u128, e: u128, f: u128, g: u128, h: u128) -> [u128; 4] {
    let [o1, t1, f1] = cs4(a, b, c, d);
    let [o2, t2, f2] = cs4(e, f, g, h);
    let [b1, t3] = cs2(o1, o2);
    let [b2, f3] = cs3(t1, t2, t3);
    let [b3, b4] = cs3(f1, f2, f3);
    [b1, b2, b3, b4]
}

#[inline(always)]
fn cs4(a: u128, b: u128, c: u128, d: u128) -> [u128; 3] {
    let (ab, cd) = (a & b, c & d);
    let (p0, p1) = (a ^ b, c ^ d);
    [p0 ^ p1, ab ^ cd ^ p0 & p1, ab & cd]
}

#[inline(always)]
fn cs3(a: u128, b: u128, c: u128) -> [u128; 2] {
    let p = a ^ b;
    [p ^ c, (a & b) | (c & p)]
}

#[inline(always)]
fn cs2(a: u128, b: u128) -> [u128; 2] {
    [a ^ b, a & b]
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    const EXAMPLE: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test_case(EXAMPLE, 5 => 4)]
    fn part_1_example(input: &str, steps: usize) -> u32 {
        super::part_1::<6>(input, steps)
    }
    #[test_case(INPUT, 100 => 1061)]
    fn part_1(input: &str, steps: usize) -> u32 {
        super::part_1::<100>(input, steps)
    }

    #[test_case(EXAMPLE, 5 => 17)]
    fn part_2_example(input: &str, steps: usize) -> u32 {
        super::part_2::<6>(input, steps)
    }

    #[test_case(INPUT, 100 => 1006)]
    fn part_2(input: &str, steps: usize) -> u32 {
        super::part_2::<100>(input, steps)
    }
}
