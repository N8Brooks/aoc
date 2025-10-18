package util

import (
	"fmt"
	"os"
)

const testdata = "../../../test_data"

var cache = map[problem]string{}

// Input returns the input for the problem
// Caches results in memory
// Panics if it does not exist on disk
// Not set up for concurrent access
func Input(year uint16, day uint8) string {
	p := problem{year, day}
	if input, exists := cache[p]; exists {
		return input
	}
	input := p.readInput()
	cache[p] = input
	return input
}

// The year and day of a problem
type problem struct {
	Year uint16
	Day  uint8
}

// The input for the problem, panics if it does not exist
func (p problem) readInput() string {
	b, err := os.ReadFile(p.inputName())
	if err != nil {
		panic(err)
	}
	return string(b)
}

// The file name for the problem input
func (p problem) inputName() string {
	return fmt.Sprintf("%s/year_%04d/day_%02d.txt", testdata, p.Year, p.Day)
}
