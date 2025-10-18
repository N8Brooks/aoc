package day17

import (
	"testing"

	"github.com/N8Brooks/aoc/go/util/input"
)

const Example = `>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>`

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example, 3068},
		{input.Input(2022, 17), 3098},
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
		{Example, 1514285714288},
		{input.Input(2022, 17), 1525364431487},
	}
	for i, c := range cases {
		actual := Part2(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %d, expected %d", i, actual, c.expected)
		}
	}
}
