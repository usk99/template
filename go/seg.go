package main

// 使用例（区間最小値）:
//   seg := newSeg(n, func(a, b int) int { return min(a, b) }, 1<<62)
//   seg.set(i, v)
//   seg.query(l, r) // [l, r)

type Seg[T any] struct {
	n    int
	size int
	log  int
	d    []T
	op   func(T, T) T
	e    T
}

func newSeg[T any](n int, op func(T, T) T, e T) *Seg[T] {
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

func (s *Seg[T]) build(a []T) {
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

func (s *Seg[T]) set(i int, v T) {
	i += s.size
	s.d[i] = v
	for j := 1; j <= s.log; j++ {
		s.update(i >> j)
	}
}

func (s *Seg[T]) get(i int) T {
	return s.d[i+s.size]
}

func (s *Seg[T]) query(l, r int) T {
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

func (s *Seg[T]) queryAll() T {
	return s.d[1]
}

// ==========================================================
// 頻出セグ木
// ==========================================================

func newSegMax(n int) *Seg[int] {
	return newSeg(n, func(a, b int) int { return max(a, b) }, -1<<62)
}

func newSegMin(n int) *Seg[int] {
	return newSeg(n, func(a, b int) int { return min(a, b) }, 1<<62)
}

func newSegSum(n int) *Seg[int] {
	return newSeg(n, func(a, b int) int { return a + b }, 0)
}

func newSegGcd(n int) *Seg[int] {
	return newSeg(n, gcd, 0)
}

func newSegLcm(n int) *Seg[int] {
	return newSeg(n, lcm, 1)
}
