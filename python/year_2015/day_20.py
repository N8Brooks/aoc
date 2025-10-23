"""
https://adventofcode.com/2015/day/20
"""


from pathlib import Path
import pytest
import numpy as np

# TODO: there are better solutions than brute force


def part_1(text):
    n = int(text)
    houses = np.full(n // 10 + 2, 10, int)
    houses[0] = 0
    for i in range(2, n // 10 + 2):
        houses[i::i] += 10 * i
    return np.argmax(houses >= n)


def part_2(text):
    n = int(text)
    houses = np.zeros(n // 10 + 2, int)
    for i in range(1, n // 10 + 2):
        houses[i : i * 50 + 1 : i] += 11 * i
    return np.argmax(houses >= n)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_20.txt").read_text()


@pytest.mark.skip(reason='Takes too long')
def test_part_1_input():
    assert part_1(_read_input()) == 776160

def test_part_1_example_1():
    assert part_1(10) == 1

def test_part_1_example_2():
    assert part_1(70) == 4

def test_part_1_example_3():
    assert part_1(111) == 6

def test_part_1_example_4():
    assert part_1(169) == 10

@pytest.mark.skip(reason='Takes too long')
def test_part_2_input():
    assert part_2(_read_input()) == 786240

def test_part_2_example_1():
    assert part_2(8) == 1

def test_part_2_example_2():
    assert part_2(64) == 4

def test_part_2_example_3():
    assert part_2(133) == 8

def test_part_2_example_4():
    assert part_2(820) == 36
