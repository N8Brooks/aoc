package day10

import (
	"strconv"
	"strings"
)

func Part1(input string) int {
	signals := ComputeSignals(input)
	sumSignalStrength := 0
	for cycle := 20; cycle <= 220; cycle += 40 {
		signal := signals[cycle]
		signalStrength := signal * cycle
		sumSignalStrength += signalStrength
	}
	return sumSignalStrength
}

func Part2(input string) string {
	chars := []string{}
	for _, letter := range DrawLetters(input) {
		char := Letters[letter]
		chars = append(chars, char)
	}
	return strings.Join(chars, "")
}

var Letters = map[string]string{
	`###..
#..#.
#..#.
###..
#.#..
#..#.`: "R",
	`#..#.
#.#..
##...
#.#..
#.#..
#..#.`: "K",
	`###..
#..#.
#..#.
###..
#....
#....`: "P",
	`..##.
...#.
...#.
...#.
#..#.
.##..`: "J",
	`###..
#..#.
###..
#..#.
#..#.
###..`: "B",
	`#....
#....
#....
#....
#....
####.`: "L",
	`.##..
#..#.
#..#.
####.
#..#.
#..#.`: "A",
}

func DrawLetters(input string) []string {
	screen := ComputeScreen(input)
	letters := make([]string, 0, 8)
	for j := 0; j < 40; j += 5 {
		rows := make([]string, 0, 6)
		for i := range 6 {
			row := strings.Join(screen[i][j:j+5], "")
			rows = append(rows, row)
		}
		letter := strings.Join(rows, "\n")
		letters = append(letters, letter)
	}
	return letters
}

func DrawImage(input string) string {
	screen := ComputeScreen(input)
	rows := make([]string, 0, 6)
	for _, row := range screen {
		rows = append(rows, strings.Join(row, ""))
	}
	return strings.Join(rows, "\n")
}

func ComputeScreen(input string) [][]string {
	signals := ComputeSignals(input)[1:]
	screen := make([][]string, 0, 6)
	for range 6 {
		row := make([]string, 0, 40)
		for j := range 40 {
			signal := signals[0]
			signals = signals[1:]
			if signal-1 <= j && j <= signal+1 {
				row = append(row, "#")
			} else {
				row = append(row, ".")
			}
		}
		screen = append(screen, row)
	}
	return screen
}

// ComputeSignals returns an array with each signal for each cycle
func ComputeSignals(input string) []int {
	x := 1
	cycles := []int{x}
	for line := range strings.SplitSeq(input, "\n") {
		instruction, rest, _ := strings.Cut(line, " ")
		cycles = append(cycles, x)
		if instruction == "addx" {
			arg, _ := strconv.Atoi(rest)
			cycles = append(cycles, x)
			x += arg
		}
	}
	return cycles
}
