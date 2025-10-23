from pathlib import Path
from math import isqrt, prod


def part_1(input: str) -> int:
    times, _, distances = input.rstrip().partition("\n")
    times = map(int, times.strip("Time:").split())
    distances = map(int, distances.strip("Distance:").split())
    return prod(map(count_ways_to_beat, times, distances))


def part_2(input: str) -> int:
    time, _, distance = input.rstrip().partition("\n")
    time = int(time.strip("Time:").replace(" ", ""))
    distance = int(distance.strip("Distance:").replace(" ", ""))
    return count_ways_to_beat(time, distance)


def count_ways_to_beat(time: int, distance: int) -> int:
    sqrt_discriminant = isqrt(time * time - 4 * distance - 1) + 1
    i = (-time + sqrt_discriminant) // -2 + 1
    j = ceil_div(-time - sqrt_discriminant, -2)
    return j - i


def ceil_div(a: int, b: int) -> int:
    return -(a // -b)




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_06.txt").read_text()

def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 288


def test_part_1_input():
    assert part_1(_read_input()) == 625968


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 71503


def test_part_2_input():
    assert part_2(_read_input()) == 43663323


EXAMPLE_1 = """Time:      7  15   30
Distance:  9  40  200"""
