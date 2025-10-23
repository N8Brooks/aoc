"""
https://adventofcode.com/2015/day/11
"""


from pathlib import Path
import pytest
from itertools import accumulate, starmap, repeat
from operator import eq

from iteration_utilities import (
    count_items,
    starfilter,
    successive,
    unique_everseen,
)

IOL = {8, 11, 14}


def part_1(text):
    def next_pass(password, _=None):
        for i, x in zip(reversed(range(n)), reversed(password)):
            if (x := x + 1) in IOL:
                password[i] = x + 1
                password[i + 1 :] = repeat(0, n - i - 1)
            if x == 26:
                password[i] = 0
            else:
                password[i] = x
                break
        return password

    def straight(a, b, c):
        return a + 1 == b and b + 1 == c

    def valid(string):
        if 8 in string or 11 in string or 14 in string:
            return False

        if not any(starmap(straight, successive(string, 3))):
            return False

        pairs = unique_everseen(starfilter(eq, successive(string)))

        return 1 < count_items(pairs)

    n = len(processed := text.strip())

    password = next_pass(list(i - 97 for i in map(ord, processed)))

    passwords = accumulate(repeat(None), func=next_pass, initial=password)

    return "".join(chr(i + 97) for i in next(filter(valid, passwords)))


def part_2(text):
    return part_1(part_1(text))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_11.txt").read_text()


@pytest.mark.skip(reason='Takes too long')
def test_part_1_input():
    assert part_1(_read_input()) == 'hxbxxyzz'

@pytest.mark.skip(reason='Takes too long')
def test_part_1_example_1():
    assert part_1('abcdefgh') == 'abcdffaa'

@pytest.mark.skip(reason='Takes too long')
def test_part_1_example_2():
    assert part_1('ghijklmn') == 'ghjaabcc'

def test_part_1_mock_1():
    assert part_1('bbccc') == 'bbcdd'

def test_part_1_mock_2():
    assert part_1('xxydf') == 'xxyzz'

@pytest.mark.skip(reason='Takes too long')
def test_part_2_input():
    assert part_2(_read_input()) == 'hxcaabcc'

@pytest.mark.skip(reason='Takes too long')
def test_part_2_example_1():
    assert part_2('abcdefgh') == 'abcdffbb'

@pytest.mark.skip(reason='Takes too long')
def test_part_2_example_2():
    assert part_2('ghijklmn') == 'ghjbbcdd'

def test_part_2_mock_1():
    assert part_2('aabcb') == 'bbcdd'

@pytest.mark.skip(reason='Takes too long')
def test_part_2_mock_2():
    assert part_2('ddefe') == 'eefgg'
