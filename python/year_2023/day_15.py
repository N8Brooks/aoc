from pathlib import Path
from functools import reduce


def part_1(input: str) -> int:
    return sum(map(hash, input.split(",")))


def part_2(input: str) -> int:
    boxes = [{} for _ in range(256)]
    for operation in input.split(","):
        if operation[-1] == "-":
            label = operation[:-1]
            box_idx = hash(label)
            boxes[box_idx].pop(label, None)
        else:
            label, _, focal_length = operation.partition("=")
            box_idx = hash(label)
            boxes[box_idx][label] = int(focal_length)
    return sum(
        box_idx * slot * focal_length
        for box_idx, box in enumerate(boxes, 1)
        for slot, focal_length in enumerate(box.values(), 1)
    )


def hash(input: str) -> int:
    return reduce(op, map(ord, input), 0) % 256


def op(value: int, code: int) -> int:
    return (value + code) * 17




def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_15.txt").read_text()

def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 1320


def test_part_1_input():
    assert part_1(_read_input().rstrip()) == 517315


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 145


def test_part_2_input():
    assert part_2(_read_input().rstrip()) == 247763


def test_hash():
    assert hash("HASH") == 52


EXAMPLE_1 = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"""
