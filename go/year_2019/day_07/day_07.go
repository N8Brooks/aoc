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
	inputs := make(chan int)
	defer close(inputs)
	for phases := range itertools.Permutations([]int{0, 1, 2, 3, 4}, 5) {
		signal := 0
		for _, phase := range phases {
			outputs := intcode(program, inputs)
			inputs <- phase
			inputs <- signal
			signal = <-outputs
		}
		maxSignal = max(maxSignal, signal)
	}
	return maxSignal
}

func Part2(input string) int {
	program := parseProgram(input)
	maxSignal := 0
	for phases := range itertools.Permutations([]int{5, 6, 7, 8, 9}, 5) {
		input0 := make(chan int)
		output := input0
		for _, phase := range phases {
			input := output
			output = intcode(program, input)
			input <- phase
		}

		signal := 0
	loop:
		for {
			select {
			case input0 <- signal:
				signal = <-output
			default:
				break loop
			}
		}
		maxSignal = max(maxSignal, signal)
		close(input0)
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
			mode_1, mode_2 := modes%10, modes/10

			switch opcode {
			case 1:
				write(read(mode_1) + read(mode_2))
			case 2:
				write(read(mode_1) * read(mode_2))
			case 3:
				if input, ok := <-inputs; !ok {
					return
				} else {
					write(input)
				}
			case 4:
				outputs <- read(mode_1)
			case 5:
				param_1, param_2 := read(mode_1), read(mode_2)
				if param_1 != 0 {
					ip = param_2
				}
			case 6:
				param_1, param_2 := read(mode_1), read(mode_2)
				if param_1 == 0 {
					ip = param_2
				}
			case 7:
				if read(mode_1) < read(mode_2) {
					write(1)
				} else {
					write(0)
				}
			case 8:
				if read(mode_1) == read(mode_2) {
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
