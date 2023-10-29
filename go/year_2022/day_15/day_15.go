package year_2022

import (
	"regexp"
	"strconv"
	"util/imath"
)

func Part1(input string) int {
	return CountExisting(input, 2_000_000)
}

func CountExisting(input string, y int) int {
	pairs := ParseInput(input)
	existing := map[int]bool{}
	for _, pair := range pairs {
		minX, maxX := pair.GetXBounds(y)
		for j := minX; j <= maxX; j++ {
			existing[j] = true
		}
	}
	for _, pair := range pairs {
		if pair.Beacon[1] == y {
			delete(existing, pair.Beacon[1])
		}
	}
	return len(existing)
}

func Part2(input string) int {
	return FindTuningFrequency(input, 4_000_000)
}

func FindTuningFrequency(input string, maxBound int) int {
	pairs := ParseInput(input)

	pos := map[int]bool{}
	neg := map[int]bool{}
	for _, pair := range pairs {
		x, y := pair.Sensor.Values()
		r := pair.Radius() + 1
		pos[y-x+r] = true
		pos[y-x-r] = true
		neg[x+y+r] = true
		neg[x+y-r] = true
	}

	for a := range pos {
	Loop:
		for b := range neg {
			x := (b - a) / 2
			y := (a + b) / 2
			if x <= 0 || x >= maxBound || y <= 0 || y >= maxBound {
				continue
			}

			for _, pair := range pairs {
				if imath.ManhattenDistance(pair.Sensor, imath.Pos{x, y}) <= pair.Radius() {
					continue Loop
				}
			}

			return 4_000_000*x + y
		}
	}

	return -1
}

type Pair struct{ Sensor, Beacon imath.Pos }

func (pair Pair) Radius() int {
	return imath.ManhattenDistance(pair.Sensor, pair.Beacon)
}

var R = regexp.MustCompile(`Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)`)

func ParseInput(input string) []Pair {
	result := []Pair{}
	for _, match := range R.FindAllStringSubmatch(input, -1) {
		sensorX, _ := strconv.Atoi(match[1])
		sensorY, _ := strconv.Atoi(match[2])
		sensor := imath.Pos{sensorX, sensorY}
		beaconX, _ := strconv.Atoi(match[3])
		beaconY, _ := strconv.Atoi(match[4])
		beacon := imath.Pos{beaconX, beaconY}
		result = append(result, Pair{sensor, beacon})
	}
	return result
}

// Inclusive min and max x overlap at the given y value
func (pair Pair) GetXBounds(y int) (int, int) {
	dist := pair.Radius()
	diff := imath.Abs(y - pair.Sensor[1])
	radius := dist - diff
	minX := pair.Sensor[0] - radius
	maxX := pair.Sensor[0] + radius
	return minX, maxX
}
