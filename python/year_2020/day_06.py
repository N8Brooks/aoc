"""
https://adventofcode.com/2020/day/6
"""

from itertools import chain
from pathlib import Path


def part_1(text):
    def any_yes(group):
        return len(set(chain.from_iterable(group.split())))

    return sum(map(any_yes, text.split("\n\n")))


def part_2(text):
    def all_yes(group):
        return len(set.intersection(*map(set, group.split())))

    return sum(map(all_yes, text.split("\n\n")))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_06.txt").read_text()


EXAMPLE_1 = """abcx
abcy
abcz
"""

EXAMPLE_2 = """abc

a
b
c

ab
ac

a
a
a
a

b
"""

EXAMPLE_3 = """abc

a
b
c

ab
ac

a
a
a
a

b
"""


def test_part_1_input():
    assert part_1(_read_input()) == 6532


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 6


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 11


def test_part_1_example_3():
    assert part_1(EXAMPLE_3) == 11


def test_part_2_input():
    assert part_2(_read_input()) == 3427


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 3


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 6


def test_part_2_example_3():
    assert part_2(EXAMPLE_3) == 6
