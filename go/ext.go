package main

import (
	"cmp"
	"slices"
)

// ==========================================================
// 1. スライス拡張
// ==========================================================

// 合計
func SumAll[T int | int64 | float64](a []T) T {
	var s T
	for _, v := range a {
		s += v
	}
	return s
}

// 積
func ProductAll[T int | int64 | float64](a []T) T {
	var p T = 1
	for _, v := range a {
		p *= v
	}
	return p
}

// 平均（空のとき0）
func Average(a []int) float64 {
	if len(a) == 0 {
		return 0
	}
	s := 0
	for _, v := range a {
		s += v
	}
	return float64(s) / float64(len(a))
}

// 累積和（長さ len(a)+1、先頭は0）
func PrefixSum(a []int) []int {
	res := make([]int, len(a)+1)
	for i, v := range a {
		res[i+1] = res[i] + v
	}
	return res
}

// ランレングス圧縮の1要素
type RLEPair[T any] struct {
	Val T
	Cnt int
}

// ランレングス圧縮
func RLE[T comparable](a []T) []RLEPair[T] {
	var res []RLEPair[T]
	for _, v := range a {
		if len(res) > 0 && res[len(res)-1].Val == v {
			res[len(res)-1].Cnt++
		} else {
			res = append(res, RLEPair[T]{v, 1})
		}
	}
	return res
}

// 各要素の出現回数
func Counter[T comparable](a []T) map[T]int {
	m := make(map[T]int)
	for _, v := range a {
		m[v]++
	}
	return m
}

// 回文判定
func IsPalindrome[T comparable](a []T) bool {
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

// 最小値
func MinAll[T cmp.Ordered](a []T) T {
	return slices.Min(a)
}

// 最大値
func MaxAll[T cmp.Ordered](a []T) T {
	return slices.Max(a)
}

// ソート後のインデックス列
func IndexSort[T cmp.Ordered](a []T) []int {
	idx := make([]int, len(a))
	for i := range idx {
		idx[i] = i
	}
	slices.SortFunc(idx, func(i, j int) int {
		return cmp.Compare(a[i], a[j])
	})
	return idx
}

// 座標圧縮（ユニーク値列, 圧縮後インデックス列）
func Compress[T cmp.Ordered](a []T) ([]T, []int) {
	vals := slices.Clone(a)
	slices.Sort(vals)
	vals = slices.Compact(vals)
	encoded := make([]int, len(a))
	for i, v := range a {
		encoded[i], _ = slices.BinarySearch(vals, v)
	}
	return vals, encoded
}

// x 以上の最初のインデックス（ソート済み前提）
func LowerBound[T cmp.Ordered](a []T, x T) int {
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

// x より大きい最初のインデックス（ソート済み前提）
func UpperBound[T cmp.Ordered](a []T, x T) int {
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

// 次の順列（最後の順列なら false）
func NextPermutation[T cmp.Ordered](a []T) bool {
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

// 前の順列（最初の順列なら false）
func PrevPermutation[T cmp.Ordered](a []T) bool {
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

// 全要素の最大公約数
func GcdAll(a []int) int {
	if len(a) == 0 {
		return 0
	}
	res := a[0]
	for _, v := range a[1:] {
		res = gcd(res, v)
	}
	return res
}

// 全要素の最小公倍数
func LcmAll(a []int) int {
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

// 文字列の各文字の出現回数
func RuneCounter(s string) map[rune]int {
	m := make(map[rune]int)
	for _, c := range s {
		m[c]++
	}
	return m
}

// 数字文字→数値（'0'→0）
func CharToNum(c byte) int {
	return int(c - '0')
}

// アルファベット→0始まりインデックス（'a','A'→0）
func CharToAlpha(c byte) int {
	if c >= 'a' && c <= 'z' {
		return int(c - 'a')
	}
	return int(c - 'A')
}

// 0始まりインデックス→アルファベット文字
func CharFromAlpha(i int, upper bool) byte {
	if upper {
		return 'A' + byte(i)
	}
	return 'a' + byte(i)
}

// 整数→各桁のスライス（0→[0]）
func Digits(n int) []int {
	if n == 0 {
		return []int{0}
	}
	var res []int
	for n > 0 {
		res = append(res, n%10)
		n /= 10
	}
	slices.Reverse(res)
	return res
}

// 各桁の和
func DigitSum(n int) int {
	s := 0
	for n > 0 {
		s += n % 10
		n /= 10
	}
	return s
}

// 整数の累乗（繰り返し二乗法）
func Pow(base, exp int) int {
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
