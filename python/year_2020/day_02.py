"""
https://adventofcode.com/2020/day/2
"""

import re
from pathlib import Path


def part_1(text):
    def valid(line):
        lo, hi, char, password = r.match(line).groups()
        return int(lo) <= password.count(char) <= int(hi)

    r = re.compile(r"(\d+)-(\d+) (\w): (\w+)")

    return sum(map(valid, text.splitlines()))


def part_2(text):
    def valid(line):
        lo, hi, char, password = r.match(line).groups()
        return (char == password[int(lo) - 1]) != (char == password[int(hi) - 1])

    r = re.compile(r"(\d+)-(\d+) (\w): (\w+)")

    return sum(map(valid, text.splitlines()))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_02.txt").read_text()


EXAMPLE = """1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"""


def test_part_1_input():
    assert part_1(_read_input()) == 591


def test_part_1_example():
    assert part_1(EXAMPLE) == 2


def test_part_2_input():
    assert part_2(_read_input()) == 335


def test_part_2_example():
    assert part_2(EXAMPLE) == 1
