"""
https://adventofcode.com/2019/day/1
"""

from itertools import repeat, takewhile
from pathlib import Path


def part_1(text):
    return sum(int(mass) // 3 - 2 for mass in text.split())


def part_2(text):
    def fuel(mass):
        fuels = (mass := mass // 3 - 2 for _ in repeat(None))
        return sum(takewhile(lambda mass: 0 < mass, fuels))

    return sum(map(fuel, map(int, text.split())))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_01.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 3408471


def test_part_1_example_1():
    assert part_1("12") == 2


def test_part_1_example_2():
    assert part_1("14") == 2


def test_part_1_example_3():
    assert part_1("1969") == 654


def test_part_1_example_4():
    assert part_1("100756") == 33583


def test_part_2_input():
    assert part_2(_read_input()) == 5109803


def test_part_2_example_1():
    assert part_2("14") == 2


def test_part_2_example_2():
    assert part_2("1969") == 966


def test_part_2_example_3():
    assert part_2("100756") == 50346
