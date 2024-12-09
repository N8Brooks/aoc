from itertools import repeat
from operator import add


def part_1(input: str) -> int:
    wins = map(count_wins, input.rstrip().splitlines())
    return sum(1 << (w - 1) for w in wins if w > 0)


def part_2(input: str) -> int:
    lines = input.rstrip().splitlines()
    n = len(lines)
    counts = [1] * n
    for i, line, count in zip(range(1, n + 1), lines, counts):
        j = i + count_wins(line)
        counts[i:j] = map(add, counts[i:j], repeat(count))
    return sum(counts)


def count_wins(line: str):
    _, _, line = line.partition(": ")
    winners, _, numbers = line.partition(" | ")
    winners = set(winners.split())
    numbers = set(numbers.split())
    return len(winners & numbers)


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 13


def test_part_1_input():
    with open("../test_data/year_2023/day_04.txt", "r") as f:
        assert part_1(f.read()) == 26218


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 30


def test_part_2_input():
    with open("../test_data/year_2023/day_04.txt", "r") as f:
        assert part_2(f.read()) == 9997537


EXAMPLE_1 = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""
