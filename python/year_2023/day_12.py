from pathlib import Path
from functools import cache
from itertools import repeat


def part_1(input: str) -> int:
    return sum(arrangements(line, 1) for line in input.rstrip().splitlines())


def part_2(input: str) -> int:
    return sum(arrangements(line, 5) for line in input.rstrip().splitlines())


def arrangements(line: str, r: int) -> int:
    @cache
    def visit(condition_idx: int, group_idx: int) -> int:
        group = groups[group_idx]
        group_idx += 1
        j = sum(groups[group_idx:]) + n - group_idx
        count = 0
        for i in range(condition_idx, m - j - group + 1):
            if "." not in conditions[i : i + group]:
                if group_idx == n:
                    count += "#" not in conditions[i + group :]
                elif conditions[i + group] != "#":
                    count += visit(i + group + 1, group_idx)
            if conditions[i] == "#":
                break
        return count

    conditions, _, groups_str = line.partition(" ")
    conditions = "?".join(repeat(conditions, r))
    m = len(conditions)
    groups = tuple(map(int, groups_str.split(","))) * r
    n = len(groups)
    return visit(0, 0)




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_12.txt").read_text()

def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 21


def test_part_1_input():
    assert part_1(_read_input()) == 7490


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 525152


def test_part_2_input():
    assert part_2(_read_input()) == 65607131946466


EXAMPLE_1 = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""
