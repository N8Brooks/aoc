from typing import Optional


def part_1(input: str) -> int:
    return sum(map(reflection_1, input.split("\n\n")))  # type: ignore


def reflection_1(input: str) -> Optional[int]:
    pattern = input.splitlines()

    for i in range(1, len(pattern)):
        fwd = pattern[:i][::-1]
        rev = pattern[i:]
        if all(x == y for x, y in zip(fwd, rev)):
            return (i) * 100

    for j in range(1, len(pattern[0])):
        fwd = [line[:j][::-1] for line in pattern]
        rev = [line[j:] for line in pattern]
        if all(x.startswith(y) or y.startswith(x) for x, y in zip(fwd, rev)):
            return j


def reflection_2(input: str, x: int, y: int) -> Optional[int]:
    pattern = input.splitlines()

    for i in range(1, len(pattern)):
        fwd = pattern[:i][::-1]
        rev = pattern[i:]
        start = i - 1 - min(len(fwd), len(rev))
        end = i + min(len(fwd), len(rev))
        if not start < x < end:
            continue
        if all(x == y for x, y in zip(fwd, rev)):
            return (i) * 100

    for j in range(1, len(pattern[0])):
        fwd = [line[:j][::-1] for line in pattern]
        rev = [line[j:] for line in pattern]
        start = j - 1 - min(len(fwd[0]), len(rev[0]))
        end = j + min(len(fwd[0]), len(rev[0]))
        if not start < y < end:
            continue
        if all(x.startswith(y) or y.startswith(x) for x, y in zip(fwd, rev)):
            return j


def part_2(input: str) -> int:
    total = 0
    for pattern in input.split("\n\n"):
        pattern = list(map(list, pattern.splitlines()))
        for i in range(len(pattern)):
            for j in range(len(pattern[0])):
                pattern[i][j] = "." if pattern[i][j] == "#" else "#"
                x = reflection_2("\n".join("".join(line) for line in pattern), i, j)
                if x is not None:
                    total += x
                    break
                pattern[i][j] = "." if pattern[i][j] == "#" else "#"
            else:
                continue
            break
    return total


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 405


def test_part_1_input():
    with open("../testdata/year_2023/day_13.txt", "r") as f:
        assert part_1(f.read()) == 41859


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 400


def test_part_2_input():
    # NOT: 17918
    with open("../testdata/year_2023/day_13.txt", "r") as f:
        assert part_2(f.read()) == 30842


EXAMPLE_1 = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""
