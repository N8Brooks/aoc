"""
https://adventofcode.com/2017/day/6
"""

from functools import partial
from itertools import chain, islice, repeat, takewhile
from operator import add, ne
from pathlib import Path

import pytest
from iteration_utilities import applyfunc, argmax, count_items, last


def process(text):
    def spread(start, stop, inside, outside):
        yield from repeat(outside, start)
        yield from repeat(inside, stop - start)
        yield from repeat(outside, n - stop)

    def redistribute(previous):
        i = argmax(previous)
        div, mod = divmod(previous[i], n)

        chained = chain(islice(previous, i), (0,), islice(previous, i + 1, n))
        if (a := i + mod + 1) == (b := a % n):
            memory = tuple(map(add, chained, spread(i + 1, a, div + 1, div)))
        else:
            memory = tuple(map(add, chained, spread(b, i + 1, div, div + 1)))

        return visited.setdefault(previous, memory)

    def duplicated(memory):
        return memory not in visited

    visited = {}
    n = len(memory := tuple(map(int, text.split())))
    final = last(takewhile(duplicated, applyfunc(redistribute, memory)))

    return visited, final


def part_1(text):
    return len(process(text)[0])


def part_2(text):
    graph, node = process(text)
    unique = partial(ne, node)
    return count_items(takewhile(unique, applyfunc(graph.get, node))) + 1


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_06.txt").read_text()


EXAMPLE = "0 2 7 0"

MOCK_1 = "0 5 8 3 9"

MOCK_2 = "9 2 8 7 4 3 0 8 2"

MOCK_3 = "1 9 0 0 100 0"


@pytest.mark.skip(reason="Takes too long")
def test_part_1_input():
    assert part_1(_read_input()) == 12841


def test_part_1_example():
    assert part_1(EXAMPLE) == 5


def test_part_1_mock_1():
    assert part_1(MOCK_1) == 15


def test_part_1_mock_2():
    assert part_1(MOCK_2) == 13


def test_part_1_mock_3():
    assert part_1(MOCK_3) == 18


@pytest.mark.skip(reason="Takes too long")
def test_part_2_input():
    assert part_2(_read_input()) == 8038


def test_part_2_example():
    assert part_2(EXAMPLE) == 4


def test_part_2_mock_1():
    assert part_2(MOCK_1) == 5


def test_part_2_mock_2():
    assert part_2(MOCK_2) == 9


def test_part_2_mock_3():
    assert part_2(MOCK_3) == 6
