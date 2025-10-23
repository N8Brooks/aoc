"""
https://adventofcode.com/2017/day/11
"""


from pathlib import Path
from itertools import accumulate

DIRS = {
    "nw": 1 - 0j,
    "n": 0 + 1j,
    "ne": -1 + 1j,
    "se": -1 + 0j,
    "s": 0 - 1j,
    "sw": 1 - 1j,
}


def dist(c):
    return int((abs(c.imag) + abs(c.imag + c.real) + abs(c.real)) // 2)


def part_1(text):
    return dist(sum(map(DIRS.get, text.strip().split(","))))


def part_2(text):
    return max(map(dist, accumulate(map(DIRS.get, text.strip().split(",")))))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_11.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 784

def test_part_1_example_1():
    assert part_1('ne,ne,ne') == 3

def test_part_1_example_2():
    assert part_1('ne,ne,sw,sw') == 0

def test_part_1_example_3():
    assert part_1('ne,ne,s,s') == 2

def test_part_1_example_4():
    assert part_1('se,sw,se,sw,sw') == 3

def test_part_2_input():
    assert part_2(_read_input()) == 1558

def test_part_2_example_1():
    assert part_2('ne,ne,ne') == 3

def test_part_2_example_2():
    assert part_2('ne,ne,sw,sw') == 2

def test_part_2_example_3():
    assert part_2('ne,ne,s,s') == 2

def test_part_2_example_4():
    assert part_2('se,sw,se,sw,sw') == 3
