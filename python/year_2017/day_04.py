"""
https://adventofcode.com/2017/day/4
"""

from pathlib import Path

from iteration_utilities import all_distinct


def part_1(text):
    def valid(passphrase):
        return all_distinct(passphrase.split())

    return sum(map(valid, text.splitlines()))


def part_2(text):
    def valid(passphrase):
        return all_distinct(map(sorted, passphrase.split()))

    return sum(map(valid, text.splitlines()))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_04.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 325


def test_part_1_example_1():
    assert part_1("aa bb cc dd ee") == 1


def test_part_1_example_2():
    assert part_1("aa bb cc dd aa") == 0


def test_part_1_example_3():
    assert part_1("aa bb cc dd aaa") == 1


def test_part_2_input():
    assert part_2(_read_input()) == 119


def test_part_2_example_1():
    assert part_2("abcde fghij") == 1


def test_part_2_example_2():
    assert part_2("abcde xyz ecdab") == 0


def test_part_2_example_3():
    assert part_2("a ab abc abd abf abj") == 1


def test_part_2_example_4():
    assert part_2("iiii oiii ooii oooi oooo") == 1


def test_part_2_example_5():
    assert part_2("oiii ioii iioi iiio") == 0
