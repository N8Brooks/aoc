use std::{mem, str};

pub fn part_1(input: &str) -> usize {
    look_and_say(input, 40)
}

pub fn part_2(input: &str) -> usize {
    look_and_say(input, 50)
}

fn look_and_say(input: &str, num: usize) -> usize {
    let mut input = input.to_string();
    let mut output = String::new();
    for _ in 0..num {
        step_into(&input, &mut output);
        mem::swap(&mut input, &mut output);
    }
    input.len()
}

fn step_into(input: &str, output: &mut String) {
    output.clear();
    output.reserve(input.len() * 2);
    let mut bytes = input.bytes().peekable();
    while let Some(b1) = bytes.next() {
        let mut count = 1;
        while let Some(&b2) = bytes.peek() {
            if b2 != b1 {
                break;
            }
            count += 1;
            bytes.next();
        }
        push_count(output, count);
        output.push(b1 as char);
    }
}

fn push_count(out: &mut String, mut n: usize) {
    debug_assert_ne!(n, 0);
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    let s = unsafe { str::from_utf8_unchecked(&buf[i..]) };
    out.push_str(s);
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    #[test_case("1" => "11")]
    #[test_case("11" => "21")]
    #[test_case("21" => "1211")]
    #[test_case("1211" => "111221")]
    #[test_case("111221" => "312211")]
    fn step(input: &str) -> String {
        let mut out = String::new();
        super::step_into(input, &mut out);
        out
    }

    #[test_case(INPUT => 252594)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 3579328)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
