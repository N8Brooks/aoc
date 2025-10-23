"""
https://adventofcode.com/2020/day/1
"""

from operator import mul
from pathlib import Path


def part_1(text, total=2020):
    def add(entry):
        if total - entry in entries:
            return True
        entries.add(entry)

    entries = set()

    return next((total - x) * x for x in sorted(map(int, text.split())) if add(x))


def part_2(text, total=2020):
    def add(entry):
        if total - entry in twos:
            return True
        twos.update((entry + other, (entry, other)) for other in ones)
        ones.add(entry)

    def fix(entry):
        return mul(*twos[total - entry]) * entry

    ones = set()
    twos = {}

    return next(fix(x) for x in sorted(map(int, text.split())) if add(x))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_01.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 482811


def test_part_1_example():
    assert part_1("1721 979 366 299 675 1456") == 514579


def test_part_2_input():
    assert part_2(_read_input()) == 193171814


def test_part_2_example():
    assert part_2("1721 979 366 299 675 1456") == 241861950
