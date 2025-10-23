"""
https://adventofcode.com/2020/day/3
"""

from functools import partial
from itertools import islice, starmap
from math import prod
from pathlib import Path

SLOPES = ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))


def trees(lines, right, down):
    def tree(i, line, width=len(lines[0])):
        return "#" == line[right * i % width]

    return sum(starmap(tree, enumerate(islice(lines, 0, None, down))))


def part_1(text, right=3, down=1):
    return trees(text.splitlines(), right, down)


def part_2(text, slopes=SLOPES):
    return prod(starmap(partial(trees, text.splitlines()), slopes))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_03.txt").read_text()


EXAMPLE = """..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#
"""


def test_part_1_input():
    assert part_1(_read_input()) == 207


def test_part_1_example():
    assert part_1(EXAMPLE) == 7


def test_part_2_input():
    assert part_2(_read_input()) == 2655892800


def test_part_2_example():
    assert part_2(EXAMPLE) == 336
