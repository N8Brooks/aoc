from __future__ import annotations

import re
from itertools import cycle
from math import lcm
from typing import Callable, Iterable


def part_1(input: str) -> int:
    return Network.from_input(input, lambda node: node != "ZZZ").steps("AAA")


def part_2(input: str) -> int:
    network = Network.from_input(input, lambda node: node[-1] != "Z")
    return lcm(*map(network.steps, network.roots()))


class Network:
    @staticmethod
    def from_input(input: str, predicate: Callable[[str], bool]) -> Network:
        instructions, _, network_input = input.partition("\n\n")
        instructions = list(map("LR".index, instructions))
        network = {
            match[1]: (match[2], match[3]) for match in PATTERN.finditer(network_input)
        }
        return Network(instructions, network, predicate)

    def __init__(
        self,
        instructions: list[int],
        network: dict[str, tuple[str, str]],
        predicate: Callable[[str], bool],
    ):
        self.instructions = instructions
        self.network = network
        self.predicate = predicate

    def steps(self, node: str) -> int:
        instructions = cycle(self.instructions)
        count = 0
        while self.predicate(node):
            node = self.network[node][next(instructions)]
            count += 1
        return count

    def roots(self) -> Iterable[str]:
        return (node for node in self.network if node[-1] == "A")


PATTERN = re.compile(r"(\w{3}) = \((\w{3}), (\w{3})\)")


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 2


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 6


def test_part_1_input():
    with open("../test_data/year_2023/day_08.txt", "r") as f:
        assert part_1(f.read()) == 20221


def test_part_2_example_3():
    assert part_2(EXAMPLE_3) == 6


def test_part_2_input():
    with open("../test_data/year_2023/day_08.txt", "r") as f:
        assert part_2(f.read()) == 14616363770447


EXAMPLE_1 = """RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"""

EXAMPLE_2 = """LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"""

EXAMPLE_3 = """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"""
