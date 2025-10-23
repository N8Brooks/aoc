from pathlib import Path
from typing import Iterable


def part_1(input: str) -> int:
    platform = input.splitlines()
    platform = slide_2d(platform)
    platform = tuple(zip(*platform))
    return total_load(platform)


def part_2(input: str) -> int:
    platform = tuple(input.splitlines())
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
    return total_load(platform[::-1])


ITERATIONS = 1_000_000_000


def cycle(platform: tuple[str, ...]) -> tuple[str, ...]:
    north = slide_2d(platform)
    west = slide_2d(north)
    south = slide_2d(west)
    east = slide_2d(south)
    return tuple(east)


def slide_2d(x: Iterable[str]) -> Iterable[str]:
    return map(slide_1d, zip(*x))


def slide_1d(it: str) -> str:
    empty = 0
    res = ""
    for char in it:
        if char == "#":
            res += "." * empty
            res += "#"
            empty = 0
        elif char == ".":
            empty += 1
        elif char == "O":
            res += "O"
        else:
            raise ValueError("Invalid character")
    res += "." * empty
    return res[::-1]


def total_load(platform: tuple[str, ...]) -> int:
    return sum(i * row.count("O") for i, row in enumerate(platform, 1))




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_14.txt").read_text()

def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 136


def test_part_1_input():
    assert part_1(_read_input().rstrip()) == 108857


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 64


def test_part_2_input():
    assert part_2(_read_input().rstrip()) == 95273


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
