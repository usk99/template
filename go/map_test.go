package main

import "testing"

func TestNewMap(t *testing.T) {
	m := newMap[string, int]()
	if m.len() != 0 {
		t.Errorf("expected 0, got %d", m.len())
	}
}

func TestMapSet(t *testing.T) {
	m := newMap[string, int]()
	m.set("a", 1)
	if m.len() != 1 {
		t.Errorf("expected 1, got %d", m.len())
	}
	// 上書き
	m.set("a", 99)
	if m.len() != 1 {
		t.Errorf("expected 1 after overwrite, got %d", m.len())
	}
	v, _ := m.get("a")
	if v != 99 {
		t.Errorf("expected 99 after overwrite, got %d", v)
	}
}

func TestMapGet(t *testing.T) {
	m := newMap[string, int]()
	m.set("a", 1)
	v, ok := m.get("a")
	if !ok || v != 1 {
		t.Errorf("expected (1, true), got (%d, %v)", v, ok)
	}
	// 存在しないキー
	_, ok = m.get("b")
	if ok {
		t.Error("expected ok to be false for missing key")
	}
}

func TestMapGetOr(t *testing.T) {
	m := newMap[string, int]()
	m.set("a", 1)
	if m.getOr("a", -1) != 1 {
		t.Error("expected 1")
	}
	// 存在しないキー → デフォルト値
	if m.getOr("b", -1) != -1 {
		t.Error("expected -1 for missing key")
	}
	// ゼロ値が格納されている場合はゼロ値を返す（デフォルト値ではない）
	m.set("z", 0)
	if m.getOr("z", -1) != 0 {
		t.Error("expected 0 (stored zero value), not default")
	}
}

func TestMapDelete(t *testing.T) {
	m := newMap[string, int]()
	m.set("a", 1)
	m.delete("a")
	if m.contains("a") {
		t.Error("expected contains to be false after delete")
	}
	// 存在しないキーのdelete（パニックしないこと）
	m.delete("nonexistent")
	if m.len() != 0 {
		t.Errorf("expected 0, got %d", m.len())
	}
}

func TestMapContains(t *testing.T) {
	m := newMap[string, int]()
	m.set("a", 1)
	if !m.contains("a") {
		t.Error("expected contains(a) to be true")
	}
	if m.contains("b") {
		t.Error("expected contains(b) to be false")
	}
	// 空マップ
	empty := newMap[string, int]()
	if empty.contains("x") {
		t.Error("expected contains(x) to be false on empty map")
	}
}

func TestMapKeys(t *testing.T) {
	m := newMap[string, int]()
	m.set("a", 1)
	m.set("b", 2)
	ks := m.keys()
	if len(ks) != 2 {
		t.Errorf("expected len 2, got %d", len(ks))
	}
	// 全キーが含まれているか（順序不定）
	km := newSetFrom(ks)
	for _, k := range []string{"a", "b"} {
		if !km.contains(k) {
			t.Errorf("expected key %q in keys()", k)
		}
	}
	// 空マップ
	empty := newMap[string, int]()
	if ks2 := empty.keys(); len(ks2) != 0 {
		t.Errorf("expected empty, got %v", ks2)
	}
}

func TestMapValues(t *testing.T) {
	m := newMap[string, int]()
	m.set("a", 10)
	m.set("b", 20)
	vs := m.values()
	if len(vs) != 2 {
		t.Errorf("expected len 2, got %d", len(vs))
	}
	// 全値が含まれているか（順序不定）
	vm := newSetFrom(vs)
	for _, v := range []int{10, 20} {
		if !vm.contains(v) {
			t.Errorf("expected value %d in values()", v)
		}
	}
	// 空マップ
	empty := newMap[string, int]()
	if vs2 := empty.values(); len(vs2) != 0 {
		t.Errorf("expected empty, got %v", vs2)
	}
}

func TestMapLen(t *testing.T) {
	m := newMap[int, int]()
	if m.len() != 0 {
		t.Errorf("expected 0, got %d", m.len())
	}
	m.set(1, 10)
	m.set(2, 20)
	if m.len() != 2 {
		t.Errorf("expected 2, got %d", m.len())
	}
	m.delete(1)
	if m.len() != 1 {
		t.Errorf("expected 1, got %d", m.len())
	}
}
