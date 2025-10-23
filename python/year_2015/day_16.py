"""
https://adventofcode.com/2015/day/16
"""


from pathlib import Path
import re

TARGET = {
    "children": "3",
    "cats": "7",
    "samoyeds": "2",
    "pomeranians": "3",
    "akitas": "0",
    "vizslas": "0",
    "goldfish": "5",
    "trees": "3",
    "cars": "2",
    "perfumes": "1",
}


def find_sue(text, valid_prop):
    def valid_sue(line):
        return all(map(valid_prop, r.sub("", line).split(", ")))

    r = re.compile(r"Sue (\d+): ")

    sue = next(filter(valid_sue, text.splitlines()))

    return int(r.match(sue).groups()[0])


def part_1(text):
    def valid_prop(record):
        key, value = record.split(": ")
        return TARGET[key] == value

    return find_sue(text, valid_prop)


def part_2(text):
    def valid_prop(record):
        key, value = record.split(": ")
        if key == "cats" or key == "trees":
            return TARGET[key] < value
        elif key == "pomeranians" or key == "goldfish":
            return value < TARGET[key]

        return TARGET[key] == value

    return find_sue(text, valid_prop)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_16.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 213

def test_part_2_input():
    assert part_2(_read_input()) == 323
