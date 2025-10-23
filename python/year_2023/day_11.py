from pathlib import Path

def part_1(input: str) -> int:
    return sum_distances_2d(input, 2)


def part_2(input: str) -> int:
    return sum_distances_2d(input, 1_000_000)


def sum_distances_2d(input: str, expansion: int) -> int:
    rows = input.rstrip().splitlines()
    row_counts = [0] * len(rows)
    col_counts = [0] * len(rows[0])
    for i, row in enumerate(rows):
        for j, char in enumerate(row):
            if char == "#":
                row_counts[i] += 1
                col_counts[j] += 1
    sum_row_distance = sum_distances_1d(row_counts, expansion)
    sum_col_distance = sum_distances_1d(col_counts, expansion)
    return sum_row_distance + sum_col_distance


def sum_distances_1d(counts: list[int], expansion: int) -> int:
    total = 0
    i = -sum(counts)
    distance = 0
    for count in counts:
        if count:
            total += distance * count * (i + count)
            i += count + count
            distance += 1
        else:
            distance += expansion
    return total




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_11.txt").read_text()

def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 374


def test_part_1_input():
    assert part_1(_read_input()) == 9370588


def test_part_2_input():
    assert part_2(_read_input()) == 746207878188


def test_sum_distance_2d_10():
    assert sum_distances_2d(EXAMPLE_1, 10) == 1030


def test_sum_distance_2d_100():
    assert sum_distances_2d(EXAMPLE_1, 100) == 8410


EXAMPLE_1 = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""
