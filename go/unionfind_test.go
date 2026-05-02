package main

import (
	"reflect"
	"sort"
	"testing"
)

func TestUnionFind_Basic(t *testing.T) {
	uf := NewUnionFind(5)
	if uf.GroupCount() != 5 {
		t.Errorf("initial GroupCount got %v, want 5", uf.GroupCount())
	}
	if uf.IsSame(0, 1) {
		t.Error("0 and 1 should not be same initially")
	}
}

func TestUnionFind_Unite(t *testing.T) {
	uf := NewUnionFind(5)
	uf.Unite(0, 1)
	uf.Unite(1, 2)
	if !uf.IsSame(0, 2) {
		t.Error("0 and 2 should be same after Unite(0,1), Unite(1,2)")
	}
	if uf.IsSame(0, 3) {
		t.Error("0 and 3 should not be same")
	}
	if uf.GroupCount() != 3 {
		t.Errorf("GroupCount got %v, want 3", uf.GroupCount())
	}
}

func TestUnionFind_Size(t *testing.T) {
	uf := NewUnionFind(5)
	uf.Unite(0, 1)
	uf.Unite(1, 2)
	if got := uf.Size(0); got != 3 {
		t.Errorf("Size(0) got %v, want 3", got)
	}
	if got := uf.Size(3); got != 1 {
		t.Errorf("Size(3) got %v, want 1", got)
	}
}

func TestUnionFind_Unite_ReturnValue(t *testing.T) {
	uf := NewUnionFind(3)
	if !uf.Unite(0, 1) {
		t.Error("first Unite should return true")
	}
	if uf.Unite(0, 1) {
		t.Error("duplicate Unite should return false")
	}
}

func TestUnionFind_Reset(t *testing.T) {
	uf := NewUnionFind(4)
	uf.Unite(0, 1)
	uf.Unite(2, 3)
	uf.Reset()
	if uf.GroupCount() != 4 {
		t.Errorf("after Reset GroupCount got %v, want 4", uf.GroupCount())
	}
	if uf.IsSame(0, 1) {
		t.Error("after Reset 0 and 1 should not be same")
	}
}

func TestUnionFind_GetGroups(t *testing.T) {
	uf := NewUnionFind(5)
	uf.Unite(0, 1)
	uf.Unite(2, 3)
	groups := uf.GetGroups()
	if len(groups) != 3 {
		t.Errorf("got %v groups, want 3", len(groups))
	}
	// ソートして比較
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
}
