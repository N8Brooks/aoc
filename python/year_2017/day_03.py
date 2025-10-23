"""
https://adventofcode.com/2017/day/3
"""

from itertools import repeat, takewhile
from math import ceil, sqrt
from operator import add
from pathlib import Path

from iteration_utilities import applyfunc

NEIGHBORS = (-1 + 0j, 1 + 0j, -1 + 1j, 0 + 1j, 1 + 1j, -1 - 1j, 0 - 1j, 1 - 1j)


def part_1(text):
    n = int(text) + 1
    layer = ceil(sqrt(n) / 2 - 0.5)
    return layer + abs(((n - 1) % (2 * layer) - layer)) - 1


def part_2(text):
    def move(c):
        if 0 == c.real == c.imag:
            return 1
        elif -c.real < c.imag:
            return c + 1j if c.imag < c.real else c - 1
        else:
            return c - 1j if c.real < c.imag else c + 1

    n = int(text)
    grid = {(c := 0): 1}

    for c in takewhile(lambda _: grid[c] <= n, applyfunc(move, c)):
        grid[c] = sum(grid.get(x, 0) for x in map(add, repeat(c), NEIGHBORS))

    return grid[c]


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_03.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 480


def test_part_1_example_1():
    assert part_1(1) == 0


def test_part_1_example_2():
    assert part_1(12) == 3


def test_part_1_example_3():
    assert part_1(23) == 2


def test_part_1_example_4():
    assert part_1(1024) == 31


def test_part_2_input():
    assert part_2(_read_input()) == 349975


def test_part_2_example_1():
    assert part_2(0) == 1


def test_part_2_example_2():
    assert part_2(54) == 57


def test_part_2_example_3():
    assert part_2(800) == 806


def test_part_2_example_4():
    assert part_2(1000000) == 1009457
