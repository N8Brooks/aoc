package day05

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/N8Brooks/aoc/go/util"
)

func Part1(input string) string {
	stacks, rearrangements := parseInput(input)
	for _, r := range rearrangements {
		j := len(stacks[r.from]) - r.move
		crates := stacks[r.from][j:]
		stacks[r.from] = stacks[r.from][:j]
		util.Reverse(crates)
		stacks[r.to] = append(stacks[r.to], crates...)
	}
	return TopCratesOfStacks(stacks)
}

func Part2(input string) string {
	stacks, rearrangements := parseInput(input)
	for _, r := range rearrangements {
		j := len(stacks[r.from]) - r.move
		crates := stacks[r.from][j:]
		stacks[r.from] = stacks[r.from][:j]
		stacks[r.to] = append(stacks[r.to], crates...)
	}
	return TopCratesOfStacks(stacks)
}

type Rearrangement struct{ move, from, to int }

func parseInput(input string) ([][]byte, []Rearrangement) {
	inputs := strings.Split(input, "\n\n")
	stacks := parseStacks(inputs[0])
	rearrangements := parseRearrangments(inputs[1])
	return stacks, rearrangements
}

// Generates stacks from every 4th byte on each line, starting at index 1
func parseStacks(input string) [][]byte {
	lines := strings.Split(input, "\n")
	lines = lines[:len(lines)-1]
	n := (len(lines[0]) + 1) / 4
	stacks := make([][]byte, n)
	for _, line := range lines {
		for j := 1; j < len(line); j += 4 {
			if line[j] == ' ' {
				continue
			}
			crate := []byte{line[j]}
			column := j / 4
			stacks[column] = append(crate, stacks[column]...)
		}
	}
	return stacks
}

var r = regexp.MustCompile(`move (\d+) from (\d+) to (\d+)`)

func parseRearrangments(input string) []Rearrangement {
	matches := r.FindAllStringSubmatch(input, -1)
	rearrangements := make([]Rearrangement, 0, len(matches))
	for _, match := range matches {
		move, _ := strconv.Atoi(match[1])
		from, _ := strconv.Atoi(match[2])
		to, _ := strconv.Atoi(match[3])
		rearrangement := Rearrangement{move, from - 1, to - 1}
		rearrangements = append(rearrangements, rearrangement)
	}
	return rearrangements
}

func (r Rearrangement) String() string {
	return fmt.Sprintf("move %d from %d to %d", r.move, r.from, r.to)
}

func TopCratesOfStacks(stacks [][]byte) string {
	result := make([]string, 0, len(stacks))
	for _, stack := range stacks {
		crate := string(stack[len(stack)-1])
		result = append(result, crate)
	}
	return strings.Join(result, "")
}
