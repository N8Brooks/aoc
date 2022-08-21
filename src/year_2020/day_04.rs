#[derive(Default)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<Hgt>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn new(input: &str) -> Passport {
        let mut passport = Passport::default();
        for entry in input.split_whitespace() {
            let (key, val) = entry.split_once(':').unwrap();
            match key {
                "byr" => passport.byr = Some(val.parse().unwrap()),
                "iyr" => passport.iyr = Some(val.parse().unwrap()),
                "eyr" => passport.eyr = Some(val.parse().unwrap()),
                "hgt" => passport.hgt = Some(Hgt::new(val)),
                "hcl" => passport.hcl = Some(val.to_string()),
                "ecl" => passport.ecl = Some(val.to_string()),
                "pid" => passport.pid = Some(val.to_string()),
                "cid" => (),
                _ => panic!("Unexpected key"),
            }
        }
        passport
    }

    fn is_valid_part_1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_part_2<'a>(&'a self) -> bool {
        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref ECL_RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        // TODO: self.byr.is_some_and() ...
        return self.byr.is_some()
            && (1920..=2002).contains(&self.byr.unwrap())
            && self.iyr.is_some()
            && (2010..=2020).contains(&self.iyr.unwrap())
            && self.eyr.is_some()
            && (2020..=2030).contains(&self.eyr.unwrap())
            && match self.hgt {
                Some(Hgt::Cm(x)) => (150..=193).contains(&x),
                Some(Hgt::In(x)) => (59..=76).contains(&x),
                _ => false,
            }
            && self.hcl.is_some()
            && HCL_RE.is_match(self.hcl.as_ref().unwrap())
            && self.ecl.is_some()
            && ECL_RE.is_match(self.ecl.as_ref().unwrap())
            && self.pid.is_some()
            && PID_RE.is_match(self.pid.as_ref().unwrap());
    }
}

enum Hgt {
    Cm(u16),
    In(u16),
    Invalid,
}

impl Hgt {
    fn new(input: &str) -> Hgt {
        if input.ends_with("in") {
            let length = input.len() - 2;
            Hgt::In(input[..length].parse().unwrap())
        } else if input.ends_with("cm") {
            let length = input.len() - 2;
            Hgt::Cm(input[..length].parse().unwrap())
        } else {
            Hgt::Invalid
        }
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::new)
        .filter(|passport| passport.is_valid_part_1())
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::new)
        .filter(|passport| passport.is_valid_part_2())
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static EXAMPLE_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    static EXAMPLE_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_04.txt").unwrap();
    }

    #[test_case(EXAMPLE, 2)]
    #[test_case(EXAMPLE_VALID, 4)]
    #[test_case(EXAMPLE_INVALID, 4)]
    #[test_case(&INPUT, 190)]
    fn part_1_examples(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 2)]
    #[test_case(EXAMPLE_VALID, 4)]
    #[test_case(EXAMPLE_INVALID, 0)]
    #[test_case(&INPUT, 121)]
    fn part_2_examples(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected);
    }
}
