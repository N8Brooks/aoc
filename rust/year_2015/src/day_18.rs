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
        let c = grid.get(i + 1).map_or(0, |&row| row);
        grid[i] = next_row(a, b, c) & and_mask;
        a = b;
    }
}

#[inline(always)]
pub fn next_row(a: u128, b: u128, c: u128) -> u128 {
    let (a_l, a_c, a_r) = (a << 1, a, a >> 1);
    let (b_l, b_r) = (b << 1, b >> 1);
    let (c_l, c_c, c_r) = (c << 1, c, c >> 1);

    let (s1, c1) = add3(a_l, a_c, a_r);
    let (s2, c2) = add2(b_l, b_r);
    let (s3, c3) = add3(c_l, c_c, c_r);
    let (s4, ones) = add3(c1, c2, c3);

    let (f1, t1) = add3(s1, s2, s3);
    let (f2, twos) = add2(t1, s4);
    let (eights, fours) = add2(f1, f2);

    let eq2 = !ones & twos & !fours & !eights;
    let eq3 = ones & twos & !fours & !eights;

    b & eq2 | eq3
}

/// returns (sum_bit, carry_bit) for per-bit addition of three 1-bit operands
#[inline(always)]
fn add3(a: u128, b: u128, c: u128) -> (u128, u128) {
    ((a & b) | (a & c) | (b & c), a ^ b ^ c)
}

#[inline(always)]
fn add2(a: u128, b: u128) -> (u128, u128) {
    (a & b, a ^ b)
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
