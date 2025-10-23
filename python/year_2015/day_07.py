"""
https://adventofcode.com/2015/day/7
"""


from pathlib import Path
from collections import deque

from iteration_utilities import is_None

COMP = (1 << 16) - 1


def process(text):
    def cast(operand):
        return operand if operand.isalpha() else int(operand)

    def wrap(line):
        operands, dest = line.split(" -> ")
        return tuple(map(cast, operands.split())), dest

    return map(wrap, text.splitlines())


def compute(q, request):
    def val(operand):
        if isinstance(operand, str):
            return reg.get(operand, None)
        else:
            return int(operand)

    def move(exp, dest):
        if is_None(res := val(exp[0])):
            q.append((exp, dest))
        else:
            reg[dest] = res

    def complement(exp, dest):
        if is_None(res := val(exp[1])):
            q.append((exp, dest))
        else:
            reg[dest] = COMP ^ res

    def instruct(exp, dest):
        if is_None(res1 := val(exp[0])) or is_None(res2 := val(exp[2])):
            q.append((exp, dest))
            return
        if exp[1] == "AND":
            reg[dest] = res1 & res2
        elif exp[1] == "OR":
            reg[dest] = res1 | res2
        elif exp[1] == "LSHIFT":
            reg[dest] = res1 << res2
        elif exp[1] == "RSHIFT":
            reg[dest] = res1 >> res2

    def operate(exp, dest):
        if len(exp) == 1:
            move(exp, dest)
        elif len(exp) == 2:
            complement(exp, dest)
        else:
            instruct(exp, dest)

    reg = {}

    while q:
        operate(*q.popleft())

    return reg.get(request, None)


def part_1(text, request="a"):
    return compute(deque(process(text)), request)


def part_2(text, request="a"):
    q = deque(filter(lambda line: line[1] != "b", process(text)))
    q.appendleft(((part_1(text, request),), "b"))

    return compute(q, request)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_07.txt").read_text()


EXAMPLE = """123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
"""


def test_part_1_input():
    assert part_1(_read_input()) == 16076

def test_part_1_example_d():
    assert part_1(EXAMPLE, 'd') == 72

def test_part_1_example_e():
    assert part_1(EXAMPLE, 'e') == 507

def test_part_1_example_f():
    assert part_1(EXAMPLE, 'f') == 492

def test_part_1_example_g():
    assert part_1(EXAMPLE, 'g') == 114

def test_part_1_example_h():
    assert part_1(EXAMPLE, 'h') == 65412

def test_part_1_example_i():
    assert part_1(EXAMPLE, 'i') == 65079

def test_part_1_example_x():
    assert part_1(EXAMPLE, 'x') == 123

def test_part_1_example_y():
    assert part_1(EXAMPLE, 'y') == 456

def test_part_2_input():
    assert part_2(_read_input()) == 2797
