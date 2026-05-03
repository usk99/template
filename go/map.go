package main

type Map[K comparable, V any] map[K]V

func newMap[K comparable, V any]() Map[K, V] { return make(Map[K, V]) }
func (m Map[K, V]) Get(k K) (V, bool)        { v, ok := m[k]; return v, ok }
func (m Map[K, V]) GetOr(k K, def V) V {
	if v, ok := m[k]; ok {
		return v
	}
	return def
}
func (m Map[K, V]) Set(k K, v V)      { m[k] = v }
func (m Map[K, V]) Delete(k K)        { delete(m, k) }
func (m Map[K, V]) Contains(k K) bool { _, ok := m[k]; return ok }
func (m Map[K, V]) Len() int          { return len(m) }
