use hashbrown::HashMap;
use itertools::Itertools as _;
use num::{Complex, Integer as _};

pub fn part_1(input: &str) -> usize {
    paint_panels(input, false).len()
}

pub fn part_2(input: &str) -> String {
    let painted = paint_panels(input, true);
    let (min_j, max_j) = painted.keys().map(|p| p.re).minmax().into_option().unwrap();
    let (min_i, max_i) = painted.keys().map(|p| p.im).minmax().into_option().unwrap();
    (min_i..=max_i)
        .rev()
        .map(|y| {
            (min_j..=max_j)
                .map(|x| {
                    let p = Complex::new(x, y);
                    if painted.get(&p).copied().unwrap_or(false) {
                        '#'
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}

fn paint_panels(input: &str, init: bool) -> HashMap<Complex<i32>, bool> {
    let mut painted = HashMap::new();
    let mut dir = Complex::new(0, 1);
    let mut loc = Complex::new(0, 0);
    painted.insert(loc, init);
    let mut computer = Intcode::new(input);
    loop {
        let color = painted.get(&loc).copied().unwrap_or(false);
        if let Some(new_color) = computer.next(color) {
            painted.insert(loc, new_color);
        } else {
            break;
        }
        if let Some(turn_right) = computer.next(color) {
            dir = if turn_right {
                Complex::new(dir.im, -dir.re)
            } else {
                Complex::new(-dir.im, dir.re)
            };
            loc += dir;
        } else {
            break;
        }
    }
    painted
}

struct Intcode {
    /// Computer's memory
    memory: Vec<isize>,
    /// Instruction pointer
    ip: usize,
    /// Relative base
    rb: isize,
}

impl Intcode {
    fn new(input: &str) -> Self {
        let program = input.split(',').map(|num| num.parse().unwrap()).collect();
        Self {
            memory: program,
            ip: 0,
            rb: 0,
        }
    }

    fn next(&mut self, input: bool) -> Option<bool> {
        let mut input = Some(input);
        loop {
            let instruction = self.fetch();
            let (modes, opcode) = instruction.div_rem(&100);
            let (modes, mode_1) = modes.div_rem(&10);
            let (mode_3, mode_2) = modes.div_rem(&10);
            match opcode {
                1 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 + param_2, mode_3);
                }
                2 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 * param_2, mode_3);
                }
                3 => {
                    self.write(input.take().unwrap().into(), mode_1);
                }
                4 => match self.read(mode_1) {
                    0 => return Some(false),
                    1 => return Some(true),
                    output => panic!("unexpected output {output}"),
                },
                5 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    if param_1 != 0 {
                        self.ip = param_2.try_into().unwrap();
                    }
                }
                6 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    if param_1 == 0 {
                        self.ip = param_2.try_into().unwrap();
                    }
                }
                7 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write((param_1 < param_2).into(), mode_3);
                }
                8 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write((param_1 == param_2).into(), mode_3);
                }
                9 => {
                    let param_1 = self.read(mode_1);
                    self.rb += param_1;
                }
                99 => return None,
                _ => panic!("unexpected opcode {opcode}"),
            }
        }
    }

    fn read(&mut self, mode: isize) -> isize {
        let word = self.fetch();
        let i = match mode {
            0 => word,
            1 => return word,
            2 => self.rb + word,
            _ => panic!("unexpected mode {mode}"),
        };
        let u: usize = i.try_into().unwrap();
        self.memory.get(u).copied().unwrap_or(0)
    }

    fn write(&mut self, value: isize, mode: isize) {
        let word = self.fetch();
        let i = match mode {
            0 => word,
            2 => self.rb + word,
            _ => panic!("unexpected mode {mode}"),
        };
        let u: usize = i.try_into().unwrap();
        if u >= self.memory.len() {
            self.memory.resize(u + 1, 0);
        }
        self.memory[u] = value;
    }

    fn fetch(&mut self) -> isize {
        let word = self.memory[self.ip];
        self.ip += 1;
        word
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    #[test_case(INPUT => 2415)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXPECTED_2: &str = " \
 ###  #### ###  #  # #### #  # ###   ##    
 #  # #    #  # #  #    # #  # #  # #  #   
 ###  ###  #  # #  #   #  #  # #  # #      
 #  # #    ###  #  #  #   #  # ###  #      
 #  # #    #    #  # #    #  # #    #  #   
 ###  #    #     ##  ####  ##  #     ##    ";

    #[test_case(INPUT => EXPECTED_2)]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
