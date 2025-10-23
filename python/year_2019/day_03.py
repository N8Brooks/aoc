"""
https://adventofcode.com/2019/day/3
"""

import re
from itertools import accumulate, repeat
from operator import add
from pathlib import Path

DIRS = {"U": 0 + 1j, "R": 1 + 0j, "D": 0 - 1j, "L": -1 + 0j}


def path(wire, r=re.compile(r"([URDL])(\d+)")):
    location = 0 + 0j
    for direction, distance in r.findall(wire):
        direction, distance = DIRS[direction], int(distance)
        start = location + direction
        moves = repeat(direction, distance - 1)
        yield from accumulate(moves, initial=start)
        location += direction * distance


def part_1(text):
    def manhatten(location):
        return abs(location.imag) + abs(location.real)

    a, b = map(path, text.split())
    common = frozenset(a).intersection(b)

    return int(min(map(manhatten, common)))


def part_2(text):
    def steps(wire):
        locations = {}
        for i, location in enumerate(path(wire), start=1):
            locations.setdefault(location, i)
        return locations

    a, b = map(steps, text.split())
    common = tuple(a.keys() & b.keys())

    return min(map(add, map(a.get, common), map(b.get, common)))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_03.txt").read_text()


EXAMPLE_1 = """
R8,U5,L5,D3
U7,R6,D4,L4
"""

EXAMPLE_2 = """
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
"""

EXAMPLE_3 = """
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
"""


def test_part_1_input():
    assert part_1(_read_input()) == 1674


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 6


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 159


def test_part_1_example_3():
    assert part_1(EXAMPLE_3) == 135


def test_part_2_input():
    assert part_2(_read_input()) == 14012


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 30


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 610


def test_part_2_example_3():
    assert part_2(EXAMPLE_3) == 410
