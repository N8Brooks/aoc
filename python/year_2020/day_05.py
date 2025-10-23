"""
https://adventofcode.com/2020/day/5
"""

from pathlib import Path

from iteration_utilities import minmax


def location(path, dirs=str.maketrans(dict(B="1", R="1", F="0", L="0"))):
    return int(path.translate(dirs), 2)


def part_1(text):
    return max(map(location, text.splitlines()))


def part_2(text):
    taken = set(map(location, text.splitlines()))
    lo, hi = minmax(taken)

    return taken.symmetric_difference(range(lo, hi + 1)).pop()


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_05.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 953


def test_part_1_example_1():
    assert part_1("FBFBBFFRLR") == 357


def test_part_1_example_2():
    assert part_1("BFFFBBFRRR") == 567


def test_part_1_example_3():
    assert part_1("FFFBBBFRRR") == 119


def test_part_1_example_4():
    assert part_1("BBFFBBFRLL") == 820


def test_part_2_input():
    assert part_2(_read_input()) == 615
