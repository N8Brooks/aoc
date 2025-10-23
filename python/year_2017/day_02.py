"""
https://adventofcode.com/2017/day/2
"""

from itertools import combinations
from pathlib import Path

from iteration_utilities import minmax, rsub


def part_1(text):
    def diff(row):
        return rsub(*minmax(map(int, row.split())))

    return sum(map(diff, text.splitlines()))


def part_2(text):
    def div(row):
        pairs = combinations(sorted(map(int, row.split())), 2)
        return next(b // a for a, b in pairs if b % a == 0)

    return sum(map(div, text.splitlines()))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_02.txt").read_text()


EXAMPLE_1 = """5 1 9 5
7 5 3
2 4 6 8
"""

EXAMPLE_2 = """5 9 2 8
9 4 7 3
3 8 6 5
"""


def test_part_1_input():
    assert part_1(_read_input()) == 51833


def test_part_1_example():
    assert part_1(EXAMPLE_1) == 18


def test_part_2_input():
    assert part_2(_read_input()) == 288


def test_part_2_example():
    assert part_2(EXAMPLE_2) == 9
