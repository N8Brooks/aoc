package day02

import (
	"fmt"
	"strings"
)

func Part1(input string) int {
	totalScore := 0
	for line := range strings.SplitSeq(input, "\n") {
		chars := strings.Split(line, " ")
		opponent := parseShape(chars[0])
		player := parseShape(chars[1])
		outcome := player.playAgainst(opponent)
		score := player.Points() + outcome.Points()
		totalScore += score
	}
	return totalScore
}

func Part2(input string) int {
	totalScore := 0
	for line := range strings.SplitSeq(input, "\n") {
		chars := strings.Split(line, " ")
		opponent := parseShape(chars[0])
		outcome := parseOutcome(chars[1])
		player := opponent.endWith(outcome)
		score := player.Points() + outcome.Points()
		totalScore += score
	}
	return totalScore
}

type Shape int

const (
	Rock Shape = iota
	Paper
	Scissors
)

func parseShape(char string) Shape {
	switch char {
	case "A", "X":
		return Rock
	case "B", "Y":
		return Paper
	case "C", "Z":
		return Scissors
	default:
		panic(fmt.Sprintf("Unknown char %s", char))
	}
}

func (shape Shape) playAgainst(opponent Shape) Outcome {
	return Outcome((shape - opponent + 1 + 3) % 3)
}

func (shape Shape) endWith(outcome Outcome) Shape {
	return Shape((int(outcome) + int(shape) - 1 + 3) % 3)
}

func (shape Shape) Points() int {
	return int(shape) + 1
}

func (shape Shape) String() string {
	switch shape {
	case Rock:
		return "Rock"
	case Paper:
		return "Paper"
	case Scissors:
		return "Scissors"
	default:
		panic(fmt.Sprintf("Unknown Shape %d", shape))
	}
}

type Outcome int

const (
	Loose Outcome = iota
	Draw
	Win
)

func parseOutcome(char string) Outcome {
	switch char {
	case "X":
		return Loose
	case "Y":
		return Draw
	case "Z":
		return Win
	default:
		panic(fmt.Sprintf("Unknown char %s", char))
	}
}

func (outcome Outcome) Points() int {
	return int(outcome) * 3
}

func (outcome Outcome) String() string {
	switch outcome {
	case Win:
		return "Win"
	case Draw:
		return "Draw"
	case Loose:
		return "Loose"
	default:
		panic(fmt.Sprintf("Unknown Outcome %d", outcome))
	}
}
