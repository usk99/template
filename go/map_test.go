package main

import "testing"

func TestNewMap(t *testing.T) {
	m := newMap[string, int]()
	if m.Len() != 0 {
		t.Errorf("expected 0, got %d", m.Len())
	}
}

func TestMapSet(t *testing.T) {
	m := newMap[string, int]()
	m.Set("a", 1)
	if m.Len() != 1 {
		t.Errorf("expected 1, got %d", m.Len())
	}
}

func TestMapGet(t *testing.T) {
	m := newMap[string, int]()
	m.Set("a", 1)
	v, ok := m.Get("a")
	if !ok || v != 1 {
		t.Errorf("expected 1, got %d", v)
	}
	_, ok = m.Get("b")
	if ok {
		t.Error("expected ok to be false for missing key")
	}
}

func TestMapGetOr(t *testing.T) {
	m := newMap[string, int]()
	m.Set("a", 1)
	if m.GetOr("a", -1) != 1 {
		t.Error("expected 1")
	}
	if m.GetOr("b", -1) != -1 {
		t.Error("expected -1 for missing key")
	}
}

func TestMapDelete(t *testing.T) {
	m := newMap[string, int]()
	m.Set("a", 1)
	m.Delete("a")
	if m.Contains("a") {
		t.Error("expected Contains to be false after Delete")
	}
}

func TestMapContains(t *testing.T) {
	m := newMap[string, int]()
	m.Set("a", 1)
	if !m.Contains("a") {
		t.Error("expected Contains(a) to be true")
	}
	if m.Contains("b") {
		t.Error("expected Contains(b) to be false")
	}
}
