package util

// Reverses a slice in place
func Reverse[T any](slice []T) {
	for i, j := 0, len(slice)-1; i < j; i, j = i+1, j-1 {
		slice[i], slice[j] = slice[j], slice[i]
	}
}

// Returns the popped element and the new slice
func Pop[T any](oldSlice []T) (element T, newSlice []T) {
	n := len(oldSlice) - 1
	element, newSlice = oldSlice[n], oldSlice[:n]
	return
}
