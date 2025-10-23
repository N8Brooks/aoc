"""
https://adventofcode.com/2020/day/7
"""

import re
from collections import defaultdict
from functools import cache
from itertools import repeat
from pathlib import Path

from iteration_utilities import Seen

R = re.compile(r"(\d+) (.+?) bags?[,.]")


def part_1(text, target="shiny gold"):
    def parse(child):
        if contains.contains_add(child):
            return 0

        return sum(map(parse, parents[child])) + 1

    def golden(parent, child):
        parents[child].append(parent)
        if child in contains or child == target:
            return sum(map(parse, parents[child]))

        return 0

    def process(line):
        parent, _, children = line.partition(" bags contain ")
        return sum(map(golden, repeat(parent), r.findall(children)))

    r = re.compile(r"\d+ (.+?) bags?[,.]")

    parents = defaultdict(list)
    contains = Seen()

    return sum(map(process, text.splitlines()))


def part_2(text, target="shiny gold"):
    @cache
    def parse(parent):
        return sum(v * (parse(k) + 1) for k, v in graph[parent].items())

    graph = {}
    for line in text.splitlines():
        parent, _, children = line.partition(" bags contain ")
        graph[parent] = {child: int(n) for n, child in R.findall(children)}

    return parse(target)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_07.txt").read_text()


EXAMPLE_1 = """light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""

EXAMPLE_2 = """shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"""


def test_part_1_input():
    assert part_1(_read_input()) == 205


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 4


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 0


def test_part_2_input():
    assert part_2(_read_input()) == 80902


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 32


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 126
