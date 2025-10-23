"""
https://adventofcode.com/2017/day/13
"""

from heapq import heapify, heappop, heappush
from itertools import starmap
from math import log2, prod
from operator import mod
from pathlib import Path

import numpy as np


def process(line):
    return map(int, line.split(": "))


def part_1(text):
    def severity(depth, scope):
        return 0 if depth % (scope + scope - 2) else depth * scope

    return sum(starmap(severity, map(process, text.splitlines())))


def part_2(text):
    quotients, scopes = zip(*map(process, text.splitlines()))
    divisors = tuple(scope + scope - 2 for scope in scopes)

    if all(starmap(mod, zip(quotients, divisors))):
        return 0

    lo, hi = 0, 1
    heapify(queue := [((d - q) % d, d) for q, d in zip(quotients, divisors)])

    for _ in range(int(log2(prod(divisors))) + 1):
        delays = np.ones(lo, bool)
        while queue[0][0] < hi:
            mi, divisor = heappop(queue)
            delays[mi - lo : hi - lo : divisor] = False
            length = len(range(mi, hi, divisor))
            heappush(queue, (mi + divisor * length, divisor))

        if len(indices := np.argwhere(delays)):
            return lo + int(indices[0])

        lo, hi = hi, hi + hi


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_13.txt").read_text()


EXAMPLE = """0: 3
1: 2
4: 4
6: 4
"""

MOCK = """1: 2
4: 4
7: 4
"""


def test_part_1_input():
    assert part_1(_read_input()) == 2604


def test_part_1_example():
    assert part_1(EXAMPLE) == 24


def test_part_1_mock():
    assert part_1(MOCK) == 0


def test_part_2_input():
    assert part_2(_read_input()) == 3941460


def test_part_2_example():
    assert part_2(EXAMPLE) == 10


def test_part_2_mock():
    assert part_2(MOCK) == 0
