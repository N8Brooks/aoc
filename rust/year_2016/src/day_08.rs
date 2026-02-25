use Instruction::*;
use hashbrown::HashMap;
use std::{cell::LazyCell, str::FromStr};

pub fn part_1(input: &str) -> usize {
    plot(input)
        .into_iter()
        .flatten()
        .filter(|&pixel| pixel == b'#')
        .count()
}

pub fn part_2(input: &str) -> String {
    let letters: LazyCell<HashMap<[[u8; 5]; 6], char>> = LazyCell::new(|| {
        [
            (
                #[rustfmt::skip]
                [
                    *b"###  ",
                    *b"#  # ",
                    *b"#  # ",
                    *b"###  ",
                    *b"# #  ",
                    *b"#  # ",
                ],
                'R',
            ),
            (
                #[rustfmt::skip]
                [
                    *b"#  # ",
                    *b"#  # ",
                    *b"#  # ",
                    *b"#  # ",
                    *b"#  # ",
                    *b" ##  ",
                ],
                'U',
            ),
            (
                #[rustfmt::skip]
                [
                    *b" ##  ",
                    *b"#  # ",
                    *b"#    ",
                    *b"#    ",
                    *b"#  # ",
                    *b" ##  ",
                ],
                'C',
            ),
            (
                #[rustfmt::skip]
                [
                    *b"#### ",
                    *b"#    ",
                    *b"###  ",
                    *b"#    ",
                    *b"#    ",
                    *b"#### ",
                ],
                'E',
            ),
            (
                #[rustfmt::skip]
                [
                    *b" ##  ",
                    *b"#  # ",
                    *b"#  # ",
                    *b"#  # ",
                    *b"#  # ",
                    *b" ##  ",
                ],
                'O',
            ),
            (
                #[rustfmt::skip]
                [
                    *b" ### ",
                    *b"  #  ",
                    *b"  #  ",
                    *b"  #  ",
                    *b"  #  ",
                    *b" ### ",
                ],
                'I',
            ),
            (
                #[rustfmt::skip]
                [
                    *b"#    ",
                    *b"#    ",
                    *b"#    ",
                    *b"#    ",
                    *b"#    ",
                    *b"#### ",
                ],
                'L',
            ),
        ]
        .into_iter()
        .collect()
    });
    transpose(plot(input))
        .as_chunks()
        .0
        .iter()
        .map(|&cols| letters[&transpose(cols)])
        .collect()
}

fn plot(input: &str) -> [[u8; 50]; 6] {
    let mut plot = [[b' '; 50]; 6];
    for line in input.lines() {
        match line.parse().unwrap() {
            Rect { width, height } => {
                plot.iter_mut()
                    .take(height)
                    .for_each(|row| row[..width].fill(b'#'));
            }
            RotateRow { row, by } => {
                plot[row].rotate_right(by);
            }
            RotateCol { col, by } => {
                let mut column = plot.map(|row| row[col]);
                column.rotate_right(by);
                plot.iter_mut()
                    .zip(column)
                    .for_each(|(row, pixel)| row[col] = pixel);
            }
        }
    }
    plot
}

enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, by: usize },
    RotateCol { col: usize, by: usize },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rect) = s.strip_prefix("rect ") {
            let (width, height) = rect.split_once('x').unwrap();
            Ok(Rect {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
            })
        } else if let Some(rotate_row) = s.strip_prefix("rotate row y=") {
            let (row, by) = rotate_row.split_once(" by ").unwrap();
            Ok(RotateRow {
                row: row.parse().unwrap(),
                by: by.parse().unwrap(),
            })
        } else if let Some(rotate_col) = s.strip_prefix("rotate column x=") {
            let (col, by) = rotate_col.split_once(" by ").unwrap();
            Ok(RotateCol {
                col: col.parse().unwrap(),
                by: by.parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

/// Transposes a 2D array of size MxN into one of size NxM.
fn transpose<const M: usize, const N: usize, T>(m: [[T; N]; M]) -> [[T; M]; N] {
    use std::array::from_fn;
    let mut iters = m.map(|r| r.into_iter());
    from_fn(|_| from_fn(|i| iters[i].next().unwrap()))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    #[test_case(INPUT => 121)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => "RURUCEOEIL")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
