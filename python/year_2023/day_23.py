from collections import defaultdict


def part_1(input: str) -> int:
    dists = defaultdict(list)
    lines = input.splitlines()
    m = len(lines)
    n = len(lines[0])

    for i1, line in enumerate(lines):
        for j1, char in enumerate(line):
            match char:
                case ".":
                    for i2, j2, match in (
                        (i1 + 1, j1, "v"),
                        (i1 - 1, j1, "^"),
                        (i1, j1 + 1, ">"),
                        (i1, j1 - 1, "<"),
                    ):
                        if not (0 <= i2 < m and 0 <= j2 < n):
                            continue
                        if lines[i2][j2] == "." or lines[i2][j2] == match:
                            dists[(i1, j1)].append((i2, j2))
                case "v" if i1 + 1 < m:
                    dists[(i1, j1)].append((i1 + 1, j1))
                case "^" if i1 > 0:
                    dists[(i1, j1)].append((i1 - 1, j1))
                case ">" if j1 + 1 < n:
                    dists[(i1, j1)].append((i1, j1 + 1))
                case "<" if j1 > 0:
                    dists[(i1, j1)].append((i1, j1 - 1))

    init = (0, 1)
    dest = (m - 1, n - 2)
    stack = [(init, init, 0)]
    best = 0
    while stack:
        node1, node2, dist = stack.pop()

        if node2 == dest:
            best = max(best, dist)
            continue

        stack.extend(
            (node2, node3, dist + 1)  # type: ignore
            for node3 in dists[node2]
            if node1 != node3
        )

    return best


def part_2(input: str) -> int:
    neighs = defaultdict(list)
    lines = input.splitlines()
    m = len(lines)
    n = len(lines[0])

    for i1, line in enumerate(lines):
        for j1, char in enumerate(line):
            if char != "#":
                for i2, j2 in (
                    (i1 + 1, j1),
                    (i1, j1 + 1),
                    (i1 - 1, j1),
                    (i1, j1 - 1),
                ):
                    if not (0 <= i2 < m and 0 <= j2 < n):
                        continue
                    if lines[i2][j2] != "#":
                        neighs[(i1, j1)].append((i2, j2))

    dists = defaultdict(list)
    init = (0, 1)
    stack = [(init, init, init, 0)]
    while stack:
        start, node1, node2, dist = stack.pop()

        if len(neighs[node2]) == 1 and node2 != init:
            dists[start].append((node2, dist))
        elif len(neighs[node2]) == 2:
            stack.extend(
                (start, node2, node3, dist + 1)  # type: ignore
                for node3 in neighs[node2]
                if node1 != node3
            )
        else:
            if node2 not in dists:
                stack.extend(
                    (node2, node2, node3, 1)  # type: ignore
                    for node3 in neighs[node2]
                    if node1 != node3
                )
                dists[node2].append((start, dist))
            if all(x != node2 for x, _ in dists[start]):
                dists[start].append((node2, dist))

    init = (0, 1)
    dest = (m - 1, n - 2)
    stack = [(init, {init}, 0)]
    best = 0
    while stack:
        node1, seen, dist = stack.pop()

        if node1 == dest:
            best = max(best, dist)
            continue

        stack.extend(
            (node2, seen | {node2}, dist + delta)  # type: ignore
            for node2, delta in dists[node1]
            if node2 not in seen
        )

    return best


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 94


def test_part_1_input():
    with open("../testdata/year_2023/day_23.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 2106


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 154


def test_part_2_input():
    with open("../testdata/year_2023/day_23.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 6350


EXAMPLE_1 = """#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"""
