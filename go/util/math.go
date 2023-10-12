package util

func Sign(n int) int {
	var pos, neg int
	if n > 0 {
		pos = 1
	}
	if n < 0 {
		neg = 1
	}
	return pos - neg
}

func Abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}
