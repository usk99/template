package main

// 使用例（区間最小値）:
//   seg := NewSeg(n, func(a, b int) int { return min(a, b) }, 1<<62)
//   seg.Set(i, v)
//   seg.Query(l, r) // [l, r)

type Seg[T any] struct {
	n    int
	size int
	log  int
	d    []T
	op   func(T, T) T
	e    T
}

// NewSeg は n 要素のセグ木を単位元 e と演算 op で初期化する
func NewSeg[T any](n int, op func(T, T) T, e T) *Seg[T] {
	log := 0
	for (1 << log) < n {
		log++
	}
	size := 1 << log
	d := make([]T, 2*size)
	for i := range d {
		d[i] = e
	}
	return &Seg[T]{n: n, size: size, log: log, d: d, op: op, e: e}
}

// Build は slice から一括構築する（O(n)）
func (s *Seg[T]) Build(a []T) {
	for i, v := range a {
		s.d[s.size+i] = v
	}
	for i := s.size - 1; i >= 1; i-- {
		s.d[i] = s.op(s.d[2*i], s.d[2*i+1])
	}
}

func (s *Seg[T]) update(k int) {
	s.d[k] = s.op(s.d[2*k], s.d[2*k+1])
}

// Set は i 番目の要素を v に更新する
func (s *Seg[T]) Set(i int, v T) {
	i += s.size
	s.d[i] = v
	for j := 1; j <= s.log; j++ {
		s.update(i >> j)
	}
}

// Get は i 番目の要素を返す
func (s *Seg[T]) Get(i int) T {
	return s.d[i+s.size]
}

// Query は半開区間 [l, r) の総積を返す
func (s *Seg[T]) Query(l, r int) T {
	left, right := s.e, s.e
	l += s.size
	r += s.size
	for l < r {
		if l&1 != 0 {
			left = s.op(left, s.d[l])
			l++
		}
		if r&1 != 0 {
			r--
			right = s.op(s.d[r], right)
		}
		l >>= 1
		r >>= 1
	}
	return s.op(left, right)
}

// QueryAll は全体の総積を返す
func (s *Seg[T]) QueryAll() T {
	return s.d[1]
}

// ==========================================================
// 頻出セグ木
// ==========================================================

// 区間最大値
func NewSegMax(n int) *Seg[int] {
	return NewSeg(n, func(a, b int) int { return max(a, b) }, -1<<62)
}

// 区間最小値
func NewSegMin(n int) *Seg[int] {
	return NewSeg(n, func(a, b int) int { return min(a, b) }, 1<<62)
}

// 区間和
func NewSegSum(n int) *Seg[int] {
	return NewSeg(n, func(a, b int) int { return a + b }, 0)
}

// 区間GCD
func NewSegGcd(n int) *Seg[int] {
	return NewSeg(n, gcd, 0)
}

// 区間LCM
func NewSegLcm(n int) *Seg[int] {
	return NewSeg(n, lcm, 1)
}
