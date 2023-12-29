from abc import ABC, abstractmethod
from collections import deque
from itertools import count, repeat
from math import lcm
from typing import Iterable


def part_1(input: str) -> int:
    modules = Module.parse_modules(input)
    signals = [0, 0]

    for _ in range(1000):
        stack: deque[tuple[str, int, str]] = deque([("button", 0, "broadcaster")])
        while stack:
            parent, signal, child = stack.popleft()
            signals[signal] += 1
            if child not in modules:
                continue
            module = modules[child]
            stack.extend(
                (child, signal, grandchild)
                for grandchild, signal in module.receive(parent, signal)
            )

    return signals[0] * signals[1]


def part_2(input: str) -> int:
    modules = Module.parse_modules(input)
    conj = {}
    for presses in count(1):
        stack: deque[tuple[str, int, str]] = deque([("button", 0, "broadcaster")])
        while stack:
            parent, signal, child = stack.popleft()
            if child == "rx":
                continue
            module = modules[child]
            if (
                signal == 0
                and isinstance(module, Conjunction)
                and len(module.memory) == 1
                and "rx" not in module.memory
                and child not in conj
            ):
                conj[child] = presses
                if len(conj) == 4:
                    break
            stack.extend(
                (child, signal, grandchild)
                for grandchild, signal in module.receive(parent, signal)
            )
        else:
            continue
        break
    return lcm(*list(conj.values()))


class Module(ABC):
    @staticmethod
    def parse_modules(input: str):
        modules = {}

        for line in input.splitlines():
            parent_str, _, children_str = line.partition(" -> ")
            children = children_str.split(", ")

            if parent_str == "broadcaster":
                modules[parent_str] = Broadcast(children)
                continue

            module_type = parent_str[0]
            parent = parent_str[1:]
            if module_type == "%":
                modules[parent] = FlipFlop(children)
            else:
                modules[parent] = Conjunction(children)

        for parent, module in modules.items():
            for child in module.outputs:
                if child in modules:
                    modules[child].add_input(parent)

        return modules

    @abstractmethod
    def receive(self, name: str, signal: int) -> Iterable[tuple[str, int]]:
        pass

    def add_input(self, name: str):
        pass


class FlipFlop(Module):
    def __init__(self, outputs: list[str]):
        self.outputs = outputs
        self.signal = 0

    def receive(self, name: str, signal: int):
        if not signal:
            self.signal = 1 - self.signal
            yield from zip(self.outputs, repeat(self.signal))


class Conjunction(Module):
    def __init__(self, outputs: list[str]):
        self.outputs = outputs
        self.memory = {}

    def receive(self, name: str, signal: int):
        self.memory[name] = signal
        signal = 1 - all(self.memory.values())
        return zip(self.outputs, repeat(signal))

    def add_input(self, name: str):
        self.memory[name] = 0


class Broadcast(Module):
    def __init__(self, outputs: list[str]):
        self.outputs = outputs

    def receive(self, name: str, signal: int):
        return zip(self.outputs, repeat(signal))


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 32000000


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 11687500


def test_part_1_input():
    with open("../testdata/year_2023/day_20.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 788848550


# def test_part_2_example_2():
#     assert part_2(EXAMPLE_2.replace("output", "rx")) == 1


def test_part_2_input():
    with open("../testdata/year_2023/day_20.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 228300182686739


EXAMPLE_1 = """broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"""

EXAMPLE_2 = """broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"""
