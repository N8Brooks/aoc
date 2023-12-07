from collections import Counter
from typing import Callable


type Rank = list[tuple[int, ...]]


def total_winnings(input: str, rank: Callable[[str], Rank]) -> int:
    hand_bids = (line.split(" ") for line in input.rstrip().splitlines())
    hand_bids = [(rank(hand), int(bid)) for hand, bid in hand_bids]
    hand_bids.sort(key=lambda hand_bid: hand_bid[0])
    return sum(rank * bid for rank, (_, bid) in enumerate(hand_bids, 1))


def part_1(input: str) -> int:
    return total_winnings(input, rank_1)


def rank_1(hand: str) -> Rank:
    counter = Counter(hand)
    counts = sorted(counter.values(), reverse=True)
    indexes = tuple(map(CARDS_1.index, hand))
    value = [tuple() for _ in range(7)]
    match counts:
        case [5]:
            value[0] = indexes
        case [4, 1]:
            value[1] = indexes
        case [3, 2]:
            value[2] = indexes
        case [3, 1, 1]:
            value[3] = indexes
        case [2, 2, 1]:
            value[4] = indexes
        case [2, 1, 1, 1]:
            value[5] = indexes
        case _:
            value[6] = indexes
    return value


CARDS_1 = "23456789TJQKA"


def part_2(input: str) -> int:
    return total_winnings(input, rank_2)


def rank_2(hand: str):
    counter = Counter(hand)
    wildcards = counter.pop("J", 0)
    counts = sorted(counter.values(), reverse=True)
    indexes = tuple(map(CARDS_2.index, hand))
    value = [tuple() for _ in range(7)]
    if wildcards == 5 or counts[0] + wildcards == 5:
        value[0] = indexes
    elif counts[0] + wildcards == 4:
        value[1] = indexes
    elif counts[0] + wildcards >= 3 and counts[1] + (counts[0] + wildcards - 3) >= 2:
        value[2] = indexes
    elif counts[0] + wildcards == 3:
        value[3] = indexes
    elif counts[0] + wildcards >= 2 and counts[1] + (counts[0] + wildcards - 2) >= 2:
        value[4] = indexes
    elif counts[0] + wildcards == 2:
        value[5] = indexes
    else:
        value[6] = indexes
    return value


CARDS_2 = "J23456789TQKA"


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 6440


def test_part_1_input():
    with open("../testdata/year_2023/day_07.txt", "r") as f:
        assert part_1(f.read()) == 248453531


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 5905


def test_part_2_input():
    with open("../testdata/year_2023/day_07.txt", "r") as f:
        assert part_2(f.read()) == 248781813


EXAMPLE_1 = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""
