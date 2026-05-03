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
func (s Set[T]) Insert(v T)        { s[v] = struct{}{} }
func (s Set[T]) Delete(v T)        { delete(s, v) }
func (s Set[T]) Contains(v T) bool { _, ok := s[v]; return ok }
func (s Set[T]) Len() int          { return len(s) }
