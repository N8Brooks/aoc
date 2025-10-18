package day06

import (
	"testing"

	"github.com/N8Brooks/aoc/go/util"
)

const (
	Example1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
	Example2 = "bvwbjplbgvbhsrlpgdmjqwftvncz"
	Example3 = "nppdvjthqldpwncqszvftbrmjlhg"
	Example4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
	Example5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
)

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example1, 7},
		{Example2, 5},
		{Example3, 6},
		{Example4, 10},
		{Example5, 11},
		{util.Input(2022, 6), 1929},
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
		{Example1, 19},
		{Example2, 23},
		{Example3, 23},
		{Example4, 29},
		{Example5, 26},
		{util.Input(2022, 6), 3298},
	}
	for i, c := range cases {
		actual := Part2(c.input)
		if actual != c.expected {
			t.Errorf("%d: actual %d, expected %d", i, actual, c.expected)
		}
	}
}
