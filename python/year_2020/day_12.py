"""
https://adventofcode.com/2020/day/12
"""

import re
from pathlib import Path

R = re.compile(r"([NESWLRF])(\d+)")

DIRS = {"N": 0 + 1j, "E": 1 + 0j, "S": 0 - 1j, "W": -1 + 0j}


def manhatten(c):
    return int(abs(c.real) + abs(c.imag))


def part_1(text):
    def move(line):
        nonlocal direction
        action, value = R.match(line).groups()

        if action == "L":
            direction *= pow(1j, int(value) // 90)
        elif action == "R":
            direction *= pow(-1j, int(value) // 90)
        elif action == "F":
            return direction * int(value)
        else:
            return DIRS[action] * int(value)

        return 0 + 0j

    direction = 1 + 0j

    return manhatten(sum(map(move, text.splitlines())))


def part_2(text, size=25):
    def move(line):
        nonlocal waypoint
        action, value = R.match(line).groups()

        if action == "L":
            waypoint *= pow(1j, int(value) // 90)
        elif action == "R":
            waypoint *= pow(-1j, int(value) // 90)
        elif action == "F":
            return waypoint * int(value)
        else:
            waypoint += DIRS[action] * int(value)

        return 0 + 0j

    waypoint = 10 + 1j

    return manhatten(sum(map(move, text.splitlines())))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_12.txt").read_text()


EXAMPLE = """F10
N3
F7
R90
F11
"""


def test_part_1_input():
    assert part_1(_read_input()) == 2847


def test_part_1_example():
    assert part_1(EXAMPLE) == 25


def test_part_2_input():
    assert part_2(_read_input()) == 29839


def test_part_2_example():
    assert part_2(EXAMPLE) == 286
