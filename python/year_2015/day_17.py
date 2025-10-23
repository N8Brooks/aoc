"""
https://adventofcode.com/2015/day/17
"""

from collections import Counter
from functools import reduce
from itertools import combinations
from pathlib import Path

from iteration_utilities import count_items


def part_1(text, target=150):
    def update(freq, b):
        freq += {a + b: count for a, count in freq.items() if a + b <= target}
        return freq

    return reduce(update, map(int, text.split()), Counter((0,))).get(target, 0)


def part_2(text, target=150):
    def target_sum(combo):
        return sum(combo) == target

    def valid_combo(k):
        return count_items(combinations(nums, k), target_sum)

    nums = tuple(map(int, text.split()))

    return next(filter(bool, map(valid_combo, range(len(nums)))))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_17.txt").read_text()


EXAMPLE = "20 15 10 5 5"


def test_part_1_input():
    assert part_1(_read_input()) == 654


def test_part_1_example():
    assert part_1(EXAMPLE, 25) == 4


def test_part_2_input():
    assert part_2(_read_input()) == 57


def test_part_2_example():
    assert part_2(EXAMPLE, 25) == 3
