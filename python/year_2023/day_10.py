from itertools import product
from typing import Callable


def part_1(input: str) -> int:
    return (len(parse_loop(input)) + 1) // 2


def part_2(input: str) -> int:
    lines = input.rstrip().splitlines()
    m = len(lines)
    n = len(lines[0])
    loop = parse_loop(input)
    chars = "-|JFS"
    total = 0
    for i in range(m):
        parity = 0
        for i, j in zip(range(i, m), range(n)):
            if (i, j) in loop:
                if lines[i][j] in chars:
                    parity ^= 1
            else:
                total += parity

    for j in range(1, n):
        parity = 0
        for i, j in zip(range(m), range(j, n)):
            if (i, j) in loop:
                if lines[i][j] in chars:
                    parity ^= 1
            else:
                total += parity

    return total


def parse_loop(input: str) -> set[tuple[int, int]]:
    lines = input.rstrip().splitlines()

    def neighbors(i: int, j: int) -> tuple[tuple[int, int], ...]:
        match lines[i][j]:
            case "-":
                return ((i, j - 1), (i, j + 1))
            case "|":
                return ((i - 1, j), (i + 1, j))
            case "L":
                return ((i - 1, j), (i, j + 1))
            case "J":
                return ((i - 1, j), (i, j - 1))
            case "F":
                return ((i, j + 1), (i + 1, j))
            case "7":
                return ((i, j - 1), (i + 1, j))
            case "S":
                return tuple(
                    sorted(
                        (i2, j2)
                        for i2, j2 in (
                            (i - 1, j),
                            (i, j + 1),
                            (i + 1, j),
                            (i, j - 1),
                        )
                        if lines[i2][j2] != "."
                        and (i, j) in neighbors(i2, j2)
                        and (i, j) != (i2, j2)
                    )
                )
            case _:
                raise ValueError(f"Unknown character {lines[i][j]} at {i}, {j}")

    frontier = {
        next(
            (i, j)
            for i, line in enumerate(lines)
            for j, char in enumerate(line)
            if char == "S"
        )
    }
    seen: set[tuple[int, int]] = set()
    while frontier:
        seen |= frontier
        frontier = {
            nxt for cur in frontier for nxt in neighbors(*cur) if nxt not in seen
        }

    return seen


CHARS = {
    "-": [(0, -1), (0, 1)],
    "|": [(-1, 0), (1, 0)],
    "L": [(-1, 0), (0, 1)],
    "J": [(-1, 0), (0, -1)],
    "F": [(0, 1), (1, 0)],
    "7": [(0, -1), (1, 0)],
}


def test_part_1_example_1_a():
    assert (
        part_1(
            """\
.....
.S-7.
.|.|.
.L-J.
....."""
        )
        == 4
    )


def test_part_1_example_1_b():
    assert (
        part_1(
            """\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF"""
        )
        == 4
    )


def test_part_1_example_2_a():
    assert (
        part_1(
            """\
..F7.
.FJ|.
SJ.L7
|F--J
LJ..."""
        )
        == 8
    )


def test_part_1_example_2_b():
    assert (
        part_1(
            """\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"""
        )
        == 8
    )


def test_part_1_input():
    with open("../testdata/year_2023/day_10.txt", "r") as f:
        assert part_1(f.read()) == 6951


def test_part_2_example_1_a():
    assert (
        part_2(
            """\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."""
        )
        == 4
    )


def test_part_2_example_1_b():
    assert (
        part_2(
            """\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."""
        )
        == 4
    )


def test_part_2_example_2():
    assert (
        part_2(
            """\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."""
        )
        == 8
    )


def test_part_2_example_3():
    assert (
        part_2(
            """\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"""
        )
        == 10
    )


def test_part_2_input():
    with open("../testdata/year_2023/day_10.txt", "r") as f:
        assert part_2(f.read()) == 563
