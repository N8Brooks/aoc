"""
https://adventofcode.com/2019/day/4
"""

import multiprocessing as mp
from itertools import starmap
from operator import le
from pathlib import Path

import pytest
from iteration_utilities import successive

LIMIT = 1000


def is_sorted(n):
    return all(starmap(le, successive(n)))


def check1(start, stop, step=mp.cpu_count()):
    def check(digits):
        return is_sorted(digits) and any(2 <= digits.count(d) for d in set(digits))

    return sum(check(str(n)) for n in range(start, stop, step))


def check2(start, stop, step=mp.cpu_count()):
    def check(digits):
        return is_sorted(digits) and 2 in map(digits.count, set(digits))

    return sum(check(str(n)) for n in range(start, stop, step))


def process(func, text):
    lo, hi = map(int, text.split("-"))
    if LIMIT < hi - lo:
        args = ((lo + i, hi + 1) for i in range(mp.cpu_count()))
        with mp.Pool() as p:
            return sum(p.starmap(func, args))
    else:
        return func(lo, hi + 1, 1)


def part_1(text):
    return process(check1, text)


def part_2(text):
    return process(check2, text)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_04.txt").read_text()


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input():
    assert part_1(_read_input()) == 495


def test_part_1_example_1():
    assert part_1("111111-111111") == 1


def test_part_1_example_2():
    assert part_1("223450-223450") == 0


def test_part_1_example_3():
    assert part_1("123789-123789") == 0


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == 305


def test_part_2_example_1():
    assert part_2("112233-112233") == 1


def test_part_2_example_2():
    assert part_2("123444-123444") == 0


def test_part_2_example_3():
    assert part_2("111122-111122") == 1
