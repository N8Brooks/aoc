use std::sync::LazyLock;

use hashbrown::HashSet;

use crate::day_04::Unit::*;

pub fn part_1(input: &str) -> usize {
    static REQUIRED_FIELDS: LazyLock<HashSet<&'static str>> =
        LazyLock::new(|| HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]));
    input
        .split("\n\n")
        .filter(|passport| {
            let fields = passport
                .split_ascii_whitespace()
                .map(|field| field.split_once(':').map(|(key, _)| key))
                .try_collect()
                .unwrap();
            REQUIRED_FIELDS.is_subset(&fields)
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|&lines| matches!(lines.try_into().unwrap(), Passport {
            byr: Some(1920..=2002),
            iyr: Some(2010..=2020),
            eyr: Some(2020..=2030),
            hgt: Some((150..=193, Cm) | (59..=76, In)),
            hcl: Some([b'#', b'0'..=b'9' | b'a'..=b'f', b'0'..=b'9' | b'a'..=b'f', b'0'..=b'9' | b'a'..=b'f', b'0'..=b'9' | b'a'..=b'f', b'0'..=b'9' | b'a'..=b'f', b'0'..=b'9' | b'a'..=b'f']),
            ecl: Some(b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth"),
            pid: Some([b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9']),
            cid: _,
        }))
        .count()
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Passport<'a> {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<(u16, Unit)>,
    hcl: Option<&'a [u8]>,
    ecl: Option<&'a [u8]>,
    pid: Option<&'a [u8]>,
    cid: Option<u16>,
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = &'static str;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut passport = Passport::default();

        for field in s.split_ascii_whitespace() {
            let (key, value) = field.split_once(':').ok_or("missing ':'")?;

            match key {
                "byr" => passport.byr = Some(value.parse().map_err(|_| "invalid byr")?),
                "iyr" => passport.iyr = Some(value.parse().map_err(|_| "invalid iyr")?),
                "eyr" => passport.eyr = Some(value.parse().map_err(|_| "invalid eyr")?),
                "hgt" => {
                    let (value, unit) = value
                        .strip_suffix("cm")
                        .map(|v| (v, Cm))
                        .or_else(|| value.strip_suffix("in").map(|v| (v, In)))
                        .unwrap_or((value, Na));
                    let value = value.parse().map_err(|_| "invalid hgt")?;
                    passport.hgt = Some((value, unit));
                }
                "hcl" => passport.hcl = Some(value.as_bytes()),
                "ecl" => passport.ecl = Some(value.as_bytes()),
                "pid" => passport.pid = Some(value.as_bytes()),
                "cid" => passport.cid = Some(value.parse().map_err(|_| "invalid cid")?),
                _ => return Err("unknown field"),
            }
        }

        Ok(passport)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Cm,
    In,
    Na,
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    const EXAMPLE_1: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test_case(EXAMPLE_1 => 2)]
    #[test_case(INPUT => 190)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const EXAMPLE_3: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test_case(EXAMPLE_2 => 0)]
    #[test_case(EXAMPLE_3 => 4)]
    #[test_case(INPUT => 121)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
