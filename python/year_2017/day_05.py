"""
https://adventofcode.com/2017/day/5
"""

from itertools import takewhile
from pathlib import Path

import pytest
from iteration_utilities import applyfunc, count_items


def part_1(text):
    def jump(i):
        instructions[i] += 1
        return i + instructions[i] - 1

    n = len(instructions := list(map(int, text.split())))

    return count_items(takewhile(lambda i: i < n, applyfunc(jump, 0))) + 1


def part_2(text):
    def jump(i):
        jumps = instructions[i]
        instructions[i] += 1 if jumps < 3 else -1
        return i + jumps

    n = len(instructions := list(map(int, text.split())))

    return count_items(takewhile(lambda i: i < n, applyfunc(jump, 0))) + 1


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_05.txt").read_text()


EXAMPLE = "0 3 0 1 -3"

MOCK_1 = "1 2 -2 -3 0"

MOCK_2 = "2 3 -1 0 -4"

MOCK_3 = "0 -1 -2 1 0"


def test_part_1_input():
    assert part_1(_read_input()) == 325922


def test_part_1_example_1():
    assert part_1(EXAMPLE) == 5


def test_part_1_mock_1():
    assert part_1(MOCK_1) == 10


def test_part_1_mock_2():
    assert part_1(MOCK_2) == 9


def test_part_1_mock_3():
    assert part_1(MOCK_3) == 9


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == 24490906


def test_part_2_example_1():
    assert part_2(EXAMPLE) == 10


def test_part_2_mock_1():
    assert part_2(MOCK_1) == 10


def test_part_2_mock_2():
    assert part_2(MOCK_2) == 10


def test_part_2_mock_3():
    assert part_2(MOCK_3) == 9
