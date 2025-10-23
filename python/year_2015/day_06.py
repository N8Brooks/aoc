"""
https://adventofcode.com/2015/day/6
"""


from pathlib import Path
import numpy as np
import re

def process(text, instruction, grid):
    r = re.compile(r"(\D+) (\d+),(\d+) through (\d+),(\d+)")

    for line in text.splitlines():
        instruction(*r.match(line).groups())

    return int(grid.sum())


def part_1(text, size=1000):
    def instruction(action, *coordinates):
        x1, y1, x2, y2 = map(int, coordinates)
        x2, y2 = (x2 + 1, y2 + 1)

        if action == "toggle":
            grid[y1:y2, x1:x2] = 1 - grid[y1:y2, x1:x2]
        elif action:
            grid[y1:y2, x1:x2] = action == "turn on"

    grid = np.zeros((size, size), bool)

    return process(text, instruction, grid)


def part_2(text, size=1000):
    def instruction(action, *coordinates):
        x1, y1, x2, y2 = map(int, coordinates)
        x2, y2 = (x2 + 1, y2 + 1)

        if action == "toggle":
            grid[y1:y2, x1:x2] += 2
        elif action == "turn on":
            grid[y1:y2, x1:x2] += 1
        else:
            grid[y1:y2, x1:x2] -= 1
            np.clip(grid[y1:y2, x1:x2], 0, None, grid[y1:y2, x1:x2])

    grid = np.zeros((size, size), int)

    return process(text, instruction, grid)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_06.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 543903

def test_part_1_example_1():
    assert part_1('turn on 0,0 through 999,999') == 1000000

def test_part_1_example_2():
    assert part_1('toggle 0,0 through 999,0') == 1000

def test_part_1_example_3():
    assert part_1('turn off 499,499 through 500,500') == 0

def test_part_2_input():
    assert part_2(_read_input(), 1000) == 14687245

def test_part_2_example_1():
    assert part_2('turn on 0,0 through 0,0') == 1

def test_part_2_example_2():
    assert part_2('toggle 0,0 through 999,999') == 2000000
