"""
https://adventofcode.com/2018/day/1
"""

from itertools import accumulate, cycle
from pathlib import Path

from iteration_utilities import duplicates


def part_1(text):
    return sum(map(int, text.split()))


def part_2(text):
    return next(duplicates(accumulate(cycle(map(int, text.split())), initial=0)))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_01.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 599


def test_part_1_example_1():
    assert part_1("+1 -2 +3 +1") == 3


def test_part_1_example_2():
    assert part_1("+1 +1 +1") == 3


def test_part_1_example_3():
    assert part_1("+1 +1 -2") == 0


def test_part_1_example_4():
    assert part_1("-1 -2 -3") == -6


def test_part_2_input():
    assert part_2(_read_input()) == 81204


def test_part_2_example_1():
    assert part_2("+1 -1") == 0


def test_part_2_example_2():
    assert part_2("+3 +3 +4 -2 -4") == 10


def test_part_2_example_3():
    assert part_2("-6 +3 +8 +5 -6") == 5


def test_part_2_example_4():
    assert part_2("+7 +7 -2 -7 -4") == 14
