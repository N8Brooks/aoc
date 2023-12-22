from itertools import islice
from typing import Iterable


# https://www.reddit.com/r/adventofcode/comments/18nevo3/comment/keaiiq7/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button


def part_1(input: str) -> int:
    return nth(iter_reachable_plots(input), 64)


def iter_reachable_plots(input: str) -> Iterable[int]:
    lines = input.splitlines()
    n = len(lines)
    a = set()
    x = 0
    b = {(n // 2, n // 2)}
    y = 1
    yield y
    while True:
        a, b = (
            b,
            {
                (i2, j2)
                for i1, j1 in b
                for i2, j2 in ((i1, j1 + 1), (i1, j1 - 1), (i1 + 1, j1), (i1 - 1, j1))
                if lines[i2 % n][j2 % n] != "#"
            }
            - a,
        )
        x, y = y, x + len(b)
        yield y


def part_2(input: str) -> int:
    return count_reachable_plots(input, 26501365)


def count_reachable_plots(input: str, steps: int) -> int:
    n = input.count("\n") + 1
    q, r = divmod(steps, n)
    it = iter_reachable_plots(input)
    b0 = nth(it, r)
    b1 = nth(it, n - 1)
    b2 = nth(it, n - 1)
    a0 = b0
    a1 = b1 - a0
    a2 = b2 - b1 - a1
    return a0 + a1 * q + a2 * q * (q - 1) // 2


def nth(iterable, n):
    return next(islice(iterable, n, None))


def test_part_1_example_1():
    assert nth(iter_reachable_plots(EXAMPLE_1), 6) == 16


def test_part_1_input():
    with open("../testdata/year_2023/day_21.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 3585


# def test_count_reachable_plots_2_6():
#     assert count_reachable_plots_2(EXAMPLE_1, 6) == 16
#
#
# def test_count_reachable_plots_2_10():
#     assert count_reachable_plots_2(EXAMPLE_1, 10) == 50
#
#
# def test_count_reachable_plots_2_50():
#     assert count_reachable_plots_2(EXAMPLE_1, 50) == 1594
#
#
# def test_count_reachable_plots_2_100():
#     assert count_reachable_plots_2(EXAMPLE_1, 100) == 6536
#
#
# def test_count_reachable_plots_2_500():
#     assert count_reachable_plots_2(EXAMPLE_1, 500) == 167004
#
#
# def test_count_reachable_plots_2_1000():
#     assert count_reachable_plots_2(EXAMPLE_1, 1000) == 668697
#
#
# def test_count_reachable_plots_2_5000():
#     assert count_reachable_plots_2(EXAMPLE_1, 5000) == 16733044


def test_part_2_input():
    with open("../testdata/year_2023/day_21.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 597102953699891


EXAMPLE_1 = """...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."""
