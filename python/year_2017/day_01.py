"""
https://adventofcode.com/2017/day/1
"""

from pathlib import Path


def process(func):
    def wrapper(text):
        return func(tuple(map(int, text.strip())))

    return wrapper


@process
def part_1(processed):
    def match(i):
        return processed[i] == processed[i - 1]

    return sum(processed[i] for i in filter(match, range(len(processed))))


@process
def part_2(processed):
    def match(i, halfway=len(processed) // 2):
        return processed[i] == processed[i - halfway]

    return sum(processed[i] for i in filter(match, range(len(processed))))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_01.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 1136


def test_part_1_example_1():
    assert part_1("1122") == 3


def test_part_1_example_2():
    assert part_1("1111") == 4


def test_part_1_example_3():
    assert part_1("1234") == 0


def test_part_1_example_4():
    assert part_1("91212129") == 9


def test_part_2_input():
    assert part_2(_read_input()) == 1092


def test_part_2_example_1():
    assert part_2("1212") == 6


def test_part_2_example_2():
    assert part_2("1221") == 0


def test_part_2_example_3():
    assert part_2("123425") == 4


def test_part_2_example_4():
    assert part_2("123123") == 12


def test_part_2_example_5():
    assert part_2("12131415") == 4
