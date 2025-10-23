"""
https://adventofcode.com/2018/day/2
"""

from collections import Counter
from pathlib import Path

from iteration_utilities import Seen


def part_1(text):
    def checksum(line):
        frequencies = set(Counter(line).values())
        return complex(2 in frequencies, 3 in frequencies)

    total = sum(map(checksum, text.splitlines()))
    return int(total.imag * total.real)


def part_2(text):
    lines = text.strip().split()
    length = len(lines[0])
    commons = [Seen() for _ in range(length)]

    for line in lines:
        for i, common in enumerate(commons):
            string = f"{line[:i]}{line[i + 1:]}"
            if common.contains_add(string):
                return string


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_02.txt").read_text()


EXAMPLE_1 = """abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"""

EXAMPLE_2 = """abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"""


def test_part_1_input():
    assert part_1(_read_input()) == 5681


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 12


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 0


def test_part_2_input():
    expected = "uqyoeizfvmbistpkgnocjtwld"
    assert part_2(_read_input()) == expected


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == "abcde"


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == "fgij"
