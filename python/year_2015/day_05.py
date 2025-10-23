"""
https://adventofcode.com/2015/day/5
"""

from itertools import islice, starmap
from operator import eq, ne
from pathlib import Path

from iteration_utilities import successive


def part_1(text):
    def nice(string):
        if any(two in string for two in ("ab", "cd", "pq", "xy")):
            return False

        if all(starmap(ne, successive(string))):
            return False

        return 3 <= sum(map(string.count, "aeiou"))

    return sum(map(nice, text.split()))


def part_2(text):
    def split_pair(string, start):
        return any(starmap(eq, successive(islice(string, start, None, 2))))

    def twin_pair(first, second):
        if second in seen:
            return True
        seen.add(first)

    def nice(string):
        if not split_pair(string, 0) and not split_pair(string, 1):
            return False

        seen.clear()

        return any(starmap(twin_pair, successive(successive(string))))

    seen = set()

    return sum(map(nice, text.split()))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_05.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 238


def test_part_1_example_1():
    assert part_1("ugknbfddgicrmopn") == 1


def test_part_1_example_2():
    assert part_1("aaa") == 1


def test_part_1_example_3():
    assert part_1("jchzalrnumimnmhp") == 0


def test_part_1_example_4():
    assert part_1("haegwjzuvuyypxyu") == 0


def test_part_1_example_5():
    assert part_1("dvszwmarrgswjxmb") == 0


def test_part_2_input():
    assert part_2(_read_input()) == 69


def test_part_2_example_1():
    assert part_2("qjhvhtzxzqqjkmpb") == 1


def test_part_2_example_2():
    assert part_2("xxyxx") == 1


def test_part_2_example_3():
    assert part_2("uurcxstgmygtbstg") == 0


def test_part_2_example_4():
    assert part_2("ieodomkazucvgmuy") == 0
