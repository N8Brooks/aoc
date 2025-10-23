"""
https://adventofcode.com/2015/day/3
"""


from pathlib import Path
from itertools import accumulate, islice

def direction(char):
    if char == "^":
        return 1j
    elif char == ">":
        return 1
    elif char == "v":
        return -1j
    elif char == "<":
        return -1


def part_1(text):
    return len(set({0} | set(accumulate(map(direction, text)))))


def part_2(text):
    locations = {0}
    locations.update(accumulate(map(direction, islice(text, 0, None, 2))))
    locations.update(accumulate(map(direction, islice(text, 1, None, 2))))

    return len(locations)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_03.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 2081

def test_part_1_example_1():
    assert part_1('>') == 2

def test_part_1_example_2():
    assert part_1('^>v<') == 4

def test_part_1_example_3():
    assert part_1('^v^v^v^v^v') == 2

def test_part_2_input():
    assert part_2(_read_input()) == 2341

def test_part_2_example_1():
    assert part_2('^v') == 3

def test_part_2_example_2():
    assert part_2('^>v<') == 3

def test_part_2_example_3():
    assert part_2('^v^v^v^v^v') == 11
