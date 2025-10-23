"""
https://adventofcode.com/2015/day/19
"""

import re
from collections import defaultdict
from itertools import chain, starmap
from pathlib import Path
from random import shuffle

from iteration_utilities import empty


def part_1(text):
    def replace(i, token):
        for replacement in replacements.get(token, empty):
            tokens[i] = replacement
            yield "".join(tokens)
        tokens[i] = token

    def tokenize(molecule):
        return re.findall(r"[A-Z][a-z]?", molecule)

    lines, molecule = text.strip().split("\n\n")

    replacements = defaultdict(list)
    for line in lines.split("\n"):
        key, val = line.split(" => ")
        replacements[key].append(val)

    tokens = tokenize(molecule)

    return len(set(chain.from_iterable(starmap(replace, enumerate(tokens)))))


def part_2(text, starting="e"):
    def process(line):
        return line.split(" => ")

    lines, molecule = text.strip().split("\n\n")
    replacements = list(map(process, lines.split("\n")))
    saved, steps = molecule, 0

    while molecule != starting:
        for x, y in replacements:
            if y not in molecule:
                continue

            molecule = molecule.replace(y, x, 1)
            steps += 1
            break
        else:
            shuffle(replacements)
            molecule, steps = saved, 0

    return steps


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_19.txt").read_text()


EXAMPLE_1 = """H => HO
H => OH
O => HH

HOH
"""

EXAMPLE_2 = """e => H
e => O
H => HO
H => OH
O => HH

HOH
"""


def test_part_1_input():
    assert part_1(_read_input()) == 576


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 4


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 4


def test_part_2_input():
    assert part_2(_read_input()) == 207


def test_part_2_example_1():
    assert part_2(EXAMPLE_1, "O") == 2


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 3
