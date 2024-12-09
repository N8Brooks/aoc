from itertools import accumulate, count, pairwise, takewhile
from functools import reduce


def part_1(input: str) -> int:
    return sum(map(next_value, input.rstrip().splitlines()))


def next_value(line: str) -> int:
    return sum(row[-1] for row in takewhile(any, get_deltas(line)))


def part_2(input: str) -> int:
    return sum(map(previous_value, input.rstrip().splitlines()))


def previous_value(line: str) -> int:
    return reduce(lambda value, row: row[0] - value, reversed(get_deltas(line)), 0)


def get_deltas(line: str) -> list[list[int]]:
    deltas = accumulate(
        count(),
        lambda row, _: [b - a for a, b in pairwise(row)],
        initial=list(map(int, line.split())),
    )
    return list(takewhile(any, deltas))


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 114


def test_part_1_input():
    with open("../test_data/year_2023/day_09.txt", "r") as f:
        assert part_1(f.read()) == 1637452029


def test_part_2_example_3():
    assert part_2(EXAMPLE_1) == 2


def test_part_2_input():
    with open("../test_data/year_2023/day_09.txt", "r") as f:
        assert part_2(f.read()) == 908


EXAMPLE_1 = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""
