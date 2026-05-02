package main

type UnionFind struct {
	parentOrSize []int
	groupCount   int
}

func NewUnionFind(n int) *UnionFind {
	s := make([]int, n)
	for i := range s {
		s[i] = -1
	}
	return &UnionFind{parentOrSize: s, groupCount: n}
}

func (uf *UnionFind) Reset() {
	for i := range uf.parentOrSize {
		uf.parentOrSize[i] = -1
	}
	uf.groupCount = len(uf.parentOrSize)
}

func (uf *UnionFind) Root(i int) int {
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

func (uf *UnionFind) IsSame(u, v int) bool {
	return uf.Root(u) == uf.Root(v)
}

func (uf *UnionFind) Unite(u, v int) bool {
	x, y := uf.Root(u), uf.Root(v)
	if x == y {
		return false
	}
	if uf.parentOrSize[x] > uf.parentOrSize[y] {
		x, y = y, x
	}
	uf.parentOrSize[x] += uf.parentOrSize[y]
	uf.parentOrSize[y] = x
	uf.groupCount--
	return true
}

func (uf *UnionFind) Size(i int) int {
	return -uf.parentOrSize[uf.Root(i)]
}

func (uf *UnionFind) GroupCount() int {
	return uf.groupCount
}

func (uf *UnionFind) GetGroups() [][]int {
	n := len(uf.parentOrSize)
	tmp := make([][]int, n)
	for i := range n {
		tmp[uf.Root(i)] = append(tmp[uf.Root(i)], i)
	}
	result := tmp[:0]
	for _, g := range tmp {
		if len(g) > 0 {
			result = append(result, g)
		}
	}
	return result
}
