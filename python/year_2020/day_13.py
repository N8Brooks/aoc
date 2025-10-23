"""
https://adventofcode.com/2020/day/13
"""

from pathlib import Path


def part_1(text):
    time, notes = text.splitlines()

    earliest = int(time)
    buses = (int(bus) for bus in notes.split(",") if bus != "x")
    minimum = min(buses, key=lambda bus: (bus - earliest) % bus)

    return (minimum - earliest) % minimum * minimum


def part_2(text):
    _, notes = text.splitlines()

    time, lcm = 0, 1
    for i, bus in enumerate(notes.split(",")):
        if bus == "x":
            continue

        bus = int(bus)
        k = -(time + i) * pow(lcm, -1, bus) % bus
        time += k * lcm
        lcm *= bus

    return time


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_13.txt").read_text()


EXAMPLE_1 = """939
7,13,x,x,59,x,31,19
"""

EXAMPLE_2 = """1234
17,x,13,19
"""

EXAMPLE_3 = """1337
67,7,59,61
"""

EXAMPLE_4 = """7331
67,x,7,59,61
"""

EXAMPLE_5 = """448
67,7,x,59,61
"""

EXAMPLE_6 = """8901
1789,37,47,1889
"""


def test_part_1_input():
    assert part_1(_read_input()) == 4722


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 295


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 13


def test_part_1_example_3():
    assert part_1(EXAMPLE_3) == 0


def test_part_1_example_4():
    assert part_1(EXAMPLE_4) == 35


def test_part_1_example_5():
    assert part_1(EXAMPLE_5) == 0


def test_part_1_example_6():
    assert part_1(EXAMPLE_6) == 592


def test_part_2_input():
    assert part_2(_read_input()) == 825305207525452


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 1068781


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 3417


def test_part_2_example_3():
    assert part_2(EXAMPLE_3) == 754018


def test_part_2_example_4():
    assert part_2(EXAMPLE_4) == 779210


def test_part_2_example_5():
    assert part_2(EXAMPLE_5) == 1261476


def test_part_2_example_6():
    assert part_2(EXAMPLE_6) == 1202161486
