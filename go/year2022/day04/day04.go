package day04

import (
	"strconv"
	"strings"
)

func Part1(input string) int {
	return countTest(input, fullyContains)
}

func fullyContains(start1, stop1, start2, stop2 int) bool {
	return start2 <= start1 && stop1 <= stop2
}

func Part2(input string) int {
	return countTest(input, overlapAtAll)
}

func overlapAtAll(start1, _stop1, start2, stop2 int) bool {
	return start2 <= start1 && start1 <= stop2
}

type Test = func(start1, stop1, start2, stop2 int) bool

func countTest(input string, test Test) (count int) {
	for line := range strings.SplitSeq(input, "\n") {
		a, b, c, d := parseLine(line)
		if test(a, b, c, d) || test(c, d, a, b) {
			count++
		}
	}
	return count
}

func parseLine(line string) (start1, stop1, start2, stop2 int) {
	ranges := strings.Split(line, ",")
	start1, stop1 = parseRange(ranges[0])
	start2, stop2 = parseRange(ranges[1])
	return start1, stop1, start2, stop2
}

func parseRange(r string) (start, stop int) {
	range_ := strings.Split(r, "-")
	start, _ = strconv.Atoi(range_[0])
	stop, _ = strconv.Atoi(range_[1])
	return start, stop
}
