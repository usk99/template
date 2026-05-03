package main

type Map[K comparable, V any] map[K]V

func newMap[K comparable, V any]() Map[K, V] { return make(Map[K, V]) }
func (m Map[K, V]) get(k K) (V, bool)        { v, ok := m[k]; return v, ok }
func (m Map[K, V]) getOr(k K, def V) V {
	if v, ok := m[k]; ok {
		return v
	}
	return def
}
func (m Map[K, V]) set(k K, v V)      { m[k] = v }
func (m Map[K, V]) delete(k K)        { delete(m, k) }
func (m Map[K, V]) contains(k K) bool { _, ok := m[k]; return ok }
func (m Map[K, V]) len() int          { return len(m) }
