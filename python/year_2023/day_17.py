from heapq import heappop, heappush
from itertools import accumulate, islice, repeat, takewhile
from math import inf
from operator import ne


def part_1(input: str) -> int:
    grid = [list(map(int, line)) for line in input.splitlines()]
    m = len(grid)
    n = len(grid[0])
    queue: list[tuple[int, tuple[int, int], tuple[tuple[int, int], ...]]] = [
        (0, (0, 0), ((0, 0),))
    ]
    dists = [[{} for _ in range(n)] for _ in range(m)]
    dists[0][0][((0, 0),)] = 0
    target = (m - 1, n - 1)
    while queue[0][1] != target:
        dist1, (i1, j1), path1 = heappop(queue)
        if dist1 > dists[i1][j1][path1]:
            continue
        for delta2 in ((1, 0), (0, 1), (-1, 0), (0, -1)):
            di, dj = delta2
            i2 = i1 + di
            j2 = j1 + dj
            if not (0 <= i2 < m and 0 <= j2 < n):
                continue
            path2 = path1[-2:] + (delta2,)
            dist2 = dist1 + grid[i2][j2]
            if (
                path1[-1] != (-di, -dj)
                and any(delta1 != delta2 for delta1 in path1)
                and dist2 < dists[i2][j2].get(path2, inf)
            ):
                dists[i2][j2][path2] = dist2
                heappush(
                    queue,
                    (dist2, (i2, j2), path2),
                )
    return queue[0][0]


def part_2(input: str) -> int:
    grid = [list(map(int, line)) for line in input.splitlines()]
    m = len(grid)
    n = len(grid[0])
    queue: list[tuple[int, tuple[int, int], tuple[int, int]]] = [(0, (0, 0), (0, 0))]
    dists = [[{} for _ in range(n)] for _ in range(m)]
    dists[0][0][(0, 0)] = 0
    target = (m - 1, n - 1)
    while queue[0][1] != target:
        dist1, (i1, j1), delta1 = heappop(queue)
        if dist1 > dists[i1][j1][delta1]:
            continue
        for delta2 in (
            delta2
            for delta2 in ((1, 0), (0, 1), (-1, 0), (0, -1))
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
            indexes = list(zip(row_indexes, col_indexes))
            it_dist = accumulate((grid[i][j] for i, j in indexes), initial=dist1)
            next(it_dist)  # skip init
            for (i2, j2), dist2 in islice(zip(indexes, it_dist), 3, None):
                if dists[i2][j2].get(delta2, inf) > dist2:
                    dists[i2][j2][(delta2)] = dist2
                    heappush(
                        queue,
                        (dist2, (i2, j2), delta2),
                    )
    return queue[0][0]


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 102


def test_part_1_input():
    with open("../testdata/year_2023/day_17.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 758


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 94


def test_part_2_input():
    with open("../testdata/year_2023/day_17.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 892


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
