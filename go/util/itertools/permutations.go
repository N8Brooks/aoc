package itertools

import "iter"

// Permutations returns an iterator over all length-r permutations (without
// replacement) drawn from the values produced by seq, in lexicographic index
// order (matching Python/itertools behavior). If r > n, it yields nothing.
// If r == 0, it yields a single empty slice.
func Permutations[T any](pool []T, r int) iter.Seq[[]T] {
	if r < 0 {
		panic("combinatorics.Permutations: r must be a non-negative integer")
	}
	return func(yield func([]T) bool) {
		n := len(pool)
		if r > n {
			return
		}

		indices := make([]int, n)
		for i := range indices {
			indices[i] = i
		}

		cycles := make([]int, r)
		for i := range cycles {
			cycles[i] = n - i
		}

		out := make([]T, r)
		copy(out, pool[:r])
		if !yield(out) {
			return
		}

	loop:
		for {
			for i := r - 1; i >= 0; i-- {
				cycles[i]--
				if cycles[i] == 0 {
					tmp := indices[i]
					copy(indices[i:], indices[i+1:])
					indices[n-1] = tmp
					cycles[i] = n - i
				} else {
					j := n - cycles[i]
					indices[i], indices[j] = indices[j], indices[i]
					out := make([]T, r)
					for i, k := range indices[:r] {
						out[i] = pool[k]
					}
					if !yield(out) {
						return
					}
					continue loop
				}
			}
			return
		}
	}
}
