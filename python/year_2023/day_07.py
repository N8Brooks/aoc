from collections import Counter
from typing import Callable
from functools import reduce


def total_winnings(input: str, rank: Callable[[str], int]) -> int:
    hand_bids = (line.split(" ") for line in input.rstrip().splitlines())
    hand_bids = [(rank(hand), int(bid)) for hand, bid in hand_bids]
    hand_bids.sort(key=lambda hand_bid: hand_bid[0])
    return sum(rank * bid for rank, (_, bid) in enumerate(hand_bids, 1))


def part_1(input: str) -> int:
    return total_winnings(input, rank_1)


def rank_1(hand: str) -> int:
    counter = Counter(hand)
    counts = iter(sorted(counter.values(), reverse=True))
    mode_1 = next(counts)
    mode_2 = next(counts, 0)
    hand_type = HAND_TYPES_1.index((mode_1, mode_2))
    return reduce(lambda a, b: a * 13 + b, map(CARDS_1.index, hand), hand_type)


CARDS_1 = "23456789TJQKA"

HAND_TYPES_1 = [
    (1, 1),  # High Card
    (2, 1),  # One Pair
    (2, 2),  # Two Pair
    (3, 1),  # Three of a Kind
    (3, 2),  # Full House
    (4, 1),  # Four of a Kind
    (5, 0),  # Five of a Kind
]


def part_2(input: str) -> int:
    return total_winnings(input, rank_2)


def rank_2(hand: str) -> int:
    counter = Counter(hand)
    wildcards = counter.pop("J", 0)
    counts = iter(sorted(counter.values(), reverse=True))
    mode_1 = next(counts, 0) + wildcards
    mode_2 = next(counts, 0)
    hand_type = HAND_TYPES_2.index((mode_1, mode_2))
    return reduce(lambda a, b: a * 13 + b, map(CARDS_2.index, hand), hand_type)


CARDS_2 = "J23456789TQKA"

HAND_TYPES_2 = [
    (1, 1),  # High Card
    (2, 1),  # One Pair
    (2, 2),  # Two Pair
    (3, 1),  # Three of a Kind
    (3, 2),  # Full House
    (4, 1),  # Four of a Kind
    (5, 0),  # Five of a Kind
]


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
