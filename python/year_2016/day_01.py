"""
https://adventofcode.com/2016/day/1
"""

import re
from itertools import accumulate, chain, repeat
from operator import mul
from pathlib import Path

from iteration_utilities import duplicates


def process(text, r=re.compile(r"([LR])(\d+)")):
    def turn(direction):
        return 1j if direction == "L" else -1j

    turns, dists = zip(*(r.match(line).groups() for line in text.split(", ")))

    return accumulate(map(turn, turns), mul), map(int, dists)


def manhatten(location):
    return int(abs(location.imag) + abs(location.real))


def part_1(text):
    return manhatten(sum(map(mul, *process(text))))


def part_2(text):
    movements = chain.from_iterable(map(repeat, *process(text)))

    return manhatten(next(duplicates(accumulate(movements, initial=0))))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_01.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 161


def test_part_1_example_1():
    assert part_1("R2, L3") == 5


def test_part_1_example_2():
    assert part_1("R2, R2, R2") == 2


def test_part_1_example_3():
    assert part_1("R5, L5, R5, R3") == 12


def test_part_2_input():
    assert part_2(_read_input()) == 110


def test_part_2_example_1():
    assert part_2("R8, R4, R4, R8") == 4
