package day08

import (
	"testing"

	"github.com/N8Brooks/aoc/go/util/input"
)

const Example = `30373
25512
65332
33549
35390`

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example, 21},
		{input.Input(2022, 8), 1785},
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
		{Example, 8},
		{input.Input(2022, 8), 345168},
	}
	for i, c := range cases {
		actual := Part2(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %d, expected %d", i, actual, c.expected)
		}
	}
}
