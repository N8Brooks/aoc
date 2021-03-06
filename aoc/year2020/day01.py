#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
https://adventofcode.com/2020/day/1
"""


from operator import mul

from data.utils import get_input


def part1(text, total=2020):
    def add(entry):
        if total - entry in entries:
            return True
        entries.add(entry)

    entries = set()

    return next((total - x) * x for x in sorted(map(int, text.split())) if add(x))


def part2(text, total=2020):
    def add(entry):
        if total - entry in twos:
            return True
        twos.update((entry + other, (entry, other)) for other in ones)
        ones.add(entry)

    def fix(entry):
        return mul(*twos[total - entry]) * entry

    ones = set()
    twos = {}

    return next(fix(x) for x in sorted(map(int, text.split())) if add(x))


if __name__ == "__main__":  # pragma: no cover
    text = get_input(2020, 1)

    print(part1(text))
    print(part2(text))
