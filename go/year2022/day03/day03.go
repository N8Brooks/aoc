package day03

import (
	"fmt"
	"strings"
)

func Part1(input string) int {
	var sumPriority int
	for line := range strings.SplitSeq(input, "\n") {
		sumPriority += GetCommonItem1(line).Priority()
	}
	return sumPriority
}

// GetCommonItem1 finds the common item between the first and second compartments
// Assumes the length is divisible by two (rounds down) and ascii input
// Returns the first common byte, panicking if there are none
func GetCommonItem1(rucksack string) Item {
	length := len(rucksack)
	pivot := length / 2
	elements := map[byte]bool{}
	for i := range pivot {
		byte := rucksack[i]
		elements[byte] = true
	}
	for i := pivot; i < length; i++ {
		byte := rucksack[i]
		if elements[byte] {
			return Item(byte)
		}
	}
	panic("No common bytes")
}

func Part2(input string) int {
	rucksacks := strings.Split(input, "\n")
	var sumPriority int
	for i := 0; i < len(rucksacks); i += 3 {
		item := GetCommonItem2(rucksacks[i : i+3])
		sumPriority += item.Priority()
	}
	return sumPriority
}

// GetCommonItem2 finds the common item across all `rucksacks`
// Returns the first common on the last line if there are multiple
// Panics if there are none
func GetCommonItem2(rucksacks []string) Item {
	next_index := map[rune]int{}
	for i, rucksack := range rucksacks {
		for _, rune := range rucksack {
			last_seen := next_index[rune]
			if i == last_seen {
				if last_seen == 2 {
					return Item(rune)
				}
				next_index[rune] += 1
			}
		}
	}
	panic("No common runes")
}

type Item rune

func (item Item) Priority() int {
	switch {
	case 'a' <= item && item <= 'z':
		return int(item - 'a' + 1)
	case 'A' <= item && item <= 'Z':
		return int(item - 'A' + 27)
	default:
		panic(fmt.Sprintf("Unknown char %c", item))
	}
}
