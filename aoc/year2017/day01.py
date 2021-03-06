#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
https://adventofcode.com/2017/day/1
"""


from data.utils import get_input


def process(func):
    def wrapper(text):
        return func(tuple(map(int, text.strip())))

    return wrapper


@process
def part1(processed):
    def match(i):
        return processed[i] == processed[i - 1]

    return sum(processed[i] for i in filter(match, range(len(processed))))


@process
def part2(processed):
    def match(i, halfway=len(processed) // 2):
        return processed[i] == processed[i - halfway]

    return sum(processed[i] for i in filter(match, range(len(processed))))


if __name__ == "__main__":  # pragma: no cover
    text = get_input(2017, 1)

    print(part1(text))
    print(part2(text))
