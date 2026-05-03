package main

type UnionFind struct {
	parentOrSize []int
	cnt          int
}

func newUnionFind(n int) *UnionFind {
	s := make([]int, n)
	for i := range s {
		s[i] = -1
	}
	return &UnionFind{parentOrSize: s, cnt: n}
}

func (uf *UnionFind) reset() {
	for i := range uf.parentOrSize {
		uf.parentOrSize[i] = -1
	}
	uf.cnt = len(uf.parentOrSize)
}

func (uf *UnionFind) root(i int) int {
	root := i
	for uf.parentOrSize[root] >= 0 {
		root = uf.parentOrSize[root]
	}
	for uf.parentOrSize[i] >= 0 {
		next := uf.parentOrSize[i]
		uf.parentOrSize[i] = root
		i = next
	}
	return root
}

func (uf *UnionFind) isSame(u, v int) bool {
	return uf.root(u) == uf.root(v)
}

func (uf *UnionFind) unite(u, v int) bool {
	x, y := uf.root(u), uf.root(v)
	if x == y {
		return false
	}
	if uf.parentOrSize[x] > uf.parentOrSize[y] {
		x, y = y, x
	}
	uf.parentOrSize[x] += uf.parentOrSize[y]
	uf.parentOrSize[y] = x
	uf.cnt--
	return true
}

func (uf *UnionFind) size(i int) int {
	return -uf.parentOrSize[uf.root(i)]
}

func (uf *UnionFind) groupCount() int {
	return uf.cnt
}

func (uf *UnionFind) getGroups() [][]int {
	n := len(uf.parentOrSize)
	tmp := make([][]int, n)
	for i := range n {
		tmp[uf.root(i)] = append(tmp[uf.root(i)], i)
	}
	result := tmp[:0]
	for _, g := range tmp {
		if len(g) > 0 {
			result = append(result, g)
		}
	}
	return result
}
