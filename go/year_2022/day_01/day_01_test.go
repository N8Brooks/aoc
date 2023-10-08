package day_01

import (
	_ "embed"
	"io/ioutil"
	"testing"
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

func input() string {
	b, err := ioutil.ReadFile("../../../testdata/year_2022/day_01.txt")
	if err != nil {
		panic(err)
	}
	return string(b)
}

func TestPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{Example, 24000},
		{input(), 68802},
	}
	for i, c := range cases {
		actual := Part1(&c.input)
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
		{input(), 205370},
	}
	for i, c := range cases {
		actual := Part2(&c.input)
		if actual != c.expected {
			t.Errorf("Part1 %d == %d, expected %d", i, actual, c.expected)
		}
	}
}
