"""
https://adventofcode.com/2016/day/5
"""

from hashlib import md5
from itertools import count, islice
from pathlib import Path

import pytest


def hashes(text, start):
    def hash_i(i):
        hasher = prefix.copy()
        hasher.update(str(i).encode())
        return hasher.hexdigest()

    def valid(hash_i):
        return hash_i.startswith(start)

    prefix = md5(text.strip().encode())

    return filter(valid, map(hash_i, count()))


def part_1(text, length=8, start="00000"):
    iterator = (hash_i[len(start)] for hash_i in hashes(text, start))
    return "".join(islice(iterator, 0, length))


def part_2(text, length=8, start="00000"):
    def char(i):
        while password[i] is None:
            code = next(iterator)
            check = ord(code[index]) - 48
            if i <= check < length and password[check] is None:
                password[check] = code[index + 1]
        return password[i] or next()

    index = len(start)
    password = [None] * length
    iterator = hashes(text, start)

    return "".join(map(char, range(length)))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_05.txt").read_text()


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input():
    assert part_1(_read_input()) == "c6697b55"


@pytest.mark.skip(reason="Takes too long")
def test_part_1_example():
    assert part_1("abc") == "18f47a30"


def test_part_1_mock_1():
    assert part_1("abc", 4, "0") == "890c"


def test_part_1_mock_2():
    assert part_1("xyz", 2, "00") == "ee"


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == "8c35d1ab"


@pytest.mark.skip(reason="Takes too long")
def test_part_2_example():
    assert part_2("abc") == "05ace8e3"


def test_part_2_mock_1():
    assert part_2("abc", 4, "0") == "3010"


def test_part_2_mock_2():
    assert part_2("xyz", 2, "00") == "cd"
