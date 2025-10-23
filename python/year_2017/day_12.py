"""
https://adventofcode.com/2017/day/12
"""

from pathlib import Path

from iteration_utilities import count_items


def unite(text):
    def find(a):
        return a if a == parent.setdefault(a, a) else find(parent[a])

    parent = {}

    for line in text.splitlines():
        a, bs = line.split(" <-> ")
        parent.update(dict.fromkeys(map(find, bs.split(", ")), find(a)))

    return parent, find


def part_1(text, group="0"):
    parent, find = unite(text)

    return count_items(map(find, parent.values()), find(group), True)


def part_2(text):
    parent, find = unite(text)

    return len(set(map(find, parent.values())))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_12.txt").read_text()


EXAMPLE = """0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
"""


def test_part_1_input():
    assert part_1(_read_input()) == 306


def test_part_1_example():
    assert part_1(EXAMPLE) == 6


def test_part_2_input():
    assert part_2(_read_input()) == 200


def test_part_2_example():
    assert part_2(EXAMPLE) == 2
