from pathlib import Path
from collections import Counter
from math import prod


def part_1(input: str) -> int:
    return sum(i for i, counts in enumerate(parse(input), 1) if counts <= CONFIGURATION)


CONFIGURATION = Counter(red=12, green=13, blue=14)


def part_2(input: str) -> int:
    return sum(prod(map(counts.get, COLORS)) for counts in parse(input))  # type: ignore


COLORS = ["red", "green", "blue"]


def parse(input: str):
    return map(min_cube_set, input.rstrip().splitlines())


def min_cube_set(line: str):
    _, _, cube_sets = line.partition(": ")
    counts = Counter()
    for cube_set in cube_sets.split("; "):
        for item in cube_set.split(", "):
            count, _, color = item.partition(" ")
            count = int(count)
            counts[color] = max(counts[color], count)
    return counts




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_02.txt").read_text()

def test_part_1_example():
    assert part_1(EXAMPLE_1) == 8


def test_part_1_input():
    assert part_1(_read_input()) == 2101


def test_part_2_example():
    assert part_2(EXAMPLE_1) == 2286


def test_part_2_input():
    assert part_2(_read_input()) == 58269


EXAMPLE_1 = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""
