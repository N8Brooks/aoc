"""
https://adventofcode.com/2017/day/9
"""

from pathlib import Path


def part_1(text):
    def garbage(i):
        while (char := text[i]) != ">":
            i += (char == "!") + 1
        return i

    depth = count = i = 0
    length = len(text)

    while i < length:
        if (char := text[i]) == "<":
            i = garbage(i + 1)
        elif char == "{":
            depth += 1
            count += depth
        elif char == "}":
            depth -= 1
        i += 1

    return count


def part_2(text):
    count = i = 0
    length = len(text)

    while i < length:
        if text[i] != "<":
            i += 1
            continue

        i += 1
        while (char := text[i]) != ">":
            if char == "!":
                i += 2
            else:
                count += 1
                i += 1
        i += 1

    return count


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_09.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 11089


def test_part_1_example_1():
    assert part_1("{}") == 1


def test_part_1_example_2():
    assert part_1("{{{}}}") == 6


def test_part_1_example_3():
    assert part_1("{{},{}}") == 5


def test_part_1_example_4():
    assert part_1("{{{},{},{{}}}}") == 16


def test_part_1_example_5():
    assert part_1("{<a>,<a>,<a>,<a>}") == 1


def test_part_1_example_6():
    assert part_1("{{<ab>},{<ab>},{<ab>},{<ab>}}") == 9


def test_part_1_example_7():
    assert part_1("{{<!!>},{<!!>},{<!!>},{<!!>}}") == 9


def test_part_1_example_8():
    assert part_1("{{<a!>},{<a!>},{<a!>},{<ab>}}") == 3


def test_part_2_input():
    assert part_2(_read_input()) == 5288


def test_part_2_example_1():
    assert part_2("<>") == 0


def test_part_2_example_2():
    assert part_2("<random characters>") == 17


def test_part_2_example_3():
    assert part_2("<<<<>") == 3


def test_part_2_example_4():
    assert part_2("<{!>}>") == 2


def test_part_2_example_5():
    assert part_2("<!!>") == 0


def test_part_2_example_6():
    assert part_2("<!!!>>") == 0


def test_part_2_example_7():
    assert part_2('<{o"i!a,<{i<a>') == 10
