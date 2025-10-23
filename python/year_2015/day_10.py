"""
https://adventofcode.com/2015/day/10
"""

from functools import reduce
from itertools import groupby
from pathlib import Path

import pytest
from iteration_utilities import count_items


def look_say(s, _=None):
    return "".join(f"{count_items(g)}{d}" for d, g in groupby(s))


def process(text, iterations):
    return len(reduce(look_say, range(iterations), text.strip()))


def part_1(text, iterations=40):
    return process(text, iterations)


def part_2(text, iterations=50):
    return process(text, iterations)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_10.txt").read_text()


def test_look_say_example_1():
    assert look_say("1") == "11"


def test_look_say_example_2():
    assert look_say("11") == "21"


def test_look_say_example_3():
    assert look_say("21") == "1211"


def test_look_say_example_4():
    assert look_say("1211") == "111221"


def test_look_say_example_5():
    assert look_say("111221") == "312211"


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input():
    assert part_1(_read_input()) == 252594


def test_part_1_mock_1():
    assert part_1("1234", 10) == 112


def test_part_1_mock_2():
    assert part_1("121", 20) == 750


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == 3579328


def test_part_2_mock_1():
    assert part_2("1", 5) == 6


def test_part_2_mock_2():
    assert part_2("1", 15) == 102
