from itertools import chain, repeat


def part_1(input: str) -> int:
    return countEnergized(input.splitlines(), (0, 0, 0, 1))


def part_2(input: str) -> int:
    lines = input.splitlines()
    m = len(lines)
    n = len(lines[0])
    row_0 = zip(repeat(0), range(n), repeat(1), repeat(0))
    row_1 = zip(repeat(m - 1), range(n), repeat(-1), repeat(0))
    col_0 = zip(range(m), repeat(0), repeat(0), repeat(1))
    col_1 = zip(range(m), repeat(n - 1), repeat(0), repeat(-1))
    return max(map(countEnergized, repeat(lines), chain(row_0, row_1, col_0, col_1)))


def countEnergized(lines: list[str], init: tuple[int, int, int, int]) -> int:
    m = len(lines)
    n = len(lines[0])
    stack: list[tuple[int, int, int, int]] = [init]
    seen = set()
    while stack:
        i1, j1, di, dj = stack.pop()
        if not 0 <= i1 < m or not 0 <= j1 < n or (i1, j1, di, dj) in seen:
            continue
        seen.add((i1, j1, di, dj))
        char = lines[i1][j1]
        match char:
            case "/":
                stack.append((i1 - dj, j1 - di, -dj, -di))
            case "\\":
                stack.append((i1 + dj, j1 + di, dj, di))
            case "|" if dj != 0:
                stack.append((i1 + 1, j1, 1, 0))
                stack.append((i1 - 1, j1, -1, 0))
            case "-" if di != 0:
                stack.append((i1, j1 + 1, 0, 1))
                stack.append((i1, j1 - 1, 0, -1))
            case _:
                stack.append((i1 + di, j1 + dj, di, dj))
    return len(set((i1, j1) for i1, j1, _, _ in seen))


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 46


def test_part_1_input():
    with open("../test_data/year_2023/day_16.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 7046


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 51


def test_part_2_input():
    with open("../test_data/year_2023/day_16.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 7313


EXAMPLE_1 = """.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."""
