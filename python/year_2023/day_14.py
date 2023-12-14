from itertools import repeat
from typing import Iterable


def part_1(input: str) -> int:
    east = input.splitlines()
    north = slide_2d(east)
    west = tuple(zip(*north))
    return total_load(west[::-1])


def part_2(input: str) -> int:
    platform = tuple(map(tuple, input.splitlines()))
    i = ITERATIONS
    memo = {platform: i}
    for i in reversed(range(i)):
        platform = cycle(platform)
        if platform in memo:
            i %= memo[platform] - i
            break
        memo[platform] = i
    for _ in range(i):
        platform = cycle(platform)
    return total_load(platform)


ITERATIONS = 1_000_000_000


def cycle(platform: tuple[tuple[str, ...], ...]) -> tuple[tuple[str, ...], ...]:
    north = slide_2d(platform)
    west = slide_2d(north)
    south = slide_2d(west)
    east = slide_2d(south)
    return tuple(map(tuple, east))


def slide_2d(x):
    return (slide_1d(reversed(row)) for row in zip(*x))


def slide_1d(it: Iterable[str]) -> Iterable[str]:
    round = 0
    for char in it:
        if char == "#":
            yield from repeat("O", round)
            yield "#"
            round = 0
        elif char == ".":
            yield "."
        elif char == "O":
            round += 1
        else:
            raise ValueError("Invalid character")
    yield from repeat("O", round)


def total_load(east: tuple[tuple[str, ...], ...]) -> int:
    return sum(i * col.count("O") for i, col in enumerate(reversed(east), 1))


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 136


def test_part_1_input():
    with open("../testdata/year_2023/day_14.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 108857


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 64


def test_part_2_input():
    with open("../testdata/year_2023/day_14.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 95273


EXAMPLE_1 = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."""
