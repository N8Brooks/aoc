package itertools

import (
	"fmt"
	"iter"
	"reflect"
	"slices"
	"testing"
)

// permutations1: reference implementation used only for testing.
// Equivalent to filtering permutations-with-replacement for distinct indices.
func permutations1[T any](pool []T, r int) [][]T {
	n := len(pool)
	if r > n {
		return nil
	}
	res := make([][]T, 0)
	for tup := range productIndices(n, r) {
		if allDistinct(tup, n) {
			out := make([]T, r)
			for i, k := range tup {
				out[i] = pool[k]
			}
			res = append(res, out)
		}
	}
	return res
}

func TestPermutations_RNegativePanics(t *testing.T) {
	want := "combinatorics.Permutations: r must be a non-negative integer"
	mustPanic(t, want, func() {
		_ = Permutations([]int{1, 2, 3}, -1)
	})
}

func TestPermutations_NEqREq0(t *testing.T) {
	var pool []string
	r := 0
	got := slices.Collect(Permutations(pool, r))
	want := [][]string{{}}
	mustDeepEqual2D(t, got, want)
}

func TestPermutations_RZero(t *testing.T) {
	pool := []string{"a", "b", "c"}
	r := 0
	got := slices.Collect(Permutations(pool, r))
	want := [][]string{{}}
	mustDeepEqual2D(t, got, want)
}

func TestPermutations_NZero(t *testing.T) {
	var pool []string
	r := 1
	got := slices.Collect(Permutations(pool, r))
	if len(got) != 0 {
		t.Fatalf("expected empty result, got %#v", got)
	}
}

func TestPermutations_RGreaterThanN(t *testing.T) {
	pool := []string{"a", "b", "c"}
	r := 4
	got := slices.Collect(Permutations(pool, r))
	var want [][]string
	mustDeepEqual2D(t, got, want)
}

func TestPermutations_NEqR_Strings(t *testing.T) {
	pool := []string{"a", "b", "c"}
	r := 3
	got := slices.Collect(Permutations(pool, r))
	want := [][]string{
		{"a", "b", "c"},
		{"a", "c", "b"},
		{"b", "a", "c"},
		{"b", "c", "a"},
		{"c", "a", "b"},
		{"c", "b", "a"},
	}
	mustDeepEqual2D(t, got, want)
}

func TestPermutations_RLessThanN_Ints(t *testing.T) {
	pool := []int{0, 1, 2, 3}
	r := 3
	got := slices.Collect(Permutations(pool, r))
	want := [][]int{
		{0, 1, 2},
		{0, 1, 3},
		{0, 2, 1},
		{0, 2, 3},
		{0, 3, 1},
		{0, 3, 2},
		{1, 0, 2},
		{1, 0, 3},
		{1, 2, 0},
		{1, 2, 3},
		{1, 3, 0},
		{1, 3, 2},
		{2, 0, 1},
		{2, 0, 3},
		{2, 1, 0},
		{2, 1, 3},
		{2, 3, 0},
		{2, 3, 1},
		{3, 0, 1},
		{3, 0, 2},
		{3, 1, 0},
		{3, 1, 2},
		{3, 2, 0},
		{3, 2, 1},
	}
	mustDeepEqual2D(t, got, want)
}

func TestPermutations_Counts(t *testing.T) {
	for n := range 8 {
		pool := ints(n)
		for r := range 8 {
			t.Run(fmt.Sprintf("perm(%d,%d)", n, r), func(t *testing.T) {
				got := slices.Collect(Permutations(pool, r))
				wantLen := permCount(n, r)
				if len(got) != wantLen {
					t.Fatalf("length mismatch for perm(%d,%d): got %d, want %d", n, r, len(got), wantLen)
				}
			})
		}
	}
}

func TestPermutations_EqualsPermutations1(t *testing.T) {
	for n := range 8 {
		pool := ints(n)
		for r := range 8 {
			t.Run(fmt.Sprintf("permutations1([0..%d),%d)", n, r), func(t *testing.T) {
				got := slices.Collect(Permutations(pool, r))
				want := permutations1(pool, r)
				mustDeepEqual2D(t, got, want)
			})
		}
	}
}

func mustDeepEqual2D[T any](t *testing.T, got, want [][]T) {
	t.Helper()
	if !reflect.DeepEqual(got, want) {
		t.Fatalf("\ngot : %#v\nwant: %#v", got, want)
	}
}

func mustPanic(t *testing.T, wantMsg string, f func()) {
	t.Helper()
	defer func() {
		r := recover()
		if r == nil {
			t.Fatalf("expected panic %q, but no panic occurred", wantMsg)
		}
		switch v := r.(type) {
		case string:
			if v != wantMsg {
				t.Fatalf("panic message mismatch:\n got: %q\nwant: %q", v, wantMsg)
			}
		case error:
			if v.Error() != wantMsg {
				t.Fatalf("panic message mismatch:\n got: %q\nwant: %q", v.Error(), wantMsg)
			}
		default:
			t.Fatalf("panic value mismatch:\n got: %#v\nwant: %q", r, wantMsg)
		}
	}()
	f()
}

func ints(n int) []int {
	a := make([]int, n)
	for i := range a {
		a[i] = i
	}
	return a
}

// permCount returns n P r (order matters, without replacement).
func permCount(n, r int) int {
	if n < 0 {
		panic("n must be a non-negative integer")
	}
	if r < 0 {
		panic("r must be a non-negative integer")
	}
	if r > n {
		return 0
	}
	p := 1
	for i := range r {
		p *= n - i
	}
	return p
}

// productIndices returns all length-r tuples with elements in [0..n),
// with replacement, in odometer/lexicographic order.
func productIndices(n, r int) iter.Seq[[]int] {
	return func(yield func([]int) bool) {
		if r < 0 {
			panic("productIndices: r must be non-negative")
		}
		if n == 0 {
			if r == 0 {
				if !yield(nil) {
					return
				}
			}
			return
		}
		if r == 0 {
			if !yield(nil) {
				return
			}
			return
		}
		idx := make([]int, r)
		for {
			tup := make([]int, r)
			copy(tup, idx)
			if !yield(tup) {
				return
			}
			for i := r - 1; i >= 0; i-- {
				idx[i]++
				if idx[i] < n {
					break
				}
				idx[i] = 0
				if i == 0 {
					return
				}
			}
		}
	}
}

func allDistinct(a []int, n int) bool {
	seen := make([]bool, n)
	for _, x := range a {
		if x < 0 || x >= n {
			return false
		}
		if seen[x] {
			return false
		}
		seen[x] = true
	}
	return true
}
