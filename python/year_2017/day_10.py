"""
https://adventofcode.com/2017/day/10
"""

from functools import reduce
from itertools import chain, cycle, islice
from operator import xor
from pathlib import Path

from iteration_utilities import applyfunc, grouper, nth, packed


@packed
def process(array, lengths, first, skip):
    size = len(array)

    for length in lengths:
        iterable = chain(islice(array, length, None), reversed(array[:length]))
        array = tuple(islice(cycle(iterable), skip, skip + size))
        first -= skip + length
        skip = (skip + 1) % size

    return array, lengths, first, skip


def part_1(text, size=256):
    lengths = map(int, text.split(","))
    array, _, first, _ = process((tuple(range(size)), lengths, 0, 0))

    return array[first % size] * array[(first + 1) % size]


def part_2(text, size=256):
    lengths = tuple(chain(map(ord, text.strip()), (17, 31, 73, 47, 23)))
    args = (tuple(range(size)), lengths, 0, 0)

    array, _, first, _ = nth(63)(applyfunc(process, args))
    array = array[first % size :] + array[: first % size]

    return bytes(reduce(xor, group) for group in grouper(array, 16)).hex()


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_10.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 4480


def test_part_1_example():
    assert part_1("3,4,1,5", 5) == 12


def test_part_2_input():
    expected = "c500ffe015c83b60fad2e4b7d59dabc4"
    assert part_2(_read_input()) == expected


def test_part_2_example_1():
    assert part_2("3,4,1,5", 5) == "04"


def test_part_2_example_2():
    expected = "a2582a3a0e66e6e86e3812dcb672a272"
    assert part_2("") == expected


def test_part_2_example_3():
    expected = "33efeb34ea91902bb2f59c9920caa6cd"
    assert part_2("AoC 2017") == expected


def test_part_2_example_4():
    expected = "3efbe78a8d82f29979031a4aa0b16a9d"
    assert part_2("1,2,3") == expected


def test_part_2_example_5():
    expected = "63960835bcdc130f0b66d7ff4f6a5a8e"
    assert part_2("1,2,4") == expected
