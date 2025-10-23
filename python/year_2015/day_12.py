"""
https://adventofcode.com/2015/day/12
"""


from pathlib import Path
from json import loads

def part_1(text):
    def parse(node):
        if isinstance(node, int):
            return node
        elif isinstance(node, list):
            return sum(map(parse, node))
        elif isinstance(node, dict):
            return sum(map(parse, node.values()))
        else:
            return 0

    return parse(loads(text))


def part_2(text):
    def parse(node):
        if isinstance(node, int):
            return node
        elif isinstance(node, list):
            return sum(map(parse, node))
        elif isinstance(node, dict):
            return "red" not in node.values() and sum(map(parse, node.values()))
        else:
            return 0

    return parse(loads(text))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_12.txt").read_text()


def test_part_1_input():
    assert part_1(_read_input()) == 191164

def test_part_1_example_1():
    assert part_1('[1,2,3]') == 6

def test_part_1_example_2():
    assert part_1('{"a":2,"b":4}') == 6

def test_part_1_example_3():
    assert part_1('[[[3]]]') == 3

def test_part_1_example_4():
    assert part_1('{"a":{"b":4},"c":-1}') == 3

def test_part_1_example_5():
    assert part_1('{"a":[-1,1]}') == 0

def test_part_1_example_6():
    assert part_1('[-1,{"a":1}]') == 0

def test_part_1_example_7():
    assert part_1('[]') == 0

def test_part_1_example_8():
    assert part_1('{}') == 0

def test_part_2_input():
    assert part_2(_read_input()) == 87842

def test_part_2_example_1():
    assert part_2('[1,2,3]') == 6

def test_part_2_example_2():
    assert part_2('[1,{"c":"red","b":2},3]') == 4

def test_part_2_example_3():
    assert part_2('{"d":"red","e":[1,2,3,4],"f":5}') == 0

def test_part_2_example_4():
    assert part_2('[1,"red",5]') == 6
