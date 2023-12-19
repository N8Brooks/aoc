def part_1(input: str) -> int:
    loc = 0j
    edges = set()
    for line in input.splitlines():
        direction, distance = parse_instruction_1(line)
        for _ in range(distance):
            loc += direction
            edges.add(loc)

    min_x = min(int(loc.real) for loc in edges) - 1
    max_x = max(int(loc.real) for loc in edges) + 2
    min_y = min(int(loc.imag) for loc in edges) - 1
    max_y = max(int(loc.imag) for loc in edges) + 2

    outside = set()

    stack = [min_x + min_y * 1j]

    while stack:
        loc = stack.pop()
        outside.add(loc)
        for delta in (1, -1, 1j, -1j):
            new_loc = loc + delta
            if (
                min_x <= new_loc.real < max_x
                and min_y <= new_loc.imag < max_y
                and new_loc not in edges
                and new_loc not in outside
            ):
                stack.append(new_loc)

    return (max_y - min_y) * (max_x - min_x) - len(outside)


def parse_instruction_1(line: str) -> tuple[complex, int]:
    direction_str, distance_str, _ = line.split()
    match direction_str:
        case "R":
            direction = 1
        case "L":
            direction = -1
        case "U":
            direction = -1j
        case "D":
            direction = 1j
        case _:
            raise ValueError(f"Unknown direction {direction_str}")
    distance = int(distance_str)
    return direction, distance


def part_2(input: str) -> int:
    return 0


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 62


def test_part_1_input():
    with open("../testdata/year_2023/day_18.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 40131


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 0


def test_part_2_input():
    with open("../testdata/year_2023/day_18.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 0


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
