from pathlib import Path
from heapq import heappop, heappush
from itertools import accumulate, islice, repeat, takewhile
from math import inf
from operator import ne


def part_1(input: str) -> int:
    grid = [list(map(int, line)) for line in input.splitlines()]
    fscores = get_fscores(grid)
    m = len(grid)
    n = len(grid[0])
    queue: list[tuple[int, int, tuple[int, int], tuple[tuple[int, int], ...]]] = [
        (fscores[0][0], 0, (0, 0), ((0, 0),))
    ]
    dists = [[{} for _ in range(n)] for _ in range(m)]
    dists[0][0][((0, 0),)] = 0
    target = (m - 1, n - 1)
    while queue[0][2] != target:
        _, dist1, (i1, j1), path1 = heappop(queue)
        if dist1 > dists[i1][j1][path1]:
            continue
        for delta2 in DIRECTIONS:
            di, dj = delta2
            i2 = i1 + di
            j2 = j1 + dj
            if not (0 <= i2 < m and 0 <= j2 < n):
                continue
            if path1[-1] == (-di, -dj) or all(delta1 == delta2 for delta1 in path1):
                continue
            path2 = path1[-2:] + (delta2,)
            dist2 = dist1 + grid[i2][j2]
            if dist2 < dists[i2][j2].get(path2, inf):
                dists[i2][j2][path2] = dist2
                fscore2 = dist2 + fscores[i2][j2]
                heappush(
                    queue,
                    (fscore2, dist2, (i2, j2), path2),
                )
    return queue[0][1]


def part_2(input: str) -> int:
    grid = [list(map(int, line)) for line in input.splitlines()]
    fscores = get_fscores(grid)
    m = len(grid)
    n = len(grid[0])
    queue: list[tuple[int, int, tuple[int, int], tuple[int, int]]] = [
        (fscores[0][0], 0, (0, 0), (0, 0))
    ]
    dists = [[{} for _ in range(n)] for _ in range(m)]
    dists[0][0][(0, 0)] = 0
    target = (m - 1, n - 1)
    while queue[0][2] != target:
        _, dist1, (i1, j1), delta1 = heappop(queue)
        if dist1 > dists[i1][j1][delta1]:
            continue
        for delta2 in (
            delta2
            for delta2 in DIRECTIONS
            if any(map(ne, map(bool, delta1), map(bool, delta2)))
        ):
            di2, dj2 = delta2
            row_indexes = takewhile(
                lambda i: 0 <= i < m,
                accumulate(repeat(di2, 9), initial=i1 + di2),
            )
            col_indexes = takewhile(
                lambda j: 0 <= j < n,
                accumulate(repeat(dj2, 9), initial=j1 + dj2),
            )
            indexes = zip(row_indexes, col_indexes)
            dist2 = dist1 + sum(grid[i2][j2] for i2, j2 in islice(indexes, 3))
            for i2, j2 in indexes:
                dist2 += grid[i2][j2]
                if dists[i2][j2].get(delta2, inf) > dist2:
                    dists[i2][j2][(delta2)] = dist2
                    fscore2 = dist2 + fscores[i2][j2]
                    heappush(
                        queue,
                        (fscore2, dist2, (i2, j2), delta2),
                    )
    return queue[0][1]


def get_fscores(grid: list[list[int]]) -> list[list[int]]:
    m = len(grid)
    n = len(grid[0])
    i = m - 1
    j = n - 1
    fscores = [[-1 for _ in range(n)] for _ in range(m)]
    queue = [(grid[i][j], i, j)]
    while queue:
        fscore1, i1, j1 = heappop(queue)
        for di, dj in DIRECTIONS:
            i2 = i1 + di
            j2 = j1 + dj
            if not (0 <= i2 < m and 0 <= j2 < n):
                continue
            if fscores[i2][j2] == -1:
                fscore2 = fscore1 + grid[i2][j2]
                fscores[i2][j2] = fscore2
                heappush(queue, (fscore2, i2, j2))
    return fscores


DIRECTIONS = [(1, 0), (0, 1), (-1, 0), (0, -1)]




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_17.txt").read_text()

def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 102


def test_part_1_input():
    assert part_1(_read_input().rstrip()) == 758


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 94


def test_part_2_input():
    assert part_2(_read_input().rstrip()) == 892


EXAMPLE_1 = """2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"""
