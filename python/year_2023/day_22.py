from pathlib import Path
from collections import defaultdict
from itertools import product


def part_1(input: str) -> int:
    heights = defaultdict(int)
    indexes = {}
    disentegratable = set()
    for i, ((x1, y1, z1), (x2, y2, z2)) in enumerate(parse(input)):
        dz = z2 - z1 + 1
        z = max(heights[key] for key in product(range(x1, x2 + 1), range(y1, y2 + 1)))
        support = set(
            indexes[key]
            for key in product(range(x1, x2 + 1), range(y1, y2 + 1))
            if key in indexes and heights[key] == z
        )
        match tuple(support):
            case [j]:
                disentegratable.discard(j)
        for key in product(range(x1, x2 + 1), range(y1, y2 + 1)):
            indexes[key] = i
            heights[key] = z + dz
        disentegratable.add(i)
    return len(disentegratable)


def parse(input: str):
    return sorted(map(parse_brick, input.splitlines()), key=lambda brick: brick[0][2])


def parse_brick(line: str):
    a, _, b = line.partition("~")
    x1, y1, z1 = map(int, a.split(","))
    x2, y2, z2 = map(int, b.split(","))
    return (x1, y1, z1), (x2, y2, z2)


def part_2(input: str) -> int:
    heights = defaultdict(int)
    indexes = {}
    parents = []
    i = -1
    for i, ((x1, y1, z1), (x2, y2, z2)) in enumerate(parse(input)):
        dz = z2 - z1 + 1
        z = max(heights[key] for key in product(range(x1, x2 + 1), range(y1, y2 + 1)))
        support = set(
            indexes[key]
            for key in product(range(x1, x2 + 1), range(y1, y2 + 1))
            if key in indexes and heights[key] == z
        )
        parents.append(support)
        for key in product(range(x1, x2 + 1), range(y1, y2 + 1)):
            indexes[key] = i
            heights[key] = z + dz

    n = len(parents)
    children = [[] for _ in range(n)]
    for i, js in enumerate(parents):
        for j in js:
            children[j].append(i)

    def dfs(i: int):
        remaining = list(map(len, parents))
        stack = [i]
        count = 0
        while stack:
            j = stack.pop()
            for k in children[j]:
                remaining[k] -= 1
                if remaining[k] == 0:
                    stack.append(k)
                    count += 1
        return count

    return sum(dfs(i) for i in range(len(parents)))




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_22.txt").read_text()

def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 5


def test_part_1_input():
    assert part_1(_read_input().rstrip()) == 393


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 7


def test_part_2_input():
    assert part_2(_read_input().rstrip()) == 58440


EXAMPLE_1 = """1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"""
