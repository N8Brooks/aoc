package day07

import (
	"fmt"
	"strconv"
	"strings"
	"util/itertools"
)

func Part1(input string) int {
	program := parseProgram(input)
	maxSignal := 0
	for phases := range itertools.Permutations([]int{0, 1, 2, 3, 4}, 5) {
		input := make(chan int)
		defer close(input)
		signal := 0
		for _, phase := range phases {
			output := intcode(program, input)
			input <- phase
			input <- signal
			signal = <-output
		}
		maxSignal = max(maxSignal, signal)
	}
	return maxSignal
}

func Part2(input string) int {
	program := parseProgram(input)
	maxSignal := 0
	for phases := range itertools.Permutations([]int{5, 6, 7, 8, 9}, 5) {
		input1 := make(chan int)
		outputN := input1
		for _, phase := range phases {
			inputN := outputN
			outputN = intcode(program, inputN)
			inputN <- phase
		}

		signal := 0
	loop:
		for {
			select {
			case input1 <- signal:
				signal = <-outputN
			default:
				break loop
			}
		}
		maxSignal = max(maxSignal, signal)
		close(input1)
	}
	return maxSignal
}

func parseProgram(input string) []int {
	program := []int{}
	for word := range strings.SplitSeq(input, ",") {
		word, err := strconv.Atoi(word)
		if err != nil {
			panic(err)
		}
		program = append(program, word)
	}
	return program
}

func intcode(program []int, inputs chan int) chan int {
	memory := make([]int, len(program))
	copy(memory, program)
	outputs := make(chan int)

	ip := 0

	next := func() int {
		param := memory[ip]
		ip++
		return param
	}

	read := func(mode int) int {
		param := next()
		if mode == 0 {
			return memory[param]
		}
		return param
	}

	write := func(value int) {
		i := next()
		memory[i] = value
	}

	go func() {
		defer close(outputs)
		for {
			instr := next()
			modes, opcode := instr/100, instr%100
			mode1, mode2 := modes%10, modes/10

			switch opcode {
			case 1:
				write(read(mode1) + read(mode2))
			case 2:
				write(read(mode1) * read(mode2))
			case 3:
				if input, ok := <-inputs; !ok {
					return
				} else {
					write(input)
				}
			case 4:
				outputs <- read(mode1)
			case 5:
				param1, param2 := read(mode1), read(mode2)
				if param1 != 0 {
					ip = param2
				}
			case 6:
				param1, param2 := read(mode1), read(mode2)
				if param1 == 0 {
					ip = param2
				}
			case 7:
				if read(mode1) < read(mode2) {
					write(1)
				} else {
					write(0)
				}
			case 8:
				if read(mode1) == read(mode2) {
					write(1)
				} else {
					write(0)
				}
			case 99:
				return
			default:
				panic(fmt.Sprintf("unknown opcode %d", opcode))
			}
		}
	}()

	return outputs
}
