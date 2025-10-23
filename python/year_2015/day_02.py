"""
https://adventofcode.com/2015/day/2
"""


from pathlib import Path
def process(text):
    return (sorted(map(int, line.split("x"))) for line in text.split())


def part_1(text):
    return sum(3 * a * b + 2 * (a * c + b * c) for a, b, c in process(text))


def part_2(text):
    return sum(a + a + b + b + a * b * c for a, b, c in process(text))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_02.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 1606483

def test_part_1_example_1():
    assert part_1('2x3x4') == 58

def test_part_1_example_2():
    assert part_1('1x1x10') == 43

def test_part_2_input():
    assert part_2(_read_input()) == 3842356

def test_part_2_example_1():
    assert part_2('2x3x4') == 34

def test_part_2_example_2():
    assert part_2('1x1x10') == 14
