use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            (x, y)
        })
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let dx = x1.abs_diff(x2) + 1;
            let dy = y1.abs_diff(y2) + 1;
            dx * dy
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut points: Vec<_> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            (x, y)
        })
        .collect();
    points.push(points[0]);

    points[..points.len() - 1]
        .iter()
        .tuple_combinations()
        .filter(|&(&(x1, y1), &(x2, y2))| {
            // Inner rectangle
            let (rx1, rx2) = (x1.min(x2) + 1, x1.max(x2) - 1);
            let (ry1, ry2) = (y1.min(y2) + 1, y1.max(y2) - 1);
            !points.array_windows().any(|&[(tx1, ty1), (tx2, ty2)]| {
                // Union  between rectangle and tiles
                let ux1 = tx1.min(tx2).max(rx1);
                let ux2 = tx1.max(tx2).min(rx2);
                let uy1 = ty1.min(ty2).max(ry1);
                let uy2 = ty1.max(ty2).min(ry2);
                ux1 <= ux2 && uy1 <= uy2
            })
        })
        .map(|(&(x1, y1), &(x2, y2))| {
            let dx = x1.abs_diff(x2) + 1;
            let dy = y1.abs_diff(y2) + 1;
            dx * dy
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test_case(EXAMPLE => 50)]
    #[test_case(INPUT => 4763932976)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 24)]
    #[test_case(INPUT => 1501292304)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
