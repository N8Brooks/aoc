package year_2022

import (
	"testing"
	"util"
)

const Example1 = `R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`

const Example2 = `R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20`

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example1, 13},
		{util.Input(2022, 9), 6190},
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
		{Example1, 1},
		{Example2, 36},
		{util.Input(2022, 9), 2516},
	}
	for i, c := range cases {
		actual := Part2(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %d, expected %d", i, actual, c.expected)
		}
	}
}
