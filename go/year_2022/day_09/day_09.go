package year_2022

import (
	"strconv"
	"strings"
	"util"
)

func Part1(input string) int {
	return CountTailPositions(input, 2)
}

func Part2(input string) int {
	return CountTailPositions(input, 10)
}

func CountTailPositions(input string, length int) int {
	bridge := NewBridge(length)
	for _, motion := range strings.Split(input, "\n") {
		bridge.Move(motion)
	}
	return len(bridge.tailPositions)
}

type Bridge struct {
	knots         []*Knot
	tailPositions map[Knot]struct{}
}

func NewBridge(length int) Bridge {
	knots := make([]*Knot, 0, length)
	for i := 0; i < length; i++ {
		knots = append(knots, &Knot{})
	}
	tailPositions := map[Knot]struct{}{}
	return Bridge{knots, tailPositions}
}

func (bridge *Bridge) Move(motion string) {
	di, dj, steps := ParseVelocity(motion)
	for step := 0; step < steps; step++ {
		leader := bridge.knots[0]
		leader.Move(di, dj)
		for _, knot := range bridge.knots[1:] {
			knot.Follow(leader)
			leader = knot
		}
		tail := *leader
		bridge.tailPositions[tail] = struct{}{}
	}
}

func ParseVelocity(motion string) (di, dj, steps int) {
	dir, stepsString, _ := strings.Cut(motion, " ")
	steps, _ = strconv.Atoi(stepsString)
	switch dir {
	case "L":
		dj = -1
	case "R":
		dj = 1
	case "U":
		di = -1
	case "D":
		di = 1
	}
	return
}

type Knot struct{ i, j int }

func (knot *Knot) Move(di, dj int) {
	knot.i += di
	knot.j += dj
}

func (knot *Knot) Follow(leader *Knot) {
	di := leader.i - knot.i
	dj := leader.j - knot.j
	if util.Abs(di)+util.Abs(dj) > 2 {
		knot.i += util.Sign(di)
		knot.j += util.Sign(dj)
	} else if di == 0 && util.Abs(dj) == 2 {
		knot.j += dj / 2
	} else if util.Abs(di) == 2 && dj == 0 {
		knot.i += di / 2
	}
}
