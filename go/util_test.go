package main

import "testing"

func TestChmax(t *testing.T) {
	a := 3
	if !chmax(&a, 5) || a != 5 {
		t.Errorf("chmax should update: got %v", a)
	}
	if chmax(&a, 3) || a != 5 {
		t.Errorf("chmax should not update: got %v", a)
	}
}

func TestChmin(t *testing.T) {
	a := 5
	if !chmin(&a, 3) || a != 3 {
		t.Errorf("chmin should update: got %v", a)
	}
	if chmin(&a, 5) || a != 3 {
		t.Errorf("chmin should not update: got %v", a)
	}
}

func TestAbs(t *testing.T) {
	if abs(-5) != 5 {
		t.Errorf("abs(-5) got %v, want 5", abs(-5))
	}
	if abs(3) != 3 {
		t.Errorf("abs(3) got %v, want 3", abs(3))
	}
	if abs(0) != 0 {
		t.Errorf("abs(0) got %v, want 0", abs(0))
	}
}

func TestAbsDiff(t *testing.T) {
	if absDiff(3, 7) != 4 {
		t.Errorf("absDiff(3,7) got %v, want 4", absDiff(3, 7))
	}
	if absDiff(7, 3) != 4 {
		t.Errorf("absDiff(7,3) got %v, want 4", absDiff(7, 3))
	}
	if absDiff(5, 5) != 0 {
		t.Errorf("absDiff(5,5) got %v, want 0", absDiff(5, 5))
	}
}
