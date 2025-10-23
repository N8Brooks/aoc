"""
https://adventofcode.com/2016/day/2
"""

from functools import reduce
from itertools import accumulate, islice
from pathlib import Path

DIRS = {"L": -1, "U": 1j, "R": 1, "D": -1j}

KEYPAD_A = {
    -1 + 1j: "1",
    0 + 1j: "2",
    1 + 1j: "3",
    -1 + 0j: "4",
    0 + 0j: "5",
    1 + 0j: "6",
    -1 - 1j: "7",
    0 - 1j: "8",
    1 - 1j: "9",
}

KEYPAD_B = {
    0 + 2j: "1",
    -1 + 1j: "2",
    0 + 1j: "3",
    1 + 1j: "4",
    -2 + 0j: "5",
    -1 + 0j: "6",
    0 + 0j: "7",
    1 + 0j: "8",
    2 + 0j: "9",
    -1 - 1j: "A",
    0 - 1j: "B",
    1 - 1j: "C",
    0 - 2j: "D",
}


def simulate(text, keys, start):
    def act(cur, move):
        nxt = cur + DIRS[move]
        return nxt if nxt in keys else cur

    def row(loc, moves):
        return reduce(act, moves, loc)

    keycode = islice(accumulate(text.split(), row, initial=start), 1, None)

    return "".join(map(keys.get, keycode))


def part_1(text, initial=0 + 0j):
    return simulate(text, KEYPAD_A, initial)


def part_2(text, initial=-2 + 0j):
    return simulate(text, KEYPAD_B, initial)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_02.txt").read_text()


EXAMPLE = """ULL
RRDDD
LURDL
UUUUD
"""


def test_part_1_input():
    assert part_1(_read_input()) == "24862"


def test_part_1_example():
    assert part_1(EXAMPLE) == "1985"


def test_part_2_input():
    assert part_2(_read_input()) == "46C91"


def test_part_2_example():
    assert part_2(EXAMPLE) == "5DB3"
