"""
https://adventofcode.com/2019/day/6
"""

import re
from collections import defaultdict
from functools import partial
from itertools import takewhile
from operator import ne
from pathlib import Path

from iteration_utilities import applyfunc

R = re.compile(r"(\w+)\)(\w+)")


def part_1(text, com="COM"):
    children = defaultdict(list)
    for line in text.splitlines():
        a, b = R.match(line).groups()
        children[a].append(b)

    stack = [(com, 0)]
    total = 0
    while stack:
        parent, level = stack.pop()
        total += level
        stack.extend((child, level + 1) for child in children[parent])

    return total


def part_2(text, com="COM", you="YOU", san="SAN"):
    parents = {}
    for line in text.splitlines():
        a, b = R.match(line).groups()
        parents[b] = a

    path = takewhile(partial(ne, com), applyfunc(parents.get, you))
    visited = {you: i for i, you in enumerate(path)}

    path = applyfunc(parents.get, san)
    count, dest = next((i, san) for i, san in enumerate(path) if san in visited)

    return visited[dest] + count


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_06.txt").read_text()


EXAMPLE_1 = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
"""

EXAMPLE_2 = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
"""


def test_part_1_input():
    assert part_1(_read_input()) == 186597


def test_part_1_example():
    assert part_1(EXAMPLE_1) == 42


def test_part_2_input():
    assert part_2(_read_input()) == 412


def test_part_2_example():
    assert part_2(EXAMPLE_2) == 4
