def part_1(input: str) -> int:
    loop, _ = parse_loop(input)
    return (len(loop) + 1) // 2


def part_2(input: str) -> int:
    loop, lines = parse_loop(input)
    m = len(lines)
    n = len(lines[0])
    total = 0

    for i in range(m):
        parity = 0
        for i, j in zip(range(i, m), range(n)):
            if (i, j) in loop:
                if lines[i][j] in CHARS:
                    parity ^= 1
            else:
                total += parity

    for j in range(1, n):
        parity = 0
        for i, j in zip(range(m), range(j, n)):
            if (i, j) in loop:
                if lines[i][j] in CHARS:
                    parity ^= 1
            else:
                total += parity

    return total


CHARS = "-|JF"


def parse_loop(input: str) -> tuple[set[tuple[int, int]], list[str]]:
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
            case _:
                raise ValueError(f"Unknown character {lines[i][j]} at {i}, {j}")

    init = a = i, j = next(
        (i, j)
        for i, line in enumerate(lines)
        for j, char in enumerate(line)
        if char == "S"
    )
    seen = {a}
    b = next(
        (i, j)
        for i, j in (
            (i - 1, j),
            (i, j + 1),
            (i + 1, j),
            (i, j - 1),
        )
        if lines[i][j] != "." and init in neighbors(i, j)
    )
    while b != init:
        seen.add(b)
        c, d = neighbors(*b)
        a, b = b, c if a != c else d

    # Fix "S" character
    up = lines[i - 1][j] in "|7F"
    down = lines[i + 1][j] in "|LJ"
    left = lines[i][j - 1] in "-LF"
    right = lines[i][j + 1] in "-J7"

    r = ""
    if up and down:
        r = "|"
    elif left and right:
        r = "-"
    elif up and right:
        r = "L"
    elif up and left:
        r = "J"
    elif down and right:
        r = "F"
    elif down and left:
        r = "7"
    lines[i] = lines[i].replace("S", r, 1)

    return seen, lines


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
