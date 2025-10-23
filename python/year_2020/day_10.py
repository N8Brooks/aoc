"""
https://adventofcode.com/2020/day/10
"""

from collections import deque
from itertools import chain, starmap
from pathlib import Path

from iteration_utilities import rsub, successive


def part_1(text):
    adapters = sorted(map(int, text.split()))
    padded = chain((0,), adapters, (adapters[-1] + 3,))
    diffs = tuple(starmap(rsub, successive(padded)))

    return diffs.count(1) * diffs.count(3)


def part_2(text, size=25):
    adapters = sorted(map(int, text.split()))
    adapters.append(adapters[-1] + 3)
    memo = deque((0 + 1j,), maxlen=3)

    for jolts in adapters:
        while memo[0].real < jolts - 3:
            memo.popleft().imag
        memo.append(complex(jolts, sum(memo).imag))

    return int(memo[-1].imag)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_10.txt").read_text()


EXAMPLE_1 = "16 10 15 5 1 11 7 19 6 12 4"

EXAMPLE_2 = """28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"""


def test_part_1_input():
    assert part_1(_read_input()) == 2760


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 35


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 220


def test_part_2_input():
    assert part_2(_read_input()) == 13816758796288


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 8


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 19208
