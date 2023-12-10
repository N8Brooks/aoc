from itertools import product
from typing import Callable


def part_1(input: str) -> int:
    init, get_neighbors = parse_loop(input)
    frontier = {init}
    pipes = set()
    count = -1
    while frontier:
        pipes |= frontier
        frontier = {
            nxt for cur in frontier for nxt in get_neighbors(*cur) if nxt not in pipes
        }
        count += 1
    return count


def part_2(input: str) -> int:
    init, get_neighbors = parse_loop(input)
    frontier = {init}
    seen = set()
    walls = set()
    while frontier:
        seen |= frontier
        next_frontier = set()
        for cur in frontier:
            for nxt in get_neighbors(*cur):
                if nxt not in seen:
                    walls.add(frozenset((cur, nxt)))
                    next_frontier.add(nxt)
        frontier = next_frontier

    lines = input.rstrip().splitlines()
    m = len(lines)
    n = len(lines[0])
    corners = set()
    stack: list[tuple[int, int]] = [(-1, -1)]

    while stack:
        top_left = i, j = stack.pop()
        if not -1 <= i <= m or not -1 <= j <= n or top_left in corners:
            continue
        corners.add(top_left)

        top_right = (i, j + 1)
        bot_left = (i + 1, j)
        bot_right = (i + 1, j + 1)

        if frozenset((top_left, top_right)) not in walls:
            stack.append((i - 1, j))
        if frozenset((top_left, bot_left)) not in walls:
            stack.append((i, j - 1))
        if frozenset((bot_left, bot_right)) not in walls:
            stack.append((i + 1, j))
        if frozenset((top_right, bot_right)) not in walls:
            stack.append((i, j + 1))

    count = 0
    for i, j in product(range(0, m - 1), range(0, n - 1)):
        top_left = (i, j)
        top_right = (i, j + 1)
        bot_left = (i + 1, j)
        bot_right = (i + 1, j + 1)
        count += all(
            corner not in corners
            for corner in (top_left, top_right, bot_left, bot_right)
        )

    return count


def parse_loop(
    input: str
) -> tuple[tuple[int, int], Callable[[int, int], tuple[tuple[int, int], ...]]]:
    lines = input.rstrip().splitlines()
    init = next(
        (i, j)
        for i, line in enumerate(lines)
        for j, char in enumerate(line)
        if char == "S"
    )

    def pipes(i: int, j: int) -> tuple[tuple[int, int], ...]:
        match lines[i][j]:
            case ".":
                return tuple()
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
                    cur
                    for cur in ((i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1))
                    if (i, j) in pipes(*cur)
                )
            case _:
                raise ValueError(f"Unknown character {lines[i][j]} at {i}, {j}")

    return init, pipes


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
