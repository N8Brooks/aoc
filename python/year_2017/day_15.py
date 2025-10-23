"""
https://adventofcode.com/2017/day/15
"""

import re
from itertools import filterfalse, islice
from pathlib import Path

import pytest
from iteration_utilities import applyfunc, count_items, packed

BITS = (1 << 16) - 1

DIVISOR = 2147483647


def count(a_gen, b_gen, sample):
    def valid(a, b):
        return a & BITS == b & BITS

    return count_items(islice(zip(a_gen, b_gen), sample), packed(valid))


def part_1(text, a_mult=16807, b_mult=48271, sample=int(4e7)):
    a_start, b_start = map(int, re.findall(r"\d+", text))

    a_gen = applyfunc(lambda a: (a_mult * a) % DIVISOR, a_start)
    b_gen = applyfunc(lambda b: (b_mult * b) % DIVISOR, b_start)

    return count(a_gen, b_gen, sample)


def part_2(text, a_mult=16807, b_mult=48271, sample=int(5e6)):
    a_start, b_start = map(int, re.findall(r"\d+", text))

    a_gen = applyfunc(lambda a: (a_mult * a) % DIVISOR, a_start)
    a_gen = filterfalse(lambda a: a % 4, a_gen)
    b_gen = applyfunc(lambda b: (b_mult * b) % DIVISOR, b_start)
    b_gen = filterfalse(lambda b: b % 8, b_gen)

    return count(a_gen, b_gen, sample)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_15.txt").read_text()


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input():
    assert part_1(_read_input()) == 626


@pytest.mark.skip(reason="Takes too long")
def test_part_1_example():
    assert part_1("65, 8921") == 626


def test_part_1_mock_1():
    assert part_1("193, 1234", 7331, 8675309, 1000) == 1


def test_part_1_mock_2():
    assert part_1("12947, 1234", 7331, 8675309, 1000) == 2


def test_part_1_mock_3():
    assert part_1("642, 1234", 7331, 8675309, 10000) == 3


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == 306


@pytest.mark.skip(reason="Takes too long")
def test_part_2_example():
    assert part_2("65, 8921") == 309


def test_part_2_mock_1():
    assert part_2("0, 1234", 7331, 8675309, 1000) == 1


def test_part_2_mock_2():
    assert part_2("649, 1234", 7331, 8675309, 1000) == 2


def test_part_2_mock_3():
    assert part_2("313, 1234", 7331, 8675309, 10000) == 5
