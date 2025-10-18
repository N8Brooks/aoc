package day08

import "strings"

func Part1(input string) int {
	trees := NewTrees(input)
	visibility := trees.Visibility()
	return visibility.Count()
}

func Part2(input string) int {
	trees := NewTrees(input)
	n, m := len(trees), len(trees[0])
	var maxScore, up, down, left, right int
	for i, row := range trees {
		for j, tree := range row {
			tree := byte(tree)

			for right = j + 1; right < m; right++ {
				if row[right] >= tree {
					break
				}
			}
			countRight := right - j
			if right == m {
				countRight--
			}

			for left = j - 1; left >= 0; left-- {
				if row[left] >= tree {
					break
				}
			}
			countLeft := j - left
			if left == -1 {
				countLeft--
			}

			for up = i - 1; up >= 0; up-- {
				if trees[up][j] >= tree {
					break
				}
			}
			countUp := i - up
			if up == -1 {
				countUp--
			}

			for down = i + 1; down < m; down++ {
				if trees[down][j] >= tree {
					break
				}
			}
			countDown := down - i
			if down == n {
				countDown--
			}

			score := countRight * countLeft * countUp * countDown
			if score > maxScore {
				maxScore = score
			}
		}
	}

	return maxScore
}

type Trees []string

func NewTrees(input string) Trees {
	return strings.Split(input, "\n")
}

func (trees Trees) Visibility() Visible {
	m, n := len(trees), len(trees[0])
	visible := newVisible(m, n)

	// Compute visibility across rows
	for i, row := range trees {
		left, right := 0, n-1
		var maxLeft, maxRight byte
		for left <= right {
			if maxLeft < maxRight {
				if row[left] > maxLeft {
					maxLeft = row[left]
					visible[i][left] = true
				}
				left++
			} else {
				if row[right] > maxRight {
					maxRight = row[right]
					visible[i][right] = true
				}
				right--
			}
		}
	}

	// Compute visibility across columns
	for j := range n {
		up, down := 0, m-1
		var maxUp, maxDown byte
		for up <= down {
			if maxUp < maxDown {
				if trees[up][j] > maxUp {
					maxUp = trees[up][j]
					visible[up][j] = true
				}
				up++
			} else {
				if trees[down][j] > maxDown {
					maxDown = trees[down][j]
					visible[down][j] = true
				}
				down--
			}
		}
	}

	return visible
}

type Visible [][]bool

func newVisible(height, width int) Visible {
	visible := make([][]bool, height)
	for i := range visible {
		visible[i] = make([]bool, width)
	}
	return visible
}

func (visible *Visible) Count() int {
	total := 0
	for _, row := range *visible {
		for _, isVisible := range row {
			if isVisible {
				total++
			}
		}
	}
	return total
}
