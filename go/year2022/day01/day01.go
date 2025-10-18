package day01

import (
	"strconv"
	"strings"
	"util"
)

func Part1(input string) int {
	var maxCalories int
	for calories := range strings.SplitSeq(input, "\n\n") {
		var totalCalories int
		for calories := range strings.SplitSeq(calories, "\n") {
			calories, _ := strconv.Atoi(calories)
			totalCalories += calories
		}
		if totalCalories > maxCalories {
			maxCalories = totalCalories
		}
	}
	return maxCalories
}

func Part2(input string) int {
	h := &util.IntHeap{0, 0, 0}
	for calories := range strings.SplitSeq(input, "\n\n") {
		var totalCalories int
		for calories := range strings.SplitSeq(calories, "\n") {
			calories, _ := strconv.Atoi(calories)
			totalCalories += calories
		}
		h.PushPop(totalCalories)
	}
	return h.Sum()
}
