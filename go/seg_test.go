package main

import "testing"

func TestSegMax(t *testing.T) {
	s := newSegMax(5)
	s.build([]int{3, 1, 4, 1, 5})
	if got := s.query(0, 5); got != 5 {
		t.Errorf("query(0,5) got %v, want 5", got)
	}
	if got := s.query(1, 3); got != 4 {
		t.Errorf("query(1,3) got %v, want 4", got)
	}
	s.set(2, 9)
	if got := s.query(0, 5); got != 9 {
		t.Errorf("after set query(0,5) got %v, want 9", got)
	}
	// 単一要素区間
	if got := s.query(0, 1); got != 3 {
		t.Errorf("query(0,1) got %v, want 3", got)
	}
}

func TestSegMin(t *testing.T) {
	s := newSegMin(5)
	s.build([]int{3, 1, 4, 1, 5})
	if got := s.query(0, 5); got != 1 {
		t.Errorf("query(0,5) got %v, want 1", got)
	}
	if got := s.query(2, 5); got != 1 {
		t.Errorf("query(2,5) got %v, want 1", got)
	}
	// 単一要素区間
	if got := s.query(4, 5); got != 5 {
		t.Errorf("query(4,5) got %v, want 5", got)
	}
}

func TestSegSum(t *testing.T) {
	s := newSegSum(5)
	s.build([]int{1, 2, 3, 4, 5})
	if got := s.query(0, 5); got != 15 {
		t.Errorf("query(0,5) got %v, want 15", got)
	}
	if got := s.query(1, 4); got != 9 {
		t.Errorf("query(1,4) got %v, want 9", got)
	}
	s.set(0, 10)
	if got := s.queryAll(); got != 24 {
		t.Errorf("queryAll got %v, want 24", got)
	}
	// 単一要素区間
	if got := s.query(2, 3); got != 3 {
		t.Errorf("query(2,3) got %v, want 3", got)
	}
}

func TestSegGcd(t *testing.T) {
	s := newSegGcd(4)
	s.build([]int{12, 8, 6, 4})
	if got := s.query(0, 4); got != 2 {
		t.Errorf("query(0,4) got %v, want 2", got)
	}
	if got := s.query(0, 2); got != 4 {
		t.Errorf("query(0,2) got %v, want 4", got)
	}
}

func TestSegLcm(t *testing.T) {
	s := newSegLcm(3)
	s.build([]int{2, 3, 4})
	if got := s.query(0, 3); got != 12 {
		t.Errorf("query(0,3) got %v, want 12", got)
	}
}

func TestSegGet(t *testing.T) {
	s := newSegSum(4)
	s.build([]int{10, 20, 30, 40})
	if got := s.get(2); got != 30 {
		t.Errorf("get(2) got %v, want 30", got)
	}
	// 先頭・末尾
	if got := s.get(0); got != 10 {
		t.Errorf("get(0) got %v, want 10", got)
	}
	if got := s.get(3); got != 40 {
		t.Errorf("get(3) got %v, want 40", got)
	}
}

func TestSegCustom(t *testing.T) {
	// 区間積
	s := newSeg(4, func(a, b int) int { return a * b }, 1)
	s.build([]int{1, 2, 3, 4})
	if got := s.query(0, 4); got != 24 {
		t.Errorf("query(0,4) got %v, want 24", got)
	}
	// 単一要素
	if got := s.query(2, 3); got != 3 {
		t.Errorf("query(2,3) got %v, want 3", got)
	}
}

func TestSegSize1(t *testing.T) {
	// サイズ1のセグ木
	s := newSegSum(1)
	s.set(0, 42)
	if got := s.queryAll(); got != 42 {
		t.Errorf("size-1 queryAll got %v, want 42", got)
	}
}
