"""
https://adventofcode.com/2015/day/8
"""

from pathlib import Path


def part_1(text):
    def count(line):
        return len(line) - len(line.encode().decode("unicode_escape")) + 2

    return sum(map(count, text.split()))


def part_2(text):
    def count(line):
        return line.count("\\") + line.count('"') + 2

    return sum(map(count, text.split()))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_08.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 1333


def test_part_1_example_1():
    assert part_1('""') == 2


def test_part_1_example_2():
    assert part_1('"abc"') == 2


def test_part_1_example_3():
    assert part_1('"aaa\\"aaa"') == 3


def test_part_1_example_4():
    assert part_1('"\\x27"') == 5


def test_part_2_input():
    assert part_2(_read_input()) == 2046


def test_part_2_example_1():
    assert part_2('""') == 4


def test_part_2_example_2():
    assert part_2('"abc"') == 4


def test_part_2_example_3():
    assert part_2('"aaa\\"aaa"') == 6


def test_part_2_example_4():
    assert part_2('"\\x27"') == 5
