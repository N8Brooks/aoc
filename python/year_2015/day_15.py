"""
https://adventofcode.com/2015/day/15
"""


from pathlib import Path
import pytest
from itertools import repeat
import re

import numpy as np

R = re.compile(
    (
        r"(?:\w+): capacity (-?\d+), durability (-?\d+), "
        r"flavor (-?\d+), texture (-?\d+), calories (-?\d+)"
    )
)


def process(text):
    processed = [R.match(line).groups() for line in text.splitlines()]

    return np.array(processed, int).transpose()


def allocation(n, remaining=100):
    if (n := n - 1) == 0:
        yield [remaining]
        return

    for right in range(remaining + 1):
        for left in allocation(n, remaining - right):
            left.append(right)
            yield left


def score(allocations, matrix):
    totals = np.multiply(matrix, allocations).sum(axis=1)
    return np.clip(totals, 0, None).prod()


def part_1(text):
    matrix = process(text)[:-1]

    return max(map(score, allocation(matrix.shape[1]), repeat(matrix)))


def part_2(text, target=500):
    def valid(allocations):
        return (calories * allocations).sum() == target

    *matrix, calories = process(text)
    matrix = np.vstack(matrix)

    filtered = filter(valid, allocation(matrix.shape[1]))

    return max(map(score, filtered, repeat(matrix)))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_15.txt").read_text()


EXAMPLE = """Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"""

MOCK = """Sugar: capacity 2, durability 1, flavor 5, texture 1, calories 4
Capsaicin: capacity 1, durability 5, flavor -5, texture 3, calories 1
Butter: capacity 1, durability 3, flavor 3, texture 3, calories 9
"""


@pytest.mark.skip(reason='Takes too long')
def test_part_1_input():
    assert part_1(_read_input()) == 18965440

def test_part_1_example():
    assert part_1(EXAMPLE) == 62842880

@pytest.mark.skip(reason='Takes too long')
def test_part_1_mock():
    assert part_1(MOCK) == 2766555000

@pytest.mark.skip(reason='Takes too long')
def test_part_2_input():
    assert part_2(_read_input()) == 2766555000

def test_part_2_example():
    assert part_2(EXAMPLE) == 57600000

def test_part_2_mock():
    assert part_2(MOCK) == 1884745728
