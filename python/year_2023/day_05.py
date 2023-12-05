from itertools import chain
from bisect import bisect_left, bisect_right


def part_1(input: str) -> int:
    seeds, *maps = input.split("\n\n")
    _, _, seeds = seeds.partition(": ")
    nums = list(map(int, seeds.split()))
    for indexes in map(parse_map, maps):
        nums[:] = (
            next((j + num - i for j, i, n in indexes if i <= num < i + n), num)
            for num in nums
        )
    return min(nums)


def part_2(input: str) -> int:
    seeds, *maps = input.split("\n\n")
    _, _, seeds = seeds.partition(": ")
    seeds = map(int, seeds.split())
    intervals = RangeModule()
    for i2, n in zip(seeds, seeds):
        intervals.addRange(i2, i2 + n)
    for indexes in map(parse_map, maps):
        next_intervals = RangeModule()
        for i1, j1 in zip(intervals.intervals[::2], intervals.intervals[1::2]):
            for dst_i, src_i, n in indexes:
                i2 = max(i1, src_i)
                j2 = min(j1, src_i + n)
                if j2 <= i2:
                    continue
                intervals.removeRange(i2, j2)
                diff = dst_i - src_i
                next_intervals.addRange(i2 + diff, j2 + diff)
        indexes = iter(intervals.intervals)
        for i2, j2 in zip(indexes, indexes):
            next_intervals.addRange(i2, j2)
        intervals = next_intervals
    return intervals.intervals[0]


def parse_map(lines: str):
    return [tuple(map(int, row.split())) for row in lines.splitlines()[1:]]


class RangeModule:
    def __init__(self):
        self.intervals = []

    def addRange(self, left: int, right: int) -> None:
        # [start0:stop0, start1:stop1] -> [min(start0, left):max(stop1, right)]
        i = bisect_left(self.intervals, left)
        j = bisect_right(self.intervals, right, i)
        self.intervals[i:j] = chain([] if i & 1 else [left], [] if j & 1 else [right])

    def queryRange(self, left: int, right: int) -> bool:
        # Checks start < left < stop and start < right <= stop
        i = bisect_right(self.intervals, left)
        j = bisect_left(self.intervals, right)
        return i == j and bool(i & 1)

    def removeRange(self, left: int, right: int) -> None:
        # [start0:stop0, start1:stop1] -> [max(stop0, left), min(start1, right)]
        i = bisect_left(self.intervals, left)
        j = bisect_right(self.intervals, right, i)
        self.intervals[i:j] = chain([left] if i & 1 else [], [right] if j & 1 else [])


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 35


def test_part_1_input():
    with open("../testdata/year_2023/day_05.txt", "r") as f:
        assert part_1(f.read()) == 323142486


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 46


def test_part_2_input():
    with open("../testdata/year_2023/day_05.txt", "r") as f:
        assert part_2(f.read()) == 79874951


EXAMPLE_1 = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""
