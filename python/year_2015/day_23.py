"""
https://adventofcode.com/2015/day/23
"""


from pathlib import Path
from functools import partial
from itertools import takewhile
from operator import gt

from iteration_utilities import applyfunc, consume

def compute(text, a, b, target):
    def process(line):
        instruction, *operands = line.replace(",", "").split()
        if instruction == "jmp":
            return instruction, None, int(operands[0]) - 1
        elif len(operands) == 1:
            return instruction, operands[0], None
        else:
            return instruction, operands[0], int(operands[1]) - 1

    def execute(pointer):
        instruction, register, offset = program[pointer]

        if instruction == "hlf":
            registers[register] //= 2
        elif instruction == "tpl":
            registers[register] *= 3
        elif instruction == "inc":
            registers[register] += 1
        elif instruction == "jmp":
            pointer += offset
        elif instruction == "jie" and registers[register] % 2 == 0:
            pointer += offset
        elif instruction == "jio" and registers[register] == 1:
            pointer += offset

        return pointer + 1

    registers = {"a": a, "b": b}
    program = tuple(map(process, text.splitlines()))

    consume(takewhile(partial(gt, len(program)), applyfunc(execute, 0)), None)

    return registers[target]


def part_1(text, a=0, b=0, target="b"):
    return compute(text, a, b, target)


def part_2(text, a=1, b=0, target="b"):
    return compute(text, a, b, target)


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_23.txt").read_text()


EXAMPLE = """inc a
jio a, +2
tpl a
inc a
"""


def test_part_1_input():
    assert part_1(_read_input()) == 307

def test_part_1_example_a():
    assert part_1(EXAMPLE, target='a') == 2

def test_part_1_example_b():
    assert part_1(EXAMPLE, target='b') == 0

def test_part_2_input():
    assert part_2(_read_input()) == 160

def test_part_2_example_a():
    assert part_2(EXAMPLE, a=2, target='a') == 10

def test_part_2_example_b():
    assert part_2(EXAMPLE, target='b') == 0
