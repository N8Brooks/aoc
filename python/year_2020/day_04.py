"""
https://adventofcode.com/2020/day/4
"""

import re
from pathlib import Path

from iteration_utilities import count_items

FIELDS = frozenset(("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"))

ECL = frozenset(("amb", "blu", "brn", "gry", "grn", "hzl", "oth"))


def part_1(text):
    def valid(record):
        return FIELDS.issubset(field.split(":")[0] for field in record.split())

    return count_items(text.split("\n\n"), valid)


def part_2(text):
    def starred(func):
        def wrapper(record):
            return func(**dict(field.split(":") for field in record.split()))

        return wrapper

    def height(hgt):
        if hgt.endswith("cm"):
            return 150 <= int(hgt.removesuffix("cm")) <= 193
        elif hgt.endswith("in"):
            return 59 <= int(hgt.removesuffix("in")) <= 76

    @starred
    def valid(byr=0, iyr=0, eyr=0, hgt="", hcl="", ecl="", pid="", cid=None):
        if not 1920 <= int(byr) <= 2002:
            return False
        elif not 2010 <= int(iyr) <= 2020:
            return False
        elif not 2020 <= int(eyr) <= 2030:
            return False
        elif not height(hgt):
            return False
        elif not re.match(r"#[0-9a-f]{6}", hcl):
            return False
        elif ecl not in ECL:
            return False
        elif not len(pid) == 9 or not pid.isdecimal():
            return False

        return True

    return count_items(text.split("\n\n"), valid)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_04.txt").read_text()


EXAMPLE_1 = """ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"""

EXAMPLE_2 = """eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"""

EXAMPLE_3 = """pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"""


def test_part_1_input():
    assert part_1(_read_input()) == 190


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 2


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 4


def test_part_1_example_3():
    assert part_1(EXAMPLE_3) == 4


def test_part_2_input():
    assert part_2(_read_input()) == 121


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 2


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 0


def test_part_2_example_3():
    assert part_2(EXAMPLE_3) == 4
