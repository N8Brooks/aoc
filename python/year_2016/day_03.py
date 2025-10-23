"""
https://adventofcode.com/2016/day/3
"""

from itertools import chain
from pathlib import Path

from more_itertools import ichunked


def total(iterable):
    return sum(tri[2] < tri[0] + tri[1] for tri in map(sorted, iterable))


def process(text):
    return (map(int, tri.split()) for tri in text.splitlines())


def part_1(text):
    return total(process(text))


def part_2(text):
    return total(ichunked(chain.from_iterable(zip(*process(text))), 3))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_03.txt").read_text()


EXAMPLE = """101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603
"""


def test_part_1_input():
    assert part_1(_read_input()) == 869


def test_part_1_example_1():
    assert part_1("5 10 25") == 0


def test_part_1_example_2():
    assert part_1(EXAMPLE) == 3


def test_part_2_input():
    assert part_2(_read_input()) == 1544


def test_part_2_example_1():
    assert part_2("5 10 25") == 0


def test_part_2_example_2():
    assert part_2(EXAMPLE) == 6
