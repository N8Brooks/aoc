"""
https://adventofcode.com/2015/day/14
"""


from pathlib import Path
from itertools import accumulate, chain, cycle, islice, repeat, starmap
import re
from statistics import mode

from iteration_utilities import argmax

R = re.compile(
    (
        r"(?:\w+) can fly (\d+) km/s for (\d+) "
        r"seconds, but then must rest for (\d+) seconds."
    )
)


def process(text):
    def stats(line):
        return tuple(map(int, R.match(line).groups()))

    return tuple(map(stats, text.splitlines()))


def part_1(text, total=2503):
    def distance(speed, flying, resting):
        cycles, extra = divmod(total, flying + resting)
        result = speed * (flying * cycles + min(flying, extra))
        return result

    return max(starmap(distance, process(text)))


def part_2(text, total=2503):
    def distances(speed, flying, resting):
        iterable = chain(repeat(speed, flying), repeat(0, resting))
        yield from accumulate(cycle(iterable))

    iterable = zip(*starmap(distances, process(text)))
    counts = tuple(map(argmax, islice(iterable, total)))

    return counts.count(mode(counts))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_14.txt").read_text()


EXAMPLE = """\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"""


def test_part_1_input():
    assert part_1(_read_input()) == 2696

def test_part_1_example():
    assert part_1(EXAMPLE, 1000) == 1120

def test_part_2_input():
    assert part_2(_read_input()) == 1084

def test_part_2_example():
    assert part_2(EXAMPLE, 1000) == 688
