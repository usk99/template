package main

import (
	"cmp"
	"slices"
)

type Number interface{ int | int64 | float64 }

// ==========================================================
// 1. スライス拡張
// ==========================================================

func sumAll[T Number](a []T) T {
	var s T
	for _, v := range a {
		s += v
	}
	return s
}

func productAll[T Number](a []T) T {
	var p T = 1
	for _, v := range a {
		p *= v
	}
	return p
}

func average[T Number](a []T) float64 {
	if len(a) == 0 {
		return 0
	}
	var s T
	for _, v := range a {
		s += v
	}
	return float64(s) / float64(len(a))
}

func prefixSum[T Number](a []T) []T {
	res := make([]T, len(a)+1)
	for i, v := range a {
		res[i+1] = res[i] + v
	}
	return res
}

type RlePair[T any] struct {
	Val T
	Cnt int
}

func rle[T comparable](a []T) []RlePair[T] {
	res := make([]RlePair[T], 0, len(a))
	for _, v := range a {
		if len(res) > 0 && res[len(res)-1].Val == v {
			res[len(res)-1].Cnt++
		} else {
			res = append(res, RlePair[T]{v, 1})
		}
	}
	return res
}

func counter[T comparable](a []T) map[T]int {
	m := make(map[T]int)
	for _, v := range a {
		m[v]++
	}
	return m
}

func isPalindrome[T comparable](a []T) bool {
	n := len(a)
	for i := 0; i < n/2; i++ {
		if a[i] != a[n-1-i] {
			return false
		}
	}
	return true
}

// ==========================================================
// 2. Ord拡張
// ==========================================================

func minAll[T cmp.Ordered](a []T) T {
	return slices.Min(a)
}

func maxAll[T cmp.Ordered](a []T) T {
	return slices.Max(a)
}

func indexSort[T cmp.Ordered](a []T) []int {
	idx := make([]int, len(a))
	for i := range idx {
		idx[i] = i
	}
	slices.SortFunc(idx, func(i, j int) int {
		return cmp.Compare(a[i], a[j])
	})
	return idx
}

func compress[T cmp.Ordered](a []T) ([]T, []int) {
	vals := slices.Clone(a)
	slices.Sort(vals)
	vals = slices.Compact(vals)
	encoded := make([]int, len(a))
	for i, v := range a {
		encoded[i], _ = slices.BinarySearch(vals, v)
	}
	return vals, encoded
}

func lowerBound[T cmp.Ordered](a []T, x T) int {
	lo, hi := 0, len(a)
	for lo < hi {
		mid := (lo + hi) / 2
		if a[mid] < x {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
}

func upperBound[T cmp.Ordered](a []T, x T) int {
	lo, hi := 0, len(a)
	for lo < hi {
		mid := (lo + hi) / 2
		if a[mid] <= x {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
}

func nextPermutation[T cmp.Ordered](a []T) bool {
	n := len(a)
	i := n - 2
	for i >= 0 && a[i] >= a[i+1] {
		i--
	}
	if i < 0 {
		return false
	}
	j := n - 1
	for a[j] <= a[i] {
		j--
	}
	a[i], a[j] = a[j], a[i]
	slices.Reverse(a[i+1:])
	return true
}

func prevPermutation[T cmp.Ordered](a []T) bool {
	n := len(a)
	i := n - 2
	for i >= 0 && a[i] <= a[i+1] {
		i--
	}
	if i < 0 {
		return false
	}
	j := n - 1
	for a[j] >= a[i] {
		j--
	}
	a[i], a[j] = a[j], a[i]
	slices.Reverse(a[i+1:])
	return true
}

// ==========================================================
// 3. 数学拡張
// ==========================================================

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func lcm(a, b int) int {
	return a / gcd(a, b) * b
}

func gcdAll(a []int) int {
	if len(a) == 0 {
		return 0
	}
	res := a[0]
	for _, v := range a[1:] {
		res = gcd(res, v)
	}
	return res
}

func lcmAll(a []int) int {
	if len(a) == 0 {
		return 0
	}
	res := a[0]
	for _, v := range a[1:] {
		res = lcm(res, v)
	}
	return res
}

// ==========================================================
// 4. 文字・数値拡張
// ==========================================================

func runeCounter(s string) map[rune]int {
	m := make(map[rune]int)
	for _, c := range s {
		m[c]++
	}
	return m
}

func charToNum(c byte) int {
	return int(c - '0')
}

func charToAlpha(c byte) int {
	if c >= 'a' && c <= 'z' {
		return int(c - 'a')
	}
	return int(c - 'A')
}

func charFromAlpha(i int, upper bool) byte {
	if upper {
		return 'A' + byte(i)
	}
	return 'a' + byte(i)
}

func digits(n int) []int {
	if n == 0 {
		return []int{0}
	}
	res := make([]int, 0, 20)
	for n > 0 {
		res = append(res, n%10)
		n /= 10
	}
	slices.Reverse(res)
	return res
}

func digitSum(n int) int {
	s := 0
	for n > 0 {
		s += n % 10
		n /= 10
	}
	return s
}

func pow(base, exp int) int {
	res := 1
	for exp > 0 {
		if exp&1 == 1 {
			res *= base
		}
		base *= base
		exp >>= 1
	}
	return res
}
