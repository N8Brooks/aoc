from typing import Callable


# https://www.reddit.com/r/adventofcode/comments/18l0qtr/comment/kduuicl/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button


def compute_area(
    input: str, parse_instruction: Callable[[str], tuple[int, int, int]]
) -> int:
    y = perimiter = area = 0
    for line in input.splitlines():
        dx, dy, distance = parse_instruction(line)
        y += dy * distance
        perimiter += distance
        area += y * (distance * dx)
    return area + perimiter // 2 + 1


def part_1(input: str) -> int:
    return compute_area(input, parse_instruction_1)


def parse_instruction_1(line: str) -> tuple[int, int, int]:
    direction_str, distance_str, _ = line.split()
    dx = dy = 0
    match direction_str:
        case "R":
            dx = 1
        case "L":
            dx = -1
        case "U":
            dy = 1
        case "D":
            dy = -1
    distance = int(distance_str)
    return dx, dy, distance


def part_2(input: str) -> int:
    return compute_area(input, parse_instruction_2)


def parse_instruction_2(line: str) -> tuple[int, int, int]:
    _, _, color = line.split()
    dx = dy = 0
    match color[7]:
        case "0":
            dx = 1
        case "1":
            dy = -1
        case "2":
            dx = -1
        case "3":
            dy = 1
    distance = int(color[2:7], 16)
    return dx, dy, distance


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 62


def test_part_1_input():
    with open("../testdata/year_2023/day_18.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 40131


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 952408144115


def test_part_2_input():
    with open("../testdata/year_2023/day_18.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 104454050898331


EXAMPLE_1 = """R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"""
