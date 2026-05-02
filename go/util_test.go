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
