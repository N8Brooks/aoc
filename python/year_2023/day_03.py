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
            if any((i, j) in symbols for i, j in surrounding):
                total += int(number[0])
    return total


def part_2(input: str) -> int:
    lines = input.rstrip().splitlines()
    total = 0
    all_nums = {}
    for i1, line in enumerate(lines):
        for match in re.finditer(r"\d+", line):
            domain = zip(repeat(i1), range(match.start(), match.end()))
            all_nums |= dict.fromkeys(domain, match)
    for i1, line in enumerate(lines):
        for j2, char in enumerate(line):
            if char != "*":
                continue
            nums = []
            for i2, j2 in product(range(i1 - 1, i1 + 2), range(j2 - 1, j2 + 2)):
                num = all_nums.get((i2, j2), None)
                if num is None or num in nums:
                    continue
                nums.append(num)
            if len(nums) == 2:
                num1, num2 = nums
                total += int(num1[0]) * int(num2[0])
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
