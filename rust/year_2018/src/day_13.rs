pub fn part_1(input: &str) -> String {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut carts: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &b)| (i, j, b)))
        .filter_map(|(i, j, b)| {
            let dir = match b {
                b'^' => Some((-1, 0)),
                b'v' => Some((1, 0)),
                b'<' => Some((0, -1)),
                b'>' => Some((0, 1)),
                _ => None,
            }?;
            Some(((i, j), dir, 0))
        })
        .collect();

    loop {
        for i in 0..carts.len() {
            {
                let ((i, j), (di, dj), turn) = &mut carts[i];
                *i = i.strict_add_signed(*di);
                *j = j.strict_add_signed(*dj);

                match map[*i][*j] {
                    b'/' => {
                        (*di, *dj) = (-*dj, -*di);
                    }
                    b'\\' => {
                        (*di, *dj) = (*dj, *di);
                    }
                    b'+' => {
                        *turn = match *turn {
                            0 => {
                                (*di, *dj) = (-*dj, *di);
                                1
                            }
                            1 => 2,
                            2 => {
                                (*di, *dj) = (*dj, -*di);
                                0
                            }
                            _ => unreachable!(),
                        }
                    }
                    b'|' | b'^' | b'v' | b'-' | b'<' | b'>' => { /* no change */ }
                    track => panic!("invalid track {track}"),
                }
            }

            if let Some((_, ((y, x), _, _))) = carts
                .iter()
                .enumerate()
                .find(|&(j, cart)| i != j && cart.0 == carts[i].0)
            {
                return format!("{x},{y}");
            }
        }

        carts.sort_unstable_by_key(|(pos, _, _)| *pos);
    }
}

pub fn part_2(input: &str) -> String {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut carts: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &b)| (i, j, b)))
        .filter_map(|(i, j, b)| {
            let dir = match b {
                b'^' => Some((-1, 0)),
                b'v' => Some((1, 0)),
                b'<' => Some((0, -1)),
                b'>' => Some((0, 1)),
                _ => None,
            }?;
            Some(((i, j), dir, 0))
        })
        .collect();

    while carts.len() > 1 {
        let mut i = 0;
        while i < carts.len() {
            {
                let ((i, j), (di, dj), turn) = &mut carts[i];
                *i = i.strict_add_signed(*di);
                *j = j.strict_add_signed(*dj);

                match map[*i][*j] {
                    b'/' => {
                        (*di, *dj) = (-*dj, -*di);
                    }
                    b'\\' => {
                        (*di, *dj) = (*dj, *di);
                    }
                    b'+' => {
                        *turn = match *turn {
                            0 => {
                                (*di, *dj) = (-*dj, *di);
                                1
                            }
                            1 => 2,
                            2 => {
                                (*di, *dj) = (*dj, -*di);
                                0
                            }
                            _ => unreachable!(),
                        }
                    }
                    b'|' | b'^' | b'v' | b'-' | b'<' | b'>' => { /* no change */ }
                    track => panic!("invalid track {track}"),
                }
            }

            if let Some((j, _)) = carts
                .iter()
                .enumerate()
                .find(|&(j, cart)| i != j && cart.0 == carts[i].0)
            {
                if j < i {
                    carts.remove(i);
                    carts.remove(j);
                    i -= 1;
                } else {
                    carts.remove(j);
                    carts.remove(i);
                }
            } else {
                i += 1;
            }
        }

        carts.sort_unstable_by_key(|(pos, _, _)| *pos);
    }

    let ((y, x), _, _) = carts[0];
    format!("{x},{y}")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "\
/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ";

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    #[test_case(EXAMPLE_1 => "7,3")]
    #[test_case(INPUT => "41,22")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "\
/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/";

    #[test_case(EXAMPLE_2 => "6,4")]
    #[test_case(INPUT => "84,90")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
