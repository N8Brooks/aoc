from functools import cache
from itertools import repeat

# Based on: https://github.com/morgoth1145/advent-of-code/blob/6c4ef0a7f27f14993bc937ab8a7fabe736f73fe6/2023/12/solution.py


def part_1(input: str) -> int:
    return sum(arrangements(line, 1) for line in input.rstrip().splitlines())


def part_2(input: str) -> int:
    return sum(arrangements(line, 5) for line in input.rstrip().splitlines())


def arrangements(line: str, r: int) -> int:
    @cache
    def visit(conditions: str, n: int, groups: tuple[int, ...]) -> int:
        group = groups[0]
        groups = groups[1:]
        j = len(groups) + sum(groups)
        count = 0
        for i in range(n - j - group + 1):
            if "." not in conditions[i : i + group]:
                if not groups:
                    count += "#" not in conditions[i + group :]
                elif conditions[i + group] != "#":
                    count += visit(
                        conditions[i + group + 1 :], n - group - i - 1, groups
                    )
            if conditions[i] == "#":
                break
        return count

    conditions, _, groups = line.partition(" ")
    conditions = "?".join(repeat(conditions, r))
    groups = tuple(map(int, groups.split(","))) * r
    return visit(conditions, len(conditions), groups)


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 21


def test_part_1_input():
    with open("../testdata/year_2023/day_12.txt", "r") as f:
        assert part_1(f.read()) == 7490


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 525152


def test_part_2_input():
    with open("../testdata/year_2023/day_12.txt", "r") as f:
        assert part_2(f.read()) == 65607131946466


EXAMPLE_1 = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""
