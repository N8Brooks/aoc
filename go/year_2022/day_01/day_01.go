package day_01

import (
	"strconv"
	"strings"
	"util"
)

func Part1(input *string) int {
	var max_calories int
	for _, calories := range strings.Split(*input, "\n\n") {
		var total_calories int
		for _, calories := range strings.Split(calories, "\n") {
			calories, _ := strconv.Atoi(calories)
			total_calories += calories
		}
		if total_calories > max_calories {
			max_calories = total_calories
		}
	}
	return max_calories
}

func Part2(input *string) int {
	h := &util.IntHeap{0, 0, 0}
	for _, calories := range strings.Split(*input, "\n\n") {
		var total_calories int
		for _, calories := range strings.Split(calories, "\n") {
			calories, _ := strconv.Atoi(calories)
			total_calories += calories
		}
		h.PushPop(total_calories)
	}
	return h.Sum()
}
