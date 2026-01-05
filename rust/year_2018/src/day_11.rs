use itertools::Itertools;

const MAX_COORDINATE: usize = 300;

pub fn part_1(input: &str) -> String {
    let fuel_cells = FuelCells::new(input);
    let (x, y) = (1..=MAX_COORDINATE - 2)
        .cartesian_product(1..=MAX_COORDINATE - 2)
        .max_by_key(|coordinates| fuel_cells.total_power(coordinates, &3))
        .unwrap();
    format!("{x},{y}")
}

pub fn part_2(input: &str) -> String {
    let fuel_cells = FuelCells::new(input);
    let ((x, y), size) = Some(((1, 1), 0))
        .into_iter()
        .chain((1..=MAX_COORDINATE).flat_map(|size| {
            (1..=MAX_COORDINATE - size + 1)
                .cartesian_product(1..=MAX_COORDINATE - size + 1)
                .map(move |coordinates| (coordinates, size))
        }))
        .max_by_key(|(coordinates, size)| fuel_cells.total_power(coordinates, size))
        .unwrap();
    format!("{x},{y},{size}")
}

#[derive(Debug)]
struct FuelCells(Vec<[i64; MAX_COORDINATE + 1]>);

impl FuelCells {
    fn new(grid_serial_number: &str) -> FuelCells {
        let grid_serial_number = grid_serial_number.parse().unwrap();
        let accumulative_power = (1..=MAX_COORDINATE)
            .map(|y| {
                let horizontal_power_accumulation = (1..=MAX_COORDINATE)
                    .map(move |x| power_level((x, y), grid_serial_number))
                    .scan(0, |sum, power_level| {
                        *sum += power_level;
                        Some(*sum)
                    });
                Some(0).into_iter().chain(horizontal_power_accumulation)
            })
            .scan([0; MAX_COORDINATE + 1], |accumulation, power_levels| {
                accumulation
                    .iter_mut()
                    .zip(power_levels)
                    .for_each(|(acc, power_level)| *acc += power_level);
                Some(*accumulation)
            });
        let accumulative_power = Some([0; MAX_COORDINATE + 1])
            .into_iter()
            .chain(accumulative_power)
            .collect();
        FuelCells(accumulative_power)
    }

    fn total_power(&self, &(x, y): &(usize, usize), size: &usize) -> i64 {
        let (x_0, y_0) = (x - 1, y - 1);
        let (x_1, y_1) = (x + size - 1, y + size - 1);
        let top_left = self.0[y_0][x_0];
        let top_right = self.0[y_0][x_1];
        let bot_left = self.0[y_1][x_0];
        let bot_right = self.0[y_1][x_1];
        bot_right - bot_left - top_right + top_left
    }
}

fn power_level((x, y): (usize, usize), grid_serial_number: i64) -> i64 {
    let rack_id = x as i64 + 10;
    let power_level = (rack_id * y as i64 + grid_serial_number) * rack_id;
    let hundreds_digit = power_level / 100 % 10;
    hundreds_digit - 5
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    #[test_case("18" => "33,45")]
    #[test_case("42" => "21,61")]
    #[test_case(INPUT => "21,54")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case("18" => "90,269,16")]
    #[test_case("42" => "232,251,12")]
    #[test_case(INPUT => "236,268,11")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }

    #[test_case((3, 5), 8 => 4)]
    #[test_case((122, 79), 57 => -5)]
    #[test_case((217, 196), 39 => 0)]
    #[test_case((101, 153), 71 => 4)]
    fn power_level(coordinates: (usize, usize), grid_serial_number: i64) -> i64 {
        super::power_level(coordinates, grid_serial_number)
    }
}
