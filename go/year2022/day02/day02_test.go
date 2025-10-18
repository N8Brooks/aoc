package day02

import (
	"testing"

	"github.com/N8Brooks/aoc/go/util/input"
)

const Example = `A Y
B X
C Z`

func TestPlayAgainst(t *testing.T) {
	cases := []struct {
		player   Shape
		opponent Shape
		expected Outcome
	}{
		{Rock, Rock, Draw},
		{Rock, Paper, Loose},
		{Rock, Scissors, Win},
		{Paper, Rock, Win},
		{Paper, Paper, Draw},
		{Paper, Scissors, Loose},
		{Scissors, Rock, Loose},
		{Scissors, Paper, Win},
		{Scissors, Scissors, Draw},
	}
	for _, c := range cases {
		actual := c.player.playAgainst(c.opponent)
		if actual != c.expected {
			t.Errorf("%q.PlayAgainst(%q) == %q, expected %q", c.player, c.opponent, actual, c.expected)
		}
	}
}

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example, 15},
		{input.Input(2022, 2), 12645},
	}
	for i, c := range cases {
		actual := Part1(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %d, expected %d", i, actual, c.expected)
		}
	}
}

func TestPart2(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example, 12},
		{input.Input(2022, 2), 11756},
	}
	for i, c := range cases {
		actual := Part2(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %d, expected %d", i, actual, c.expected)
		}
	}
}
