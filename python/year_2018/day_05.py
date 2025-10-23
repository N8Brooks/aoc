"""
https://adventofcode.com/2018/day/5
"""

from functools import reduce
from pathlib import Path
from string import ascii_lowercase as lower
from string import ascii_uppercase as upper

import pytest
from iteration_utilities import successive

REACTIONS = frozenset(f"{a}{A}" for a, A in zip(f"{lower}{upper}", f"{upper}{lower}"))


def part_1(text):
    def remove(polymer, reaction):
        return polymer.replace(reaction, "")

    polymer = text.strip()

    while react := REACTIONS & frozenset(map("".join, successive(polymer))):
        polymer = reduce(remove, react, polymer)

    return len(polymer)


def part_2(text):
    return min(part_1(text.replace(a, "").replace(a.upper(), "")) for a in lower)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_05.txt").read_text()


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input():
    assert part_1(_read_input()) == 10886


def test_part_1_example():
    assert part_1("dabAcCaCBAcCcaDA") == 10


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == 4684


def test_part_2_example():
    assert part_2("dabAcCaCBAcCcaDA") == 4
