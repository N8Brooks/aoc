from itertools import accumulate, chain, combinations, islice
from operator import add
from typing import Iterable


def part_1(input: str) -> int:
    grid = expand_galaxies(input)
    indexes = [
        (i, j)
        for i, row in enumerate(grid)
        for j, char in enumerate(row)
        if char == "#"
    ]
    return sum(
        abs(i1 - i2) + abs(j1 - j2) for (i1, j1), (i2, j2) in combinations(indexes, 2)
    )


def expand_galaxies(input: str) -> list[str]:
    rows = input.rstrip().split("\n")
    rows = list(
        chain.from_iterable(
            [row, row] if all(char == "." for char in row) else [row] for row in rows
        )
    )
    cols = list(zip(*rows))
    cols = list(
        chain.from_iterable(
            [col, col] if all(char == "." for char in col) else [col] for col in cols
        )
    )
    return list(map("".join, zip(*cols)))


def part_2(input: str, expansion: int) -> int:
    rows = input.rstrip().split("\n")
    indexes = (
        (i, j)
        for i, row in enumerate(rows)
        for j, char in enumerate(row)
        if char == "#"
    )
    row_indexes, col_indexes = zip(*indexes)
    row_distance = total_distance(row_indexes, rows, expansion)
    col_distance = total_distance(col_indexes, zip(*rows), expansion)
    return row_distance + col_distance


def total_distance(indexes: Iterable[int], input: Iterable[str], expansion: int):
    dists = FenwickTree(
        [expansion if all(map(".".__eq__, row)) else 1 for row in input]
    )
    return sum(dists.query(i1, i2 - 1) for i1, i2 in combinations(sorted(indexes), 2))


class FenwickTree:
    def __init__(self, nums: list[int]):
        n = len(nums)
        self._bit = [0] * n
        for i, num in enumerate(nums):
            self._bit[i] += num
            j = i | (i + 1)
            if j < n:
                self._bit[j] += self._bit[i]

    def query(self, i: int, j: int) -> int:
        return self.queryRight(j) - self.queryRight(i - 1)

    def queryRight(self, j: int) -> int:
        res = 0
        while j >= 0:
            res += self._bit[j]
            j = (j & (j + 1)) - 1
        return res


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 374


def test_part_1_input():
    with open("../testdata/year_2023/day_11.txt", "r") as f:
        assert part_1(f.read()) == 9370588


def test_part_2_example_3_10():
    assert part_2(EXAMPLE_1, 10) == 1030


def test_part_2_example_3_100():
    assert part_2(EXAMPLE_1, 100) == 8410


def test_part_2_input():
    with open("../testdata/year_2023/day_11.txt", "r") as f:
        assert part_2(f.read(), 1_000_000) == 746207878188


EXAMPLE_1 = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""
