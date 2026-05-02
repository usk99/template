package main

import "testing"

func TestSegMax(t *testing.T) {
	seg := NewSegMax(5)
	seg.Build([]int{3, 1, 4, 1, 5})
	if got := seg.Query(0, 5); got != 5 {
		t.Errorf("Query(0,5) got %v, want 5", got)
	}
	if got := seg.Query(1, 3); got != 4 {
		t.Errorf("Query(1,3) got %v, want 4", got)
	}
	seg.Set(2, 9)
	if got := seg.Query(0, 5); got != 9 {
		t.Errorf("after Set Query(0,5) got %v, want 9", got)
	}
}

func TestSegMin(t *testing.T) {
	seg := NewSegMin(5)
	seg.Build([]int{3, 1, 4, 1, 5})
	if got := seg.Query(0, 5); got != 1 {
		t.Errorf("Query(0,5) got %v, want 1", got)
	}
	if got := seg.Query(2, 5); got != 1 {
		t.Errorf("Query(2,5) got %v, want 1", got)
	}
}

func TestSegSum(t *testing.T) {
	seg := NewSegSum(5)
	seg.Build([]int{1, 2, 3, 4, 5})
	if got := seg.Query(0, 5); got != 15 {
		t.Errorf("Query(0,5) got %v, want 15", got)
	}
	if got := seg.Query(1, 4); got != 9 {
		t.Errorf("Query(1,4) got %v, want 9", got)
	}
	seg.Set(0, 10)
	if got := seg.QueryAll(); got != 24 {
		t.Errorf("QueryAll got %v, want 24", got)
	}
}

func TestSegGcd(t *testing.T) {
	seg := NewSegGcd(4)
	seg.Build([]int{12, 8, 6, 4})
	if got := seg.Query(0, 4); got != 2 {
		t.Errorf("Query(0,4) got %v, want 2", got)
	}
	if got := seg.Query(0, 2); got != 4 {
		t.Errorf("Query(0,2) got %v, want 4", got)
	}
}

func TestSegLcm(t *testing.T) {
	seg := NewSegLcm(3)
	seg.Build([]int{2, 3, 4})
	if got := seg.Query(0, 3); got != 12 {
		t.Errorf("Query(0,3) got %v, want 12", got)
	}
}

func TestSegGet(t *testing.T) {
	seg := NewSegSum(4)
	seg.Build([]int{10, 20, 30, 40})
	if got := seg.Get(2); got != 30 {
		t.Errorf("Get(2) got %v, want 30", got)
	}
}

func TestSegCustom(t *testing.T) {
	// 区間積
	seg := NewSeg(4, func(a, b int) int { return a * b }, 1)
	seg.Build([]int{1, 2, 3, 4})
	if got := seg.Query(0, 4); got != 24 {
		t.Errorf("Query(0,4) got %v, want 24", got)
	}
}
