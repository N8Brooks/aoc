from typing import Iterable
from heapq import nlargest


def part_1(input: str) -> int:
    return max(iter_total_calories(input))


def part_2(input: str) -> int:
    return sum(nlargest(3, iter_total_calories(input)))


def iter_total_calories(input: str) -> Iterable[int]:
    """Get total calories for each elf"""
    return map(sum_calories, input.split("\n\n"))


def sum_calories(input: str) -> int:
    """Sum of calories for an elf"""
    return sum(map(int, input.splitlines()))


EXAMPLE = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""


def test_part_1_example():
    assert part_1(EXAMPLE) == 24000


def test_part_1_input():
    with open("../test_data/year_2022/day_01.txt", "r") as f:
        assert part_1(f.read()) == 68802


def test_part_2_example():
    assert part_2(EXAMPLE) == 45000


def test_part_2_input():
    with open("../test_data/year_2022/day_01.txt", "r") as f:
        assert part_2(f.read()) == 205370
