"""
https://adventofcode.com/2018/day/3
"""

import re
from collections import defaultdict
from itertools import chain, product, starmap
from pathlib import Path

import pytest
from iteration_utilities import duplicates, empty


def part_1(text):
    def process(line):
        x, y, width, height = map(int, r.match(line).groups())
        return starmap(complex, product(range(x, x + width), range(y, y + height)))

    r = re.compile(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)")

    return len(set(duplicates(chain.from_iterable(map(process, text.splitlines())))))


def part_2(text):
    r = re.compile(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
    seen = defaultdict(list)
    ones = set()

    for line in text.splitlines():
        num, x, y, width, height = map(int, r.match(line).groups())
        ones.add(num)

        for val in starmap(complex, product(range(x, x + width), range(y, y + height))):
            taken = val in seen
            seen[val].append(num)
            ones.difference_update(seen[val] if taken else empty)

    return ones.pop()


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_03.txt").read_text()


EXAMPLE_1 = "#123 @ 3,2: 5x4" ""

EXAMPLE_2 = """#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"""


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input():
    assert part_1(_read_input()) == 121163


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 0


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 4


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == 943


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 123


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 3
