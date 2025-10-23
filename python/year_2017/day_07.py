"""
https://adventofcode.com/2017/day/7
"""

import re
from itertools import chain
from pathlib import Path
from statistics import mode

from iteration_utilities import all_equal, empty


def unique(iterable):
    one = set()
    two = set()

    for element in iterable:
        if element in one:
            one.remove(element)
            two.add(element)
        elif element not in two:
            one.add(element)

    return one.pop()


def part_1(text):
    return unique(re.findall(r"[a-z]+", text))


def part_2(text):
    def scale(parent):
        weights[parent] += sum(map(scale, graph[parent]))
        return weights[parent]

    def deepflatten(node):
        yield node
        yield from chain(*map(deepflatten, graph[node]))

    r = re.compile(r"([a-z]+) \((\d+)\)(?: -> (.*))?")

    weights = {}
    graph = {}

    for line in text.splitlines():
        parent, weight, kids = r.match(line).groups()
        graph[parent] = empty if kids is None else kids.split(", ")
        weights[parent] = int(weight)

    scale(head := unique(chain.from_iterable(map(deepflatten, graph))))
    sibs = kids = dict((kid, weights[kid]) for kid in graph[head])

    while not all_equal(kids.values()):
        odd = unique(kids.values())
        head = next(kid for kid, weight in kids.items() if weight == odd)
        sibs, kids = kids, dict((kid, weights[kid]) for kid in graph[head])

    return mode(sibs.values()) - len(kids) * next(iter(kids.values()))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_07.txt").read_text()


EXAMPLE = """pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
"""


def test_part_1_input():
    assert part_1(_read_input()) == "ahnofa"


def test_part_1_example():
    assert part_1(EXAMPLE) == "tknk"


def test_part_2_input():
    assert part_2(_read_input()) == 802


def test_part_2_example():
    assert part_2(EXAMPLE) == 60
