"""
https://adventofcode.com/2015/day/4
"""

from hashlib import md5
from itertools import count
from pathlib import Path

import pytest


def process(text, start):
    def valid(suffix):
        hasher = prefix.copy()
        hasher.update(str(suffix).encode())
        hexadecimal = hasher.hexdigest()
        return hexadecimal.startswith(start)

    prefix = md5(text.strip().encode())

    return next(filter(valid, count()))


def part_1(text, start="00000"):
    return process(text, start)


def part_2(text, start="000000"):
    return process(text, start)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_04.txt").read_text()


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input_a():
    assert part_1(_read_input()) == 346386


@pytest.mark.skip(reason="Takes too long")
def test_part_1_example_1():
    assert part_1("abcdef") == 609043


@pytest.mark.skip(reason="Takes too long")
def test_part_1_example_2():
    assert part_1("pqrstuv") == 1048970


def test_part_1_mock_1():
    assert part_1("abc", "0") == 5


@pytest.mark.skip(reason="Takes too long")
def test_part_1_mock_2():
    assert part_1("leet", "00") == 498


@pytest.mark.skip(reason="Takes too long")
def test_part_1_mock_3():
    assert part_1("abcde", "000") == 3452


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input_b():
    assert part_2(_read_input()) == 9958218


@pytest.mark.skip(reason="Takes too long")
def test_part_2_example_1():
    assert part_2("abcdef") == 6742839


@pytest.mark.skip(reason="Takes too long")
def test_part_2_example_2():
    assert part_2("pqrstuv") == 5714438


def test_part_2_mock_1():
    assert part_2("123", "0") == 10


@pytest.mark.skip(reason="Takes too long")
def test_part_2_mock_2():
    assert part_2("panda", "00") == 107


@pytest.mark.skip(reason="Takes too long")
def test_part_2_mock_3():
    assert part_2("giraff", "000") == 1803
