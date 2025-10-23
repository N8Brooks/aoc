"""
https://adventofcode.com/2015/day/13
"""

import re
from collections import defaultdict
from functools import cache
from pathlib import Path


def family(text, buffer=False):
    @cache
    def happiness(i, s):
        def add(j):
            return pairs[(1 << i) | (1 << j)] + happiness(j, s | (1 << j))

        if s == (1 << n) - 1:
            return pairs[1 | (1 << i)]

        return max(add(j) for j in range(n) if not (s & (1 << j)))

    r = re.compile(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).")

    individuals = {}
    pairs = defaultdict(int)

    for line in text.splitlines():
        a, sign, val, b = r.match(line).groups()
        a = individuals.setdefault(a, len(individuals))
        b = individuals.setdefault(b, len(individuals))
        pairs[(1 << a) | (1 << b)] += int(val) if sign == "gain" else -int(val)

    n = len(individuals) + buffer

    return happiness(0, 1)


def part_1(text):
    return family(text)


def part_2(text):
    return family(text, True)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_13.txt").read_text()


EXAMPLE = """Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
"""

MOCK = """Jody would loose 200 happiness units by sitting next to Carol.
Jody would loose 100 happiness units by sitting next to Irwin.
Jody would loose 1000 happiness units by sitting next to Aligator.
Carol would loose 42 happiness units by sitting next to Jody.
Carol would gain 512 happiness units by sitting next to Irwin.
Carol would loose 300 happiness units by sitting next to Aligator.
Irwin would loose 30 happiness units by sitting next to Jody.
Irwin would gain 101 happiness units by sitting next to Carol.
Irwin would gain 499 happiness units by sitting next to Aligator.
Aligator would gain 999 happiness units by sitting next to Jody.
Aligator would gain 1500 happiness units by sitting next to Carol.
Aligator would loose 360 happiness units by sitting next to Irwin.
"""


def test_part_1_input():
    assert part_1(_read_input()) == 709


def test_part_1_example():
    assert part_1(EXAMPLE) == 330


def test_part_1_mock():
    assert part_1(MOCK) == 1682


def test_part_2_input():
    assert part_2(_read_input()) == 668


def test_part_2_example():
    assert part_2(EXAMPLE) == 286


def test_part_2_mock():
    assert part_2(MOCK) == 1812
