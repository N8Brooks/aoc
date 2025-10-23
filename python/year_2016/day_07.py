"""
https://adventofcode.com/2016/day/7
"""

from itertools import starmap
from pathlib import Path

from iteration_utilities import flatten, successive


def process(ip):
    def sort(part):
        *hyp, ext = part.split("]")
        return hyp, ext

    hyps, exts = zip(*map(sort, ip.split("[")))

    return flatten(hyps), exts


def part_1(text):
    def abba(a, b, c, d):
        return a == d and b == c and a != b

    def abbas(ip):
        return any(starmap(abba, successive(ip, 4)))

    def tls(hyps, exts):
        return not any(map(abbas, hyps)) and any(map(abbas, exts))

    return sum(starmap(tls, map(process, text.split())))


def part_2(text):
    def aba(abc):
        return abc[0] == abc[2] and abc[0] != abc[1]

    def abas(ip):
        return filter(aba, successive(ip, 3))

    def rev(a, b, _):
        return (b, a, b)

    def ssl(hyps, exts):
        inside = set(flatten(map(abas, hyps)))
        return not inside.isdisjoint(starmap(rev, flatten(map(abas, exts))))

    return sum(starmap(ssl, map(process, text.strip().split())))


def _read_input() -> str:
    return (Path(__file__).with_name("test_data") / "day_07.txt").read_text()


EXAMPLE_1 = """abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
"""

EXAMPLE_2 = """aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
"""


def test_part_1_input():
    assert part_1(_read_input()) == 115


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 2


def test_part_1_example_2():
    assert part_1(EXAMPLE_2) == 0


def test_part_2_input():
    assert part_2(_read_input()) == 231


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 0


def test_part_2_example_2():
    assert part_2(EXAMPLE_2) == 3
