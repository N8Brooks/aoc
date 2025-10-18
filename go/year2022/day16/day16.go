package day16

import (
	"regexp"
	"strconv"
	"strings"
)

func Part1(input string) int {
	root, rates, tunnels := ParseInput(input)
	visit := Tsp(root, rates, tunnels)
	return visit(root, 0, 30)
}

func Part2(input string) int {
	root, rates, tunnels := ParseInput(input)
	visit := Tsp(root, rates, tunnels)

	mask := 0
	for i, rate := range rates {
		if rate > 0 {
			mask |= (1 << i)
		}
	}

	maxPressure := 0
	for subset1 := mask; subset1 > 0; subset1 = (subset1 - 1) & mask {
		subset2 := mask & ^subset1
		pressure1 := visit(root, subset1, 26)
		pressure2 := visit(root, subset2, 26)
		pressure := pressure1 + pressure2
		if pressure > maxPressure {
			maxPressure = pressure
		}
	}

	return maxPressure
}

func Tsp(root int, rates []int, tunnels [][]int) func(u, visited, t int) int {
	dp := map[[3]int]int{}

	var visit func(u, visited, t int) int

	visit = func(u, visited, t int) int {
		if t <= 0 {
			return 0
		}

		key := [3]int{u, visited, t}
		if res, ok := dp[key]; ok {
			return res
		}

		maxPressure := 0

		// Open valve
		if rates[u] > 0 && visited&(1<<u) == 0 {
			openPressure := rates[u] * (t - 1)
			pressure := openPressure + visit(u, visited|(1<<u), t-1)
			if pressure > maxPressure {
				maxPressure = pressure
			}
		}

		// Visit neighboring valve
		for _, v := range tunnels[u] {
			pressure := visit(v, visited, t-1)
			if pressure > maxPressure {
				maxPressure = pressure
			}
		}

		dp[key] = maxPressure

		return maxPressure
	}

	return visit
}

var r = regexp.MustCompile(`Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([^\n]+)`)

func ParseInput(input string) (int, []int, [][]int) {
	matches := r.FindAllStringSubmatch(input, -1)
	valveNames := ParseValveNames(matches)
	rates := make([]int, len(matches))
	tunnels := make([][]int, len(matches))
	for _, match := range matches {
		index := valveNames[match[1]]
		rate, _ := strconv.Atoi(match[2])
		rates[index] = rate
		for tunnel := range strings.SplitSeq(match[3], ", ") {
			tunnels[index] = append(tunnels[index], valveNames[tunnel])
		}
	}
	return valveNames["AA"], rates, tunnels
}

func ParseValveNames(matches [][]string) map[string]int {
	valveNames := map[string]int{}
	for _, match := range matches {
		valve := match[1]
		valveNames[valve] = len(valveNames)
	}
	return valveNames
}
