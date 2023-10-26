package year_2022

import (
	"encoding/json"
	"slices"
	"strings"
)

// A JSON float64 or slice of Values, functions will panic if this is not true.
type Value = interface {
}

// A list of Values - A packet is a List
type List = []Value

func Part1(input string) int {
	sum := 0
	for i, pair := range ParsePairs(input) {
		if slices.CompareFunc(pair[0], pair[1], CompareValue) <= 0 {
			sum += i + 1
		}
	}
	return sum
}

func Part2(input string) int {
	pairs := ParsePairs(input)
	var delta2, delta6 int
	for _, pair := range pairs {
		for _, packet := range pair {
			if CompareValue(packet, List{2.}) <= 0 {
				delta2++
			} else if CompareValue(packet, List{6.}) <= 0 {
				delta6++
			}
		}
	}
	index2 := delta2 + 1
	index6 := index2 + delta6 + 1
	return index2 * index6
}

func ParsePairs(input string) [][2]List {
	packetPairs := make([][2]List, 0)
	for _, pairs := range strings.Split(input, "\n\n") {
		leftData, rightData, _ := strings.Cut(pairs, "\n")
		leftPacket := parsePacketData(leftData)
		rightPacket := parsePacketData(rightData)
		packetPair := [2]List{leftPacket, rightPacket}
		packetPairs = append(packetPairs, packetPair)
	}
	return packetPairs
}

func parsePacketData(data string) List {
	var packet List
	json.Unmarshal([]byte(data), &packet)
	return packet
}

func CompareValue(leftValue, rightValue Value) int {
	leftNum, leftIsNum := leftValue.(float64)
	rightNum, rightIsNum := rightValue.(float64)
	switch [2]bool{leftIsNum, rightIsNum} {
	case [2]bool{false, false}:
		return slices.CompareFunc(leftValue.(List), rightValue.(List), CompareValue)
	case [2]bool{false, true}:
		return slices.CompareFunc(leftValue.(List), List{rightNum}, CompareValue)
	case [2]bool{true, false}:
		return slices.CompareFunc(List{leftNum}, rightValue.(List), CompareValue)
	case [2]bool{true, true}:
		return int(leftNum) - int(rightNum)
	default:
		panic("Unreachable")
	}
}
