package day12

import (
	"strings"
)

type Pos struct{ i, j int }

// Part1 breadth first search
//   - Begins at the starting position
//   - Returns steps after reaching the ending position
//   - Marks visited positions with `255`
//   - Travels to positions with heights lte `curHeight + 1`
func Part1(input string) int {
	heightMap := ParseHeightMap(input)
	startPos, endPos := IndexPositions(heightMap)
	maxPos := Pos{len(heightMap), len(heightMap[0])}
	curFrontier := map[Pos]struct{}{startPos: {}}
	for steps := 0; len(curFrontier) > 0; steps++ {
		nxtFrontier := make(map[Pos]struct{})
		for curPos := range curFrontier {
			if curPos == endPos {
				return steps
			}
			curHeight := heightMap[curPos.i][curPos.j]
			heightMap[curPos.i][curPos.j] = 255
			for _, nxtPos := range Neighbors(curPos, maxPos) {
				nxtHeight := heightMap[nxtPos.i][nxtPos.j]
				if nxtHeight > curHeight+1 {
					continue
				}
				nxtFrontier[nxtPos] = struct{}{}
			}
		}
		curFrontier = nxtFrontier
	}
	panic("No ending position")
}

// Part2 breadth first search
//   - Begins at the ending position
//   - Returns steps after reaching a height of `'a'`
//   - Marks visited positions with `0`
//   - Travels to positions with heights gte `curHeight - 1`
func Part2(input string) int {
	heightMap := ParseHeightMap(input)
	_, startPos := IndexPositions(heightMap)
	maxPos := Pos{len(heightMap), len(heightMap[0])}
	curFrontier := map[Pos]struct{}{startPos: {}}
	for steps := 0; len(curFrontier) > 0; steps++ {
		nxtFrontier := make(map[Pos]struct{})
		for curPos := range curFrontier {
			curHeight := heightMap[curPos.i][curPos.j]
			if curHeight == 'a' {
				return steps
			}
			heightMap[curPos.i][curPos.j] = 0
			for _, nxtPos := range Neighbors(curPos, maxPos) {
				nxtHeight := heightMap[nxtPos.i][nxtPos.j]
				if nxtHeight < curHeight-1 {
					continue
				}
				nxtFrontier[nxtPos] = struct{}{}
			}
		}
		curFrontier = nxtFrontier
	}
	panic("No ending position")
}

// ParseHeightMap creates a mutable height map from the input
func ParseHeightMap(input string) [][]byte {
	rows := strings.Split(input, "\n")
	heightMap := make([][]byte, 0, len(rows))
	for _, row := range rows {
		heightMap = append(heightMap, []byte(row))
	}
	return heightMap
}

// IndexPositions returns the starting and ending positions, replacing them with their heights
func IndexPositions(heightMap [][]byte) (start, end Pos) {
	for i, row := range heightMap {
		for j, height := range row {
			switch height {
			case 'S':
				heightMap[i][j] = 'a'
				start = Pos{i, j}
			case 'E':
				heightMap[i][j] = 'z'
				end = Pos{i, j}
			}
		}
	}
	return start, end
}

// Velocities contains the possible changes in position
var Velocities = [4]struct{ di, dj int }{
	{-1, 0},
	{0, -1},
	{+1, 0},
	{0, +1},
}

func Neighbors(cur, max Pos) []Pos {
	neighbors := make([]Pos, 0)
	for _, vel := range Velocities {
		i := cur.i + vel.di
		if i < 0 || i >= max.i {
			continue
		}
		j := cur.j + vel.dj
		if j < 0 || j >= max.j {
			continue
		}
		neighbors = append(neighbors, Pos{i, j})
	}
	return neighbors
}
