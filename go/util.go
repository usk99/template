package main

var DY4 = []int{0, 1, 0, -1}
var DX4 = []int{1, 0, -1, 0} // (右,下,左,上)
var DY8 = []int{0, 1, 0, -1, -1, 1, 1, -1}
var DX8 = []int{1, 0, -1, 0, 1, 1, -1, -1} // (右,下,左,上,右上,右下,左下,左上)

const ABC = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
const abc = "abcdefghijklmnopqrstuvwxyz"
const INF = 1001001001
const INFL = int64(4004004003094073385)

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
