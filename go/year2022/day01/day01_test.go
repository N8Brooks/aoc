package day01

import (
	_ "embed"
	"testing"

	"github.com/N8Brooks/aoc/go/util/input"
)

const Example = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example, 24000},
		{input.Load(2022, 1), 68802},
	}
	for i, c := range cases {
		actual := Part1(c.input)
		if actual != c.expected {
			t.Errorf("Part1 %d == %d, expected %d", i, actual, c.expected)
		}
	}
}

func TestPart2(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example, 45000},
		{input.Load(2022, 1), 205370},
	}
	for i, c := range cases {
		actual := Part2(c.input)
		if actual != c.expected {
			t.Errorf("Part1 %d == %d, expected %d", i, actual, c.expected)
		}
	}
}
