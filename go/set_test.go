package main

import "testing"

func TestNewSet(t *testing.T) {
	s := newSet[int]()
	if s.Len() != 0 {
		t.Errorf("expected 0, got %d", s.Len())
	}
}

func TestNewSetFrom(t *testing.T) {
	s := newSetFrom([]int{1, 2, 3, 2, 1})
	if s.Len() != 3 {
		t.Errorf("expected 3, got %d", s.Len())
	}
}

func TestSetInsert(t *testing.T) {
	s := newSet[int]()
	s.Insert(1)
	if !s.Contains(1) {
		t.Error("expected Contains(1) to be true")
	}
}

func TestSetDelete(t *testing.T) {
	s := newSetFrom([]int{1, 2, 3})
	s.Delete(2)
	if s.Contains(2) {
		t.Error("expected Contains(2) to be false after Delete")
	}
	if s.Len() != 2 {
		t.Errorf("expected 2, got %d", s.Len())
	}
}

func TestSetContains(t *testing.T) {
	s := newSetFrom([]int{1, 2, 3})
	if !s.Contains(1) {
		t.Error("expected Contains(1) to be true")
	}
	if s.Contains(9) {
		t.Error("expected Contains(9) to be false")
	}
}
