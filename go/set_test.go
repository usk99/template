package main

import "testing"

func TestNewSet(t *testing.T) {
	s := newSet[int]()
	if s.len() != 0 {
		t.Errorf("expected 0, got %d", s.len())
	}
}

func TestNewSetFrom(t *testing.T) {
	// 重複排除
	s := newSetFrom([]int{1, 2, 3, 2, 1})
	if s.len() != 3 {
		t.Errorf("expected 3, got %d", s.len())
	}
	// 空スライス
	s2 := newSetFrom([]int{})
	if s2.len() != 0 {
		t.Errorf("expected 0, got %d", s2.len())
	}
	// 全要素同じ
	s3 := newSetFrom([]int{5, 5, 5})
	if s3.len() != 1 {
		t.Errorf("expected 1, got %d", s3.len())
	}
}

func TestSetInsert(t *testing.T) {
	s := newSet[int]()
	s.insert(1)
	if !s.contains(1) {
		t.Error("expected contains(1) to be true")
	}
	// 同じ要素を二重insert
	s.insert(1)
	if s.len() != 1 {
		t.Errorf("expected len 1 after duplicate insert, got %d", s.len())
	}
}

func TestSetDelete(t *testing.T) {
	s := newSetFrom([]int{1, 2, 3})
	s.delete(2)
	if s.contains(2) {
		t.Error("expected contains(2) to be false after delete")
	}
	if s.len() != 2 {
		t.Errorf("expected 2, got %d", s.len())
	}
	// 存在しない要素のdelete（パニックしないこと）
	s.delete(99)
	if s.len() != 2 {
		t.Errorf("expected 2 after deleting non-existent, got %d", s.len())
	}
}

func TestSetContains(t *testing.T) {
	s := newSetFrom([]int{1, 2, 3})
	if !s.contains(1) {
		t.Error("expected contains(1) to be true")
	}
	if s.contains(9) {
		t.Error("expected contains(9) to be false")
	}
	// 空セット
	empty := newSet[int]()
	if empty.contains(0) {
		t.Error("expected contains(0) to be false on empty set")
	}
}

func TestSetLen(t *testing.T) {
	s := newSet[string]()
	if s.len() != 0 {
		t.Errorf("expected 0, got %d", s.len())
	}
	s.insert("a")
	s.insert("b")
	if s.len() != 2 {
		t.Errorf("expected 2, got %d", s.len())
	}
	s.delete("a")
	if s.len() != 1 {
		t.Errorf("expected 1, got %d", s.len())
	}
}
