from itertools import chain, repeat, product
import re


def part_1(input: str) -> int:
    lines = input.rstrip().splitlines()
    symbols = {
        (i, j)
        for i, line in enumerate(lines)
        for j, char in enumerate(line)
        if char != "." and not char.isdigit()
    }
    total = 0
    for i, line in enumerate(lines):
        for number in re.finditer(r"\d+", line):
            j1 = number.start() - 1
            j2 = number.end() + 1
            surrounding = chain(
                zip(repeat(i - 1), range(j1, j2)),
                ((i, j1), (i, j2 - 1)),
                zip(repeat(i + 1), range(j1, j2)),
            )
            if any((i3, j3) in symbols for i3, j3 in surrounding):
                total += int(number[0])
    return total


def part_2(input: str) -> int:
    lines = input.rstrip().splitlines()
    total = 0
    numbers = {}
    for i, line in enumerate(lines):
        for match in re.finditer(r"\d+", line):
            domain = zip(repeat(i), range(match.start(), match.end()))
            numbers |= dict.fromkeys(domain, match)
    for i, line in enumerate(lines):
        for j, char in enumerate(line):
            if char != "*":
                continue
            matches = []
            for i1, j1 in product(range(i - 1, i + 2), range(j - 1, j + 2)):
                number = numbers.get((i1, j1), None)
                if number is None or any(number is match for match in matches):
                    continue
                matches.append(number)
            if len(matches) == 2:
                a, b = matches
                total += int(a[0]) * int(b[0])
    return total


def test_part_1_example():
    assert part_1(EXAMPLE_1) == 4361


def test_part_1_input():
    with open("../testdata/year_2023/day_03.txt", "r") as f:
        assert part_1(f.read()) == 532331


def test_part_2_example():
    assert part_2(EXAMPLE_1) == 467835


def test_part_2_input():
    with open("../testdata/year_2023/day_03.txt", "r") as f:
        assert part_2(f.read()) == 82301120


EXAMPLE_1 = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""
