package main

func chmax[T Number](a *T, b T) bool {
	if *a < b {
		*a = b
		return true
	}
	return false
}

func chmin[T Number](a *T, b T) bool {
	if *a > b {
		*a = b
		return true
	}
	return false
}

func abs[T Number](a T) T {
	if a < 0 {
		return -a
	}
	return a
}

func absDiff[T Number](a, b T) T {
	return abs(a - b)
}
