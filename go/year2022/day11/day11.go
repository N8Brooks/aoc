package day11

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/N8Brooks/aoc/go/util/collections"
)

func Part1(input string) int {
	monkeysInput := ParseMonkeyParams(input)
	makeOperate := func(abc [3]uint64) func(old uint64) uint64 {
		a, b, c := abc[0], abc[1], abc[2]
		return func(old uint64) uint64 {
			return (a*old*old + b*old + c) / 3
		}
	}
	return MonkeyBusiness(monkeysInput, makeOperate, 20)
}

func Part2(input string) int {
	monkeysInput := ParseMonkeyParams(input)
	monkeyMod := uint64(1)
	for _, monkeyInput := range monkeysInput {
		monkeyMod *= monkeyInput.test[0]
	}
	makeOperate := func(abc [3]uint64) func(old uint64) uint64 {
		a, b, c := abc[0], abc[1], abc[2]
		return func(old uint64) uint64 {
			return (a*old*old + b*old + c) % monkeyMod
		}
	}
	return MonkeyBusiness(monkeysInput, makeOperate, 10_000)
}

type MakeOperate func(abc [3]uint64) func(old uint64) uint64

func MonkeyBusiness(monkeysInput []MonkeyParams, makeOperate MakeOperate, rounds int) int {
	monkeys := make([]Monkey, len(monkeysInput))
	for i, monkeyInput := range monkeysInput {
		monkey := &monkeys[i]
		monkey.items = monkeyInput.items
		operate := makeOperate(monkeyInput.operation)
		divisible, ifTrue, ifFalse := monkeyInput.test[0], &monkeys[monkeyInput.test[1]], &monkeys[monkeyInput.test[2]]
		monkey.operate = func() {
			monkey.count += len(monkey.items)
			for _, old := range monkey.items {
				new := operate(old)
				if new%divisible == 0 {
					ifTrue.items = append(ifTrue.items, new)
				} else {
					ifFalse.items = append(ifFalse.items, new)
				}
			}
			monkey.items = monkey.items[:0]
		}
	}
	for range rounds {
		for _, monkey := range monkeys {
			monkey.operate()
		}
	}
	h := &collections.BinaryHeap{0, 0}
	for _, monkey := range monkeys {
		h.PushPop(monkey.count)
	}
	return h.Prod()
}

type Monkey struct {
	count   int
	items   []uint64
	operate func()
}

var r = regexp.MustCompile(`Monkey \d+:
  Starting items: ([^\n]*)
  Operation: new = ([^\n]*)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)`)

func ParseMonkeyParams(input string) []MonkeyParams {
	monkeyParams := []MonkeyParams{}
	for _, match := range r.FindAllStringSubmatch(input, -1) {
		items := ParseItems(match[1])
		operation := ParseOperation(match[2])
		divisibleBy, _ := strconv.Atoi(match[3])
		ifTrueMonkey, _ := strconv.Atoi(match[4])
		ifFalseMonkey, _ := strconv.Atoi(match[5])
		test := [3]uint64{uint64(divisibleBy), uint64(ifTrueMonkey), uint64(ifFalseMonkey)}
		monkeyParams = append(monkeyParams, MonkeyParams{items, operation, test})
	}
	return monkeyParams
}

func ParseItems(itemString string) []uint64 {
	startingItems := strings.Split(itemString, ", ")
	items := make([]uint64, 0, len(startingItems))
	for _, itemString := range startingItems {
		item, _ := strconv.Atoi(itemString)
		items = append(items, uint64(item))
	}
	return items
}

func ParseOperation(input string) [3]uint64 {
	if input == "old * old" {
		return [3]uint64{1, 0, 0}
	}
	input = strings.TrimPrefix(input, "old ")
	operator, rest, _ := strings.Cut(input, " ")
	other, _ := strconv.Atoi(rest)
	switch operator {
	case "*":
		return [3]uint64{0, uint64(other), 0}
	case "+":
		return [3]uint64{0, 1, uint64(other)}
	default:
		panic(fmt.Sprintf("Unknown operator %s", operator))
	}
}

type MonkeyParams struct {
	// Starting items
	items []uint64
	// Operations in terms of a, b, and c: a * old^2 + b * old + c
	operation [3]uint64
	// Divisible by, if true monkey, if false monkey
	test [3]uint64
}
