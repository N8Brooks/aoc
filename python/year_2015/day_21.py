"""
https://adventofcode.com/2015/day/21
"""


from pathlib import Path
from itertools import combinations, filterfalse, product
from math import ceil, inf
import re

from iteration_utilities import packed

WEAPS = ((8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0))

ARMOR = ((13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5))

RINGS = (
    (20, 0, 1),
    (20, 0, 1),
    (25, 1, 0),
    (40, 0, 2),
    (50, 2, 0),
    (80, 0, 3),
    (100, 3, 0),
)


def simulate(text, optimal, p_health):
    def turns(a, b):
        return inf if b <= 0 else ceil(a / b)

    @packed
    def battle(_, p_damage, p_armor):
        b_turns = turns(p_health, b_damage - p_armor)
        p_turns = turns(b_health, p_damage - b_armor)
        return p_turns <= b_turns

    def stats(items):
        return tuple(map(sum, zip(*items)))

    b_health, b_damage, b_armor = map(int, re.findall(r"\d+", text))

    weaps = WEAPS
    armor = ((0, 0, 0),) + ARMOR
    rings = ((0, 0, 0),) + RINGS + tuple(map(stats, combinations(RINGS, 2)))

    equipment = map(stats, product(weaps, armor, rings))

    if optimal:
        return min(filter(battle, equipment))[0]

    return max(filterfalse(battle, equipment))[0]


def part_1(text, p_health=100):
    return simulate(text, True, p_health)


def part_2(text, p_health=100):
    return simulate(text, False, p_health)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_21.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 111

def test_part_1_example():
    assert part_1('12 7 2', 8) == 65

def test_part_2_input():
    assert part_2(_read_input()) == 188

def test_part_2_mock():
    assert part_2('6 7 2', 8) == 88
