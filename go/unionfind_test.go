package main

import (
	"reflect"
	"sort"
	"testing"
)

func TestUnionFind_Basic(t *testing.T) {
	uf := newUnionFind(5)
	if uf.groupCount() != 5 {
		t.Errorf("initial groupCount got %v, want 5", uf.groupCount())
	}
	if uf.isSame(0, 1) {
		t.Error("0 and 1 should not be same initially")
	}
	// 自分自身は同じグループ
	if !uf.isSame(0, 0) {
		t.Error("node should be same as itself")
	}
}

func TestUnionFind_Unite(t *testing.T) {
	uf := newUnionFind(5)
	uf.unite(0, 1)
	uf.unite(1, 2)
	if !uf.isSame(0, 2) {
		t.Error("0 and 2 should be same after unite(0,1), unite(1,2)")
	}
	if uf.isSame(0, 3) {
		t.Error("0 and 3 should not be same")
	}
	if uf.groupCount() != 3 {
		t.Errorf("groupCount got %v, want 3", uf.groupCount())
	}
}

func TestUnionFind_Size(t *testing.T) {
	uf := newUnionFind(5)
	uf.unite(0, 1)
	uf.unite(1, 2)
	if got := uf.size(0); got != 3 {
		t.Errorf("size(0) got %v, want 3", got)
	}
	if got := uf.size(3); got != 1 {
		t.Errorf("size(3) got %v, want 1", got)
	}
	// 全てunite後はサイズ=n
	uf.unite(0, 3)
	uf.unite(0, 4)
	if got := uf.size(0); got != 5 {
		t.Errorf("size(0) after all united got %v, want 5", got)
	}
}

func TestUnionFind_Unite_ReturnValue(t *testing.T) {
	uf := newUnionFind(3)
	if !uf.unite(0, 1) {
		t.Error("first unite should return true")
	}
	if uf.unite(0, 1) {
		t.Error("duplicate unite should return false")
	}
	// 推移的に同じグループ
	uf.unite(1, 2)
	if uf.unite(0, 2) {
		t.Error("transitive same group should return false")
	}
}

func TestUnionFind_Reset(t *testing.T) {
	uf := newUnionFind(4)
	uf.unite(0, 1)
	uf.unite(2, 3)
	uf.reset()
	if uf.groupCount() != 4 {
		t.Errorf("after reset groupCount got %v, want 4", uf.groupCount())
	}
	if uf.isSame(0, 1) {
		t.Error("after reset 0 and 1 should not be same")
	}
	// reset後にuniteできること
	uf.unite(0, 1)
	if !uf.isSame(0, 1) {
		t.Error("should be able to unite after reset")
	}
}

func TestUnionFind_GetGroups(t *testing.T) {
	uf := newUnionFind(5)
	uf.unite(0, 1)
	uf.unite(2, 3)
	groups := uf.getGroups()
	if len(groups) != 3 {
		t.Errorf("got %v groups, want 3", len(groups))
	}
	for _, g := range groups {
		sort.Ints(g)
	}
	sort.Slice(groups, func(i, j int) bool {
		return groups[i][0] < groups[j][0]
	})
	want := [][]int{{0, 1}, {2, 3}, {4}}
	if !reflect.DeepEqual(groups, want) {
		t.Errorf("got %v, want %v", groups, want)
	}
	// 全てunite後はグループ数1
	uf.unite(0, 2)
	uf.unite(0, 4)
	if uf.groupCount() != 1 {
		t.Errorf("expected 1 group, got %d", uf.groupCount())
	}
}

func TestUnionFind_Size1(t *testing.T) {
	// サイズ1
	uf := newUnionFind(1)
	if uf.groupCount() != 1 {
		t.Errorf("expected 1, got %d", uf.groupCount())
	}
	if !uf.isSame(0, 0) {
		t.Error("node 0 should be same as itself")
	}
}
