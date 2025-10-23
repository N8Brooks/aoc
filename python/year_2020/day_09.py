"""
https://adventofcode.com/2020/day/9
"""

from collections import deque
from itertools import combinations, filterfalse, islice, repeat, starmap
from operator import add
from pathlib import Path

from iteration_utilities import minmax


def missing(numbers, size):
    def present(num):
        contains = num in seen
        seen.extend(map(add, preamble, repeat(num)))
        preamble.append(num)
        return contains

    nums = iter(numbers)
    preamble = deque(islice(nums, size), maxlen=size)
    seen = deque(starmap(add, combinations(preamble, 2)), maxlen=size * size)

    return next(filterfalse(present, nums))


def part_1(text, size=25):
    return missing(map(int, text.split()), size)


def part_2(text, size=25):
    nums = tuple(map(int, text.split()))
    window = deque()
    total = 0
    target = missing(nums, size)

    for right in nums:
        total += right
        window.append(right)
        while target < total:
            total -= window.popleft()
        if total == target:
            return add(*minmax(window))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_09.txt").read_text()


EXAMPLE = """35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"""


def test_part_1_input():
    assert part_1(_read_input()) == 22477624


def test_part_1_example():
    assert part_1(EXAMPLE, 5) == 127


def test_part_2_input():
    assert part_2(_read_input()) == 2980044


def test_part_2_example():
    assert part_2(EXAMPLE, 5) == 62
