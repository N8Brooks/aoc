package day14

import (
	"strconv"
	"strings"
	"util/imath"
)

type Pos = struct{ x, y int }

func Part1(input string) int {
	blocks, maxY := ParseInput(input)
	numRocks := len(blocks)
	stack := []Pos{{500, 0}}
	for len(stack) > 0 {
		pos := stack[len(stack)-1]
		if pos.y >= maxY {
			break
		} else if !blocks[Pos{pos.x, pos.y + 1}] {
			stack = append(stack, Pos{pos.x, pos.y + 1})
		} else if !blocks[Pos{pos.x - 1, pos.y + 1}] {
			stack = append(stack, Pos{pos.x - 1, pos.y + 1})
		} else if !blocks[Pos{pos.x + 1, pos.y + 1}] {
			stack = append(stack, Pos{pos.x + 1, pos.y + 1})
		} else {
			blocks[pos] = true
			stack = stack[:len(stack)-1]
		}
	}

	return len(blocks) - numRocks
}

func Part2(input string) int {
	blocks, maxY := ParseInput(input)
	maxY += 2
	numRocks := len(blocks)
	stack := []Pos{{500, 0}}
	for len(stack) > 0 {
		pos := stack[len(stack)-1]
		if pos.y+1 >= maxY {
			blocks[pos] = true
			stack = stack[:len(stack)-1]
		} else if !blocks[Pos{pos.x, pos.y + 1}] {
			stack = append(stack, Pos{pos.x, pos.y + 1})
		} else if !blocks[Pos{pos.x - 1, pos.y + 1}] {
			stack = append(stack, Pos{pos.x - 1, pos.y + 1})
		} else if !blocks[Pos{pos.x + 1, pos.y + 1}] {
			stack = append(stack, Pos{pos.x + 1, pos.y + 1})
		} else {
			blocks[pos] = true
			stack = stack[:len(stack)-1]
		}
	}

	return len(blocks) - numRocks
}

func ParseInput(input string) (map[Pos]bool, int) {
	rockStructures := ParseRockStructures(input)
	blocks := map[Pos]bool{}
	for _, positions := range rockStructures {
		curPos := positions[0]
		for _, endPos := range positions {
			dx := imath.Sign(endPos.x - curPos.x)
			dy := imath.Sign(endPos.y - curPos.y)
			blocks[curPos] = true
			for curPos != endPos {
				curPos.x += dx
				curPos.y += dy
				blocks[curPos] = true
			}
		}
	}
	return blocks, MaxY(rockStructures)
}

func ParseRockStructures(input string) [][]Pos {
	rockStructures := [][]Pos{}
	for line := range strings.SplitSeq(input, "\n") {
		rockStructure := ParseRockStructure(line)
		rockStructures = append(rockStructures, rockStructure)
	}
	return rockStructures
}

func ParseRockStructure(line string) []Pos {
	positions := []Pos{}
	for positionInput := range strings.SplitSeq(line, " -> ") {
		xInput, yInput, _ := strings.Cut(positionInput, ",")
		x, _ := strconv.Atoi(xInput)
		y, _ := strconv.Atoi(yInput)
		position := Pos{x, y}
		positions = append(positions, position)
	}
	return positions
}

func MaxY(rockStructures [][]Pos) int {
	maxY := 0
	for _, positions := range rockStructures {
		for _, pos := range positions {
			if pos.y > maxY {
				maxY = pos.y
			}
		}
	}
	return maxY
}
