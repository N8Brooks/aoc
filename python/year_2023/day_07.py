from typing import Callable
from functools import reduce
from heapq import nlargest


def part_1(input: str) -> int:
    return total_winnings(input, hand_strength_1)


def hand_strength_1(hand: str) -> int:
    counts = map(hand.count, CARDS_1)
    count_1, count_2 = nlargest(2, counts)
    hand_type_strength = HANDS.index((count_1, count_2))
    card_strengths = map(CARDS_1.index, hand)
    return reduce(lambda a, b: a * 13 + b, card_strengths, hand_type_strength)


CARDS_1 = "23456789TJQKA"


def part_2(input: str) -> int:
    return total_winnings(input, hand_strength_2)


def hand_strength_2(hand: str) -> int:
    counts = map(hand.count, CARDS_2)
    wildcards = next(counts)  # J
    count_1, count_2 = nlargest(2, counts)
    hand_type_strength = HANDS.index((count_1 + wildcards, count_2))
    card_strengths = map(CARDS_2.index, hand)
    return reduce(lambda a, b: a * 13 + b, card_strengths, hand_type_strength)


CARDS_2 = "J23456789TQKA"


def total_winnings(input: str, hand_strength: Callable[[str], int]) -> int:
    hand_bids = (line.split(" ") for line in input.rstrip().splitlines())
    ranked_bids = sorted(hand_bids, key=lambda hand_bid: hand_strength(hand_bid[0]))
    return sum(rank * int(bid) for rank, (_, bid) in enumerate(ranked_bids, 1))


HANDS = [
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
