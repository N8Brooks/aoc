from math import isqrt, prod


def part_1(input: str) -> int:
    times, _, distances = input.rstrip().partition("\n")
    times = map(int, times.strip("Time:").split())
    distances = map(int, distances.strip("Distance:").split())
    return prod(map(count_ways_to_beat, times, distances))


def part_2(input: str) -> int:
    time, _, distance = input.rstrip().partition("\n")
    time = int("".join(time.strip("Time:").split()))
    distance = int("".join(distance.strip("Distance:").split()))
    return count_ways_to_beat(time, distance)


def count_ways_to_beat(time: int, distance: int) -> int:
    sqrt_discriminant = isqrt(time**2 - 4 * distance - 1) + 1
    i = (-time + sqrt_discriminant) // -2 + 1
    j = ceil_div(-time - sqrt_discriminant, -2)
    return j - i


def ceil_div(a: int, b: int) -> int:
    return -(a // -b)


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 288


def test_part_1_input():
    with open("../testdata/year_2023/day_06.txt", "r") as f:
        assert part_1(f.read()) == 625968


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 71503


def test_part_2_input():
    with open("../testdata/year_2023/day_06.txt", "r") as f:
        assert part_2(f.read()) == 43663323


EXAMPLE_1 = """Time:      7  15   30
Distance:  9  40  200"""
