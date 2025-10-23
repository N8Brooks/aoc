"""
https://adventofcode.com/2015/day/24
"""


from pathlib import Path
from math import prod

from iteration_utilities import powerset

def partition(text, k):
    target = sum(nums := tuple(map(int, text.split()))) // k

    return prod(next(group for group in powerset(nums) if target == sum(group)))


def part_1(text, k=3):
    return partition(text, k)


def part_2(text, k=4):
    return partition(text, k)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_24.txt").read_text()


EXAMPLE = "1 2 3 4 5 7 8 9 10 11"


def test_part_1_input():
    assert part_1(_read_input()) == 10439961859

def test_part_1_example():
    assert part_1(EXAMPLE) == 99

def test_part_2_input():
    assert part_2(_read_input()) == 72050269

def test_part_2_example_a():
    assert part_2(EXAMPLE) == 44
