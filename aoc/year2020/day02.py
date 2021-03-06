#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
https://adventofcode.com/2020/day/2
"""


import re

from data.utils import get_input


def part1(text):
    def valid(line):
        lo, hi, char, password = r.match(line).groups()
        return int(lo) <= password.count(char) <= int(hi)

    r = re.compile(r"(\d+)-(\d+) (\w): (\w+)")

    return sum(map(valid, text.splitlines()))


def part2(text):
    def valid(line):
        lo, hi, char, password = r.match(line).groups()
        return (char == password[int(lo) - 1]) != (char == password[int(hi) - 1])

    r = re.compile(r"(\d+)-(\d+) (\w): (\w+)")

    return sum(map(valid, text.splitlines()))


if __name__ == "__main__":  # pragma: no cover
    text = get_input(2020, 2)

    print(part1(text))
    print(part2(text))
