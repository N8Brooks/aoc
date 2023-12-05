from itertools import chain
from bisect import bisect_left, bisect_right


def part_1(input: str) -> int:
    seeds, maps = parse_input(input)
    nums = list(seeds)
    for map_ in maps:
        nums[:] = (
            next((dst + num - src for dst, src, n in map_ if src <= num < src + n), num)
            for num in nums
        )
    return min(nums)


def part_2(input: str) -> int:
    seeds, maps = parse_input(input)

    # Initialize ranges from seeds
    a = RangeModule()
    for i, n in zip(seeds, seeds):
        a.add_range(i, i + n)

    for map_ in maps:
        b = RangeModule()

        # Map ranges across transformations
        for i, j in zip(a.intervals[::2], a.intervals[1::2]):
            for i_dst, i_src, n in map_:
                i_cut = max(i, i_src)
                j_cut = min(j, i_src + n)
                if j_cut <= i_cut:
                    continue
                a.remove_range(i_cut, j_cut)
                diff = i_dst - i_src
                b.add_range(i_cut + diff, j_cut + diff)

        # Clean up unmapped ranges
        map_ = iter(a.intervals)
        for i, j in zip(map_, map_):
            b.add_range(i, j)

        a = b

    return a.intervals[0]


def parse_input(input: str):
    seeds, *maps = input.split("\n\n")
    _, _, seeds = seeds.partition(": ")
    seeds = map(int, seeds.split())
    indexes = (
        [tuple(map(int, row.split())) for row in rows.splitlines()[1:]] for rows in maps
    )
    return seeds, indexes


class RangeModule:
    def __init__(self):
        self.intervals = []

    def add_range(self, left: int, right: int) -> None:
        # [start0:stop0, start1:stop1] -> [min(start0, left):max(stop1, right)]
        i = bisect_left(self.intervals, left)
        j = bisect_right(self.intervals, right, i)
        self.intervals[i:j] = chain([] if i & 1 else [left], [] if j & 1 else [right])

    def remove_range(self, left: int, right: int) -> None:
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
