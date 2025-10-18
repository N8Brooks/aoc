package day17

import (
	"fmt"
	"slices"
	"util/imath"
)

func Part1(input string) int {
	return SolveFor(input, 2022)
}

func Part2(input string) int {
	return SolveFor(input, 1_000_000_000_000)
}

func SolveFor(input string, n int) int {
	cache := map[imath.Pos]imath.Pos{}
	chamber := Chamber{}
	dirGetter := DirGetter(input)
	j := 0
	for i := range n {

		key := imath.Pos{i % len(RockShapes), j % len(input)}
		if entry, ok := cache[key]; ok {
			dividend := n - i
			divisor := i - entry[0]
			quotient := dividend / divisor
			remainder := dividend % divisor
			if remainder == 0 {
				return len(chamber) + (len(chamber)-entry[1])*quotient
			}
		} else {
			cache[key] = imath.Pos{i, len(chamber)}
		}

		rock := CreateRock(i, len(chamber))

		for ; !chamber.IsRestBottom(rock); j++ {
			rock.FallDown()
			dir := dirGetter(j)
			rock.Push(dir, chamber)
		}

		chamber = chamber.Update(rock)
	}

	return len(chamber)
}

func DirGetter(input string) func(j int) int {
	return func(j int) int {
		byte := input[j%len(input)]
		switch byte {
		case '<':
			return -1
		case '>':
			return 1
		default:
			panic(fmt.Sprintf("Unknown byte %s", string(byte)))
		}
	}
}

type Rock []imath.Pos

func CreateRock(rockIndex, height int) Rock {
	rockShape := RockShapes[rockIndex%len(RockShapes)]
	rock := make([]imath.Pos, len(rockShape))
	copy(rock, rockShape)
	for i := range rock {
		rock[i][0] = height + 4 - rock[i][0]
		rock[i][1] += 2
	}
	return rock
}

func (rock Rock) FallDown() {
	for i := range rock {
		rock[i][0] -= 1
	}
}

func (rock Rock) Push(dir int, chamber Chamber) {
	oobLeft := dir == -1 && chamber.IsRestLeft(rock)
	oobRight := dir == 1 && chamber.IsRestRight(rock)
	if !oobLeft && !oobRight {
		for i := range rock {
			rock[i][1] += dir
		}
	}
}

type Chamber [][7]bool

func (chamber Chamber) IsRestBottom(rock Rock) bool {
	return slices.ContainsFunc(rock, func(pos imath.Pos) bool {
		return pos[0] == 0 || len(chamber) >= pos[0] && chamber[pos[0]-1][pos[1]]
	})
}

func (chamber Chamber) IsRestLeft(rock Rock) bool {
	return slices.ContainsFunc(rock, func(pos imath.Pos) bool {
		return pos[1] == 0 || len(chamber) > pos[0] && chamber[pos[0]][pos[1]-1]
	})
}

func (chamber Chamber) IsRestRight(rock Rock) bool {
	return slices.ContainsFunc(rock, func(pos imath.Pos) bool {
		return pos[1] == 6 || len(chamber) > pos[0] && chamber[pos[0]][pos[1]+1]
	})
}

func (chamber Chamber) Update(rock Rock) Chamber {
	chamber = chamber.expandToFit(rock)
	chamber.insert(rock)
	return chamber
}

func (chamber Chamber) expandToFit(rock Rock) Chamber {
	n := slices.MaxFunc(rock, func(a, b imath.Pos) int { return a[0] - b[0] })[0] + 1
	update := n - len(chamber)
	if update > 0 {
		chamber = append(chamber, make([][7]bool, update)...)
	}
	return chamber
}

func (chamber Chamber) insert(rock Rock) {
	for _, pos := range rock {
		chamber[pos[0]][pos[1]] = true
	}
}

var RockShapes = [5]Rock{
	{{0, 0}, {0, 1}, {0, 2}, {0, 3}},
	{{-2, 1}, {-1, 0}, {-1, 1}, {-1, 2}, {0, 1}},
	{{-2, 2}, {-1, 2}, {0, 0}, {0, 1}, {0, 2}},
	{{0, 0}, {-1, 0}, {-2, 0}, {-3, 0}},
	{{-1, 0}, {-1, 1}, {0, 0}, {0, 1}},
}
