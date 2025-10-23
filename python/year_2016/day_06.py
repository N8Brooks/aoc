"""
https://adventofcode.com/2016/day/6
"""

from collections import Counter
from pathlib import Path
from statistics import mode


def part_1(text):
    return "".join(map(mode, zip(*text.splitlines())))


def part_2(text):
    def antimode(message):
        counts = Counter(message)
        return min(counts, key=counts.get)

    return "".join(map(antimode, zip(*text.splitlines())))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_06.txt").read_text()


EXAMPLE = """eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
"""


def test_part_1_input():
    assert part_1(_read_input()) == "gyvwpxaz"


def test_part_1_example():
    assert part_1(EXAMPLE) == "easter"


def test_part_2_input():
    assert part_2(_read_input()) == "jucfoary"


def test_part_2_example():
    assert part_2(EXAMPLE) == "advent"
