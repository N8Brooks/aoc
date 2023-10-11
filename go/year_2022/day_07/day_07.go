package year_2022

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) uint {
	root := ParseFilesystem(input)
	var totalSize uint
	root.Visit(func(size uint) {
		if size <= 100_000 {
			totalSize += size
		}
	})
	return totalSize
}

const AvailableSize = uint(70_000_000)
const UpdateSize = uint(30_000_000)

func Part2(input string) uint {
	root := ParseFilesystem(input)
	unusedSize := AvailableSize - root.size
	minDeleteSize := UpdateSize - unusedSize
	curDeleteSize := root.size
	root.Visit(func(size uint) {
		if minDeleteSize <= size && size < curDeleteSize {
			curDeleteSize = size
		}
	})
	return curDeleteSize
}

// Parses the file system given a log of prompts and output
func ParseFilesystem(input string) Directory {
	input = strings.TrimPrefix(input, "$ cd /\n$ ")
	root := Directory{nil, map[string]*Directory{}, 0}
	node := &root
	for _, input := range strings.Split(input, "\n$ ") {
		node = node.ParsePrompt(input)
	}
	return root
}

type Directory struct {
	// Root directory has a parent of `nil`
	parent   *Directory
	contents map[string]*Directory
	size     uint
}

// Parses the `cd` or `ls` prompt along with its output
func (directory *Directory) ParsePrompt(input string) *Directory {
	line, input, _ := strings.Cut(input, "\n")
	prompt, arg, _ := strings.Cut(line, " ")
	switch prompt {
	case "cd":
		return directory.ChangeDirectory(arg)
	case "ls":
		directory.UpdateContents(input)
		return directory
	default:
		panic(fmt.Sprintf("Unknown prompt %s", prompt))
	}
}

// Returns the directory after calling `cd` with `arg`
func (directory *Directory) ChangeDirectory(arg string) *Directory {
	if arg == ".." {
		return directory.parent
	}
	return directory.contents[arg]
}

// Updates the contents of a `Directory` with the output of `ls`
func (directory *Directory) UpdateContents(input string) {
	for _, line := range strings.Split(input, "\n") {
		a, b, _ := strings.Cut(line, " ")
		if a == "dir" {
			directory.contents[b] = &Directory{directory, map[string]*Directory{}, 0}
		} else {
			size, _ := strconv.Atoi(a)
			directory.PropagateSize(uint(size))
		}
	}
}

// Updates a `Directory` and each of its ancestors for a file size
func (node *Directory) PropagateSize(size uint) {
	for node != nil {
		node.size += size
		node = node.parent
	}
}

// Calls `visitor` with each file size
func (directory *Directory) Visit(visitor func(size uint)) {
	stack := []*Directory{directory}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		visitor(node.size)
		for _, child := range node.contents {
			stack = append(stack, child)
		}
	}
}
