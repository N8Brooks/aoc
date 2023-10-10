package year_2022

import (
	"testing"
	"util"
)

const Example = `    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected string
	}{
		{Example, "CMZ"},
		{util.Input(2022, 5), "TDCHVHJTG"},
	}
	for i, c := range cases {
		actual := Part1(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %s, expected %s", i, actual, c.expected)
		}
	}
}

func TestPart2(t *testing.T) {
	cases := []struct {
		input    string
		expected string
	}{
		{Example, "MCD"},
		{util.Input(2022, 5), "NGCMPJLHV"},
	}
	for i, c := range cases {
		actual := Part2(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %s, expected %s", i, actual, c.expected)
		}
	}
}
