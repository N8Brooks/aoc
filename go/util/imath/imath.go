package imath

type Pos [2]int

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

func ManhattenDistance(a, b Pos) int {
	return Abs(a[0]-b[0]) + Abs(a[1]-b[1])
}

func (pos Pos) Values() (int, int) {
	return pos[0], pos[1]
}
