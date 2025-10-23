from pathlib import Path
from itertools import combinations
from typing import Iterable

# https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/keqf8uq/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button


def part_1(input: str) -> int:
    return count_cross(input, 200000000000000, 400000000000000)


def count_cross(input: str, start: int, end: int) -> int:
    count = 0
    for (x1, y1, _, dx1, dy1, _), (x2, y2, _, dx2, dy2, _) in combinations(
        parse(input), 2
    ):
        m1 = dy1 / dx1
        m2 = dy2 / dx2
        m = m1 - m2
        if m == 0:
            continue  # assuming no overlapping lines
        x = (1 / m) * (m1 * x1 - m2 * x2 + y2 - y1)
        t1 = (x - x1) / dx1
        y = y1 + t1 * dy1
        t2 = (x - x2) / dx2
        if t1 > 0 and t2 > 0 and start <= x <= end and start <= y <= end:
            count += 1
    return count


def parse(input: str) -> Iterable[tuple[int, ...]]:
    return (
        tuple(map(int, line.replace(" @", ",").split(", ")))
        for line in input.splitlines()
    )


def part_2(input: str) -> int:
    hailstones = list(parse(input))
    dx, dy, dz = (next(iter(get_deltas(hailstones, i))) for i in range(3))
    x1, y1, z1, dx1, dy1, dz1 = hailstones[0]
    x2, y2, _, dx2, dy2, _ = hailstones[1]
    m1 = (dy1 - dy) / (dx1 - dx)
    m2 = (dy2 - dy) / (dx2 - dx)
    b1 = y1 - m1 * x1
    b2 = y2 - m2 * x2
    x = round((b2 - b1) / (m1 - m2))
    y = round(m1 * x + b1)
    t = round((x - x1) / (dx1 - dx))
    z = z1 + t * (dz1 - dz)
    return x + y + z


def get_deltas(hailstones: list[tuple[int, ...]], i: int):
    deltas = set(range(-1000, 1000))
    for hail1, hail2 in combinations(hailstones, 2):
        x1 = hail1[i]
        dx1 = hail1[i + 3]
        x2 = hail2[i]
        dx2 = hail2[i + 3]
        if dx1 == dx2 and abs(dx1) > 100:
            diff = x2 - x1
            deltas.intersection_update(
                dx for dx in range(-1000, 1000) if dx != dx1 and diff % (dx - dx1) == 0
            )
    return deltas




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_24.txt").read_text()

def test_part_1_example_1():
    assert count_cross(EXAMPLE_1, 7, 27) == 2


def test_part_1_input():
    assert part_1(_read_input().rstrip()) == 20847


# def test_part_2_example_1():
#     assert part_2(EXAMPLE_1) == 47


def test_part_2_input():
    assert part_2(_read_input().rstrip()) == 908621716620524


EXAMPLE_1 = """19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"""
