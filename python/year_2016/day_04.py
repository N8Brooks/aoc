"""
https://adventofcode.com/2016/day/4
"""

import re
from collections import Counter
from heapq import nsmallest
from itertools import repeat
from pathlib import Path


def process(text, r=re.compile(r"([a-z-]+)-(\d+)\[([a-z]+)\]")):
    def order(char):
        return ord(char) - 97 - 26 * counts[char]

    for room in text.split():
        name, section, proposed = r.match(room).groups()
        counts = Counter(name.replace("-", ""))
        checksum = "".join(nsmallest(5, counts, key=order))
        if checksum == proposed:
            yield int(section), name


def part_1(text):
    return sum(next(zip(*process(text))))


def part_2(text, target="northpole object storage"):
    def shift(word, n):
        return "".join(chr((ord(c) - 97 + n) % 26 + 97) for c in word)

    def valid(room):
        section, name = room
        translation = " ".join(map(shift, name.split("-"), repeat(section)))
        return translation == target

    return next(filter(valid, process(text)))[0]


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_04.txt").read_text()


EXAMPLE_1 = """aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
"""

EXAMPLE_2 = """qzmt-zixmtkozy-ivhz-343[zimth]
"""


def test_part_1_input():
    assert part_1(_read_input()) == 361724


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 1514


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 343


def test_part_2_input():
    assert part_2(_read_input()) == 482


def test_part_2_example_1():
    assert part_2(EXAMPLE_1, "z a b c d e f g") == 987


def test_part_2_example_2():
    assert part_2(EXAMPLE_2, "very encrypted name") == 343
