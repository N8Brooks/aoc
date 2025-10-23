from pathlib import Path
from itertools import chain


def part_1(input: str) -> int:
    return sum(map(calibration_value_1, input.splitlines()))


def calibration_value_1(line: str) -> int:
    first = next(c for c in line if c.isdigit())
    last = next(c for c in reversed(line) if c.isdigit())
    return int(f"{first}{last}")


def part_2(input: str) -> int:
    return sum(map(calibration_value_2, input.splitlines()))


def calibration_value_2(line: str) -> int:
    first, _ = min(NUM_ITEMS, key=lambda item: lfind(line, item[1]))
    last, _ = max(NUM_ITEMS, key=lambda item: line.rfind(item[1]))
    return int(f"{first}{last}")


NUMS = tuple(map(str, range(1, 10)))

WORD_NUMS = ("one", "two", "three", "four", "five", "six", "seven", "eight", "nine")

NUM_ITEMS = tuple(chain(zip(NUMS, NUMS), zip(NUMS, WORD_NUMS)))


def lfind(word: str, sub: str) -> int:
    i = word.find(sub)
    return i if i != -1 else len(word)




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_01.txt").read_text()

def test_part_1_example():
    assert part_1(EXAMPLE_1) == 142


def test_part_1_input():
    assert part_1(_read_input()) == 52974


def test_part_2_example():
    assert part_2(EXAMPLE_2) == 281


def test_part_2_input():
    assert part_2(_read_input()) == 53340


EXAMPLE_1 = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""


EXAMPLE_2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen """
