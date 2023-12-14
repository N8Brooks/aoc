from functools import partial
from itertools import repeat
from typing import Iterable


def part_1(input: str) -> int:
    platform = tuple(map(tuple, input.rstrip().splitlines()))
    platform = slide_north(platform)
    return total_load(platform)


def total_load(platform: tuple[tuple[str, ...], ...]) -> int:
    return sum(i * line.count("O") for i, line in enumerate(reversed(platform), 1))


def slide_2d(
    input: tuple[tuple[str, ...], ...], iter_2d, iter_1d
) -> tuple[tuple[str, ...], ...]:
    return tuple(iter_2d(map(slide_1d, iter_2d(input), repeat(iter_1d))))


def transpose(input: list[list[str]]) -> Iterable[list[str]]:
    return zip(*input)


slide_north = partial(slide_2d, iter_2d=transpose, iter_1d=reversed)
slide_south = partial(slide_2d, iter_2d=transpose, iter_1d=iter)
slide_east = partial(slide_2d, iter_2d=iter, iter_1d=iter)
slide_west = partial(slide_2d, iter_2d=iter, iter_1d=reversed)


def slide_1d(input: tuple[str], iter_1d) -> tuple[str, ...]:
    res: list[str] = []
    count = 0
    total = 0
    for char in iter_1d(input):
        if char == "#":
            res.extend(repeat(".", total))
            res.extend(repeat("O", count))
            res.append("#")
            count = 0
            total = 0
        elif char == ".":
            total += 1
        elif char == "O":
            count += 1
        else:
            raise ValueError("Invalid character")
    res.extend(repeat(".", total))
    res.extend(repeat("O", count))
    return tuple(iter_1d(res))


def part_2(input: str) -> int:
    platform = tuple(map(tuple, input.splitlines()))
    i = ITERATIONS
    memo = {}
    for i in reversed(range(ITERATIONS)):
        for slide in (slide_north, slide_west, slide_south, slide_east):
            platform = slide(platform)
        if platform in memo:
            cycle_len = memo[platform] - i
            i %= cycle_len
            break
        memo[platform] = i

    for _ in range(i):
        for slide in (slide_north, slide_west, slide_south, slide_east):
            platform = slide(platform)

    return total_load(platform)


ITERATIONS = 1_000_000_000


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
