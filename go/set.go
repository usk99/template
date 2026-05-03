package main

type Set[T comparable] map[T]struct{}

func newSet[T comparable]() Set[T] { return make(Set[T]) }
func newSetFrom[T comparable](a []T) Set[T] {
	s := make(Set[T], len(a))
	for _, v := range a {
		s[v] = struct{}{}
	}
	return s
}
func (s Set[T]) insert(v T)        { s[v] = struct{}{} }
func (s Set[T]) delete(v T)        { delete(s, v) }
func (s Set[T]) contains(v T) bool { _, ok := s[v]; return ok }
func (s Set[T]) len() int          { return len(s) }
func (s Set[T]) toSlice() []T {
	res := make([]T, 0, len(s))
	for v := range s {
		res = append(res, v)
	}
	return res
}
