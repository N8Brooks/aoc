package util

import "container/heap"

// IntHeap is a min-heap of ints.
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func (h *IntHeap) PushPop(x int) int {
	old := *h
	if len(old) > 0 && old[0] < x {
		x, old[0] = old[0], x
		heap.Fix(h, 0)
	}
	return x
}

func (h *IntHeap) Sum() int {
	var sum int
	for _, x := range []int(*h) {
		sum += x
	}
	return sum
}

func (h *IntHeap) Prod() int {
	prod := 1
	for _, x := range []int(*h) {
		prod *= x
	}
	return prod
}
