#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
https://adventofcode.com/2015/day/3
"""


from itertools import accumulate, islice

from data.utils import get_input


def direction(char):
    if char == "^":
        return 1j
    elif char == ">":
        return 1
    elif char == "v":
        return -1j
    elif char == "<":
        return -1


def part1(text):
    return len(set({0} | set(accumulate(map(direction, text)))))


def part2(text):
    locations = {0}
    locations.update(accumulate(map(direction, islice(text, 0, None, 2))))
    locations.update(accumulate(map(direction, islice(text, 1, None, 2))))

    return len(locations)


if __name__ == "__main__":  # pragma: no cover
    text = get_input(2015, 3)

    print(part1(text))
    print(part2(text))
