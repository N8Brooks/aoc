package year_2022

import ()

func Part1(input string) int {
	return FindStartOfPacketMarker(input, 4)
}

func Part2(input string) int {
	return FindStartOfPacketMarker(input, 14)
}

func FindStartOfPacketMarker(input string, n int) int {
	k := n - 1
	for j := 0; j < len(input); j++ {
		stop := AtLeast0(j - n + 1)
		for i := j - 1; i >= stop; i-- {
			if input[i] != input[j] {
				continue
			}
			if i+n > k {
				k = i + n
			}
			break
		}
		if j == k {
			return j + 1
		}
	}
	panic("No start of packet marker found")
}

func AtLeast0(n int) int {
	if n > 0 {
		return n
	}
	return 0
}
