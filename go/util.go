package main

func chmax[T int | int64 | float64](a *T, b T) bool {
	if *a < b {
		*a = b
		return true
	}
	return false
}

func chmin[T int | int64 | float64](a *T, b T) bool {
	if *a > b {
		*a = b
		return true
	}
	return false
}
