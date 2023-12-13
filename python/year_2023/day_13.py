from typing import Optional
from operator import eq


def part_1(input: str) -> int:
    return sum(map(reflection_1, input.split("\n\n")))  # type: ignore


def reflection_1(input: str) -> Optional[int]:
    rows = input.splitlines()
    m = len(rows)
    for i2 in range(1, m):
        if all(map(eq, rows[i2 - 1 :: -1], rows[i2:])):
            return i2 * 100

    cols = list(zip(*rows))
    n = len(cols)
    for j2 in range(1, n):
        if all(map(eq, cols[j2 - 1 :: -1], cols[j2:])):
            return j2


def part_2(input: str) -> int:
    total = 0
    for pattern in input.split("\n\n"):
        pattern = list(map(list, pattern.splitlines()))
        for i in range(len(pattern)):
            for j in range(len(pattern[0])):
                original = pattern[i][j]
                changed = "." if original == "#" else "#"
                pattern[i][j] = changed
                num = reflection_2(pattern, i, j)
                if num is not None:
                    total += num
                    break
                pattern[i][j] = original
            else:
                continue
            break
    return total


def reflection_2(rows: list[list[str]], i1: int, j1: int) -> Optional[int]:
    for i2 in range(i1 // 2 + 1, i1 + 1):
        if all(map(eq, rows[i2 - 1 :: -1], rows[i2:])):
            return i2 * 100

    cols = list(zip(*rows))
    for j2 in range(j1 // 2 + 1, j1 + 1):
        if all(map(eq, cols[j2 - 1 :: -1], cols[j2:])):
            return j2


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 405


def test_part_1_input():
    with open("../testdata/year_2023/day_13.txt", "r") as f:
        assert part_1(f.read()) == 41859


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 400


def test_part_2_input():
    with open("../testdata/year_2023/day_13.txt", "r") as f:
        assert part_2(f.read()) == 30842


EXAMPLE_1 = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""
