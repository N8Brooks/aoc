pub fn part_1(input: &str) -> String {
    let (map, mut carts) = parse_input(input);

    loop {
        for i in 0..carts.len() {
            carts[i] = carts[i].step(&map);

            if let Some((_, c)) = carts
                .iter()
                .enumerate()
                .find(|&(j, c)| i != j && c.pos == carts[i].pos)
            {
                let (y, x) = c.pos;
                return format!("{x},{y}");
            }
        }

        carts.sort_unstable_by_key(|c| c.pos);
    }
}

pub fn part_2(input: &str) -> String {
    let (map, mut carts) = parse_input(input);

    while carts.len() > 1 {
        let mut i = 0;
        while i < carts.len() {
            carts[i] = carts[i].step(&map);

            if let Some((j, _)) = carts
                .iter()
                .enumerate()
                .find(|&(j, c)| i != j && c.pos == carts[i].pos)
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

        carts.sort_unstable_by_key(|c| c.pos);
    }

    let (y, x) = carts[0].pos;
    format!("{x},{y}")
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Cart>) {
    let mut map: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let carts: Vec<_> = map
        .iter_mut()
        .enumerate()
        .flat_map(|(i, row)| row.iter_mut().enumerate().map(move |(j, b)| (i, j, b)))
        .filter_map(|(i, j, b)| {
            let (dir, track) = match b {
                b'^' => Some(((-1, 0), b'|')),
                b'v' => Some(((1, 0), b'|')),
                b'<' => Some(((0, -1), b'-')),
                b'>' => Some(((0, 1), b'-')),
                _ => None,
            }?;
            *b = track;
            Some(Cart {
                pos: (i, j),
                dir,
                turn: 0,
            })
        })
        .collect();
    (map, carts)
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    pos: (usize, usize),
    dir: (isize, isize),
    turn: u8,
}

impl Cart {
    fn step(self, map: &[Vec<u8>]) -> Cart {
        let Cart { pos, dir, turn } = self;

        let (i, j) = pos;
        let (di, dj) = dir;
        let i = i.strict_add_signed(di);
        let j = j.strict_add_signed(dj);

        let (dir, turn) = match map[i][j] {
            b'/' => ((-dj, -di), turn),
            b'\\' => ((dj, di), turn),
            b'+' => match turn {
                0 => ((-dj, di), 1),
                1 => (dir, 2),
                2 => ((dj, -di), 0),
                _ => panic!("invalid turn {turn}"),
            },
            b'|' | b'-' => (dir, turn),
            track => panic!("invalid track {track}"),
        };

        Cart {
            pos: (i, j),
            dir,
            turn,
        }
    }
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
