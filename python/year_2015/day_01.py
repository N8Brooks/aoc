"""
https://adventofcode.com/2015/day/1
"""

from itertools import accumulate
from pathlib import Path


def part_1(text):
    return text.count("(") - text.count(")")


def part_2(text):
    def direction(char):
        return 1 if char == "(" else -1

    position_floor = enumerate(accumulate(map(direction, text)), start=1)

    return next(i for i, floor in position_floor if floor < 0)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_01.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 232


def test_part_1_example_1():
    assert part_1("(())") == 0


def test_part_1_example_2():
    assert part_1("()()") == 0


def test_part_1_example_3():
    assert part_1("(((") == 3


def test_part_1_example_4():
    assert part_1("(()(()(") == 3


def test_part_1_example_5():
    assert part_1("))(((((") == 3


def test_part_1_example_6():
    assert part_1("())") == -1


def test_part_1_example_7():
    assert part_1("))(") == -1


def test_part_1_example_8():
    assert part_1(")))") == -3


def test_part_1_example_9():
    assert part_1(")())())") == -3


def test_part_2_input():
    assert part_2(_read_input()) == 1783


def test_part_2_example_1():
    assert part_2(")") == 1


def test_part_2_example_2():
    assert part_2("()())") == 5
