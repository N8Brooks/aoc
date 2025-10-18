package input

import (
	"fmt"
	"os"
)

const testdata = "../../../test_data"

var cache = map[problem]string{}

// Input returns the input for the problem, caching contents in memory.
// Not safe for concurrent use. Panics if the file does not exist.
func Input(year uint16, day uint8) string {
	p := problem{year, day}
	if input, exists := cache[p]; exists {
		return input
	}
	input := p.readInput()
	cache[p] = input
	return input
}

type problem struct {
	Year uint16
	Day  uint8
}

func (p problem) readInput() string {
	b, err := os.ReadFile(p.inputName())
	if err != nil {
		panic(err)
	}
	return string(b)
}

func (p problem) inputName() string {
	return fmt.Sprintf("%s/year_%04d/day_%02d.txt", testdata, p.Year, p.Day)
}
