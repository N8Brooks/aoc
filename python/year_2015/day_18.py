"""
https://adventofcode.com/2015/day/18
"""


from pathlib import Path
from functools import partial
from operator import eq

from iteration_utilities import applyfunc, nth
import numpy as np
from scipy.signal import convolve2d

def process(line):
    return tuple(map(partial(eq, "#"), line))


def part_1(text, steps=100):
    def step(grid):
        neighbors = convolve2d(grid, np.ones((3, 3)), "same") - grid
        return (neighbors == 3) | (grid & (neighbors == 2))

    grid = np.array(tuple(map(process, text.splitlines())))

    return (nth(steps - 1)(applyfunc(step, grid)) if steps else grid).sum()


def part_2(text, steps=100):
    def step(grid):
        neighbors = convolve2d(grid, np.ones((3, 3)), "same") - grid
        return (neighbors == 3) | (grid & (neighbors == 2)) | on

    grid = np.array(tuple(map(process, text.splitlines())))
    on = np.zeros_like(grid)
    on[0, 0] = on[0, -1] = on[-1, 0] = on[-1, -1] = True
    grid |= on

    return (nth(steps - 1)(applyfunc(step, grid)) if steps else grid).sum()


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_18.txt").read_text()


EXAMPLE = """.#.#.#
...##.
#....#
..#...
#.#..#
####..
"""


def test_part_1_input():
    assert part_1(_read_input()) == 1061

def test_part_1_example():
    assert part_1(EXAMPLE, 4) == 4

def test_part_2_input():
    assert part_2(_read_input()) == 1006

def test_part_2_example():
    assert part_2(EXAMPLE, 4) == 14
