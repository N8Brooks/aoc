"""
https://adventofcode.com/2016/day/16
"""

from pathlib import Path

import pytest
from iteration_utilities import applyfunc, grouper


def process(text, length):
    def dragon(data, translation=str.maketrans({"0": "1", "1": "0"})):
        return f"{data}0{data[::-1].translate(translation)}"

    def valid(data):
        return length <= len(data)

    def checksum(data):
        return "".join("1" if a == b else "0" for a, b in grouper(data, 2))

    data = next(filter(valid, applyfunc(dragon, text.strip())))[:length]

    return next(filter(lambda data: len(data) % 2, applyfunc(checksum, data)))


def part_1(text, length=272):
    return process(text, length)


def part_2(text, length=35651584):
    return process(text, length)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_16.txt").read_text()


EXAMPLE = """rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
"""


def test_part_1_input():
    assert part_1(_read_input()) == "10100011010101011"


def test_part_1_example_1():
    assert part_1("1", 10) == "01110"


def test_part_1_example_2():
    assert part_1("0", 10) == "10001"


def test_part_1_example_3():
    assert part_1("11111", 10) == "11011"


def test_part_1_example_4():
    assert part_1("111100001010", 10) == "11110"


def test_part_1_example_5():
    assert part_1("10000", 10) == "01111"


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == "01010001101011001"


def test_part_2_example_1():
    assert part_2("1", 20) == "01110"


def test_part_2_example_2():
    assert part_2("0", 20) == "01110"


def test_part_2_example_3():
    assert part_2("11111", 20) == "10111"


def test_part_2_example_4():
    assert part_2("111100001010", 20) == "11110"


def test_part_2_example_5():
    assert part_2("10000", 20) == "01100"
