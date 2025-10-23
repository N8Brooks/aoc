"""
https://adventofcode.com/2015/day/9
"""


from pathlib import Path
from collections import defaultdict
from itertools import permutations
import re

from iteration_utilities import successive

def process(text):
    def distance(path):
        return sum(dist[a][b] for a, b in successive(path))

    r = re.compile(r"(\w+) to (\w+) = (\d+)")

    dist = defaultdict(dict)

    for line in text.splitlines():
        a, b, d = r.match(line).groups()
        dist[a][b] = dist[b][a] = int(d)

    return map(distance, permutations(dist))


def part_1(text):
    return min(process(text))


def part_2(text):
    return max(process(text))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_09.txt").read_text()


EXAMPLE = """London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"""


def test_part_1_input():
    assert part_1(_read_input()) == 117

def test_part_1_example():
    assert part_1(EXAMPLE) == 605

def test_part_2_input():
    assert part_2(_read_input()) == 909

def test_part_2_example():
    assert part_2(EXAMPLE) == 982
