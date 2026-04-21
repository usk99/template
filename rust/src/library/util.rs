// 便利
#![allow(non_upper_case_globals)]
use std::{cmp::*, collections::*};

pub const ABC: &[char; 52] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub const ABC_U: &[char] = ABC.split_at(26).0;
pub const ABC_L: &[char] = ABC.split_at(26).1;

/// 優先度付きキュー(最大)
#[derive(Clone, Debug)]
pub struct MaxHeap<T> {
    heap: BinaryHeap<T>,
}
impl<T: Ord> MaxHeap<T> {
    pub const fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }
    pub fn push(&mut self, value: T) {
        self.heap.push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.heap.drain()
    }
    /// ソート済みVecに変換 (昇順)
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec()
    }
    /// 順序保証なしでVecに変換
    pub fn into_vec(self) -> Vec<T> {
        self.heap.into_vec()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter()
    }
    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }
    pub fn reserve(&mut self, additional: usize) {
        self.heap.reserve(additional);
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.heap.reserve_exact(additional);
    }
    pub fn shrink_to_fit(&mut self) {
        self.heap.shrink_to_fit();
    }
    /// 他のheapを取り込み、自身に統合する (otherは空になる)
    pub fn append(&mut self, other: &mut Self) {
        self.heap.append(&mut other.heap);
    }
}

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
    /// VecからO(N)で構築
    fn from(vec: Vec<T>) -> Self {
        Self {
            heap: BinaryHeap::from(vec),
        }
    }
}
impl<T: Ord> IntoIterator for MaxHeap<T> {
    type Item = T;
    type IntoIter = std::collections::binary_heap::IntoIter<T>;
    /// 順序保証なしのイテレータ
    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter()
    }
}
impl<T: Ord> FromIterator<T> for MaxHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            heap: iter.into_iter().collect(),
        }
    }
}
impl<T: Ord> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord> Extend<T> for MaxHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.heap.extend(iter);
    }
}

/// 優先度付きキュー(最小)
#[derive(Clone, Debug)]
pub struct MinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
}
impl<T: Ord> MinHeap<T> {
    pub const fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }
    pub fn push(&mut self, value: T) {
        self.heap.push(Reverse(value));
    }
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(value)| value)
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|Reverse(value)| value)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.heap.drain().map(|Reverse(v)| v)
    }
    /// ソート済みVecに変換 (昇順)
    pub fn into_sorted_vec(self) -> Vec<T> {
        // BinaryHeap<Reverse<T>>の昇順は降順なので、逆順にして昇順を得る
        let mut v: Vec<T> = self
            .heap
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(v)| v)
            .collect();
        v.reverse();
        v
    }
    /// 順序保証なしでVecに変換
    pub fn into_vec(self) -> Vec<T> {
        self.heap.into_vec().into_iter().map(|Reverse(v)| v).collect()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter().map(|Reverse(v)| v)
    }
    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }
    pub fn reserve(&mut self, additional: usize) {
        self.heap.reserve(additional);
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.heap.reserve_exact(additional);
    }
    pub fn shrink_to_fit(&mut self) {
        self.heap.shrink_to_fit();
    }
    /// 他のheapを取り込み、自身に統合する (otherは空になる)
    pub fn append(&mut self, other: &mut Self) {
        self.heap.append(&mut other.heap);
    }
}

impl<T: Ord> From<Vec<T>> for MinHeap<T> {
    /// VecからO(N)で構築
    fn from(vec: Vec<T>) -> Self {
        Self {
            heap: vec.into_iter().map(Reverse).collect(),
        }
    }
}
impl<T: Ord> IntoIterator for MinHeap<T> {
    type Item = T;
    type IntoIter = std::iter::Map<
        std::collections::binary_heap::IntoIter<Reverse<T>>,
        fn(Reverse<T>) -> T,
    >;
    /// 順序保証なしのイテレータ
    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter().map(|Reverse(v)| v)
    }
}
impl<T: Ord> FromIterator<T> for MinHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let heap = iter.into_iter().map(Reverse).collect();
        Self { heap }
    }
}
impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord> Extend<T> for MinHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.heap.extend(iter.into_iter().map(Reverse));
    }
}

// Hash関数早い版
pub type Set<V> = rustc_hash::FxHashSet<V>;
#[macro_export]
macro_rules! set {
    ($($x:expr), *) => {{let mut s = Set::default();$( s.insert($x); )*s}};
}
pub type Map<K, V> = rustc_hash::FxHashMap<K, V>;
#[macro_export]
macro_rules! map {
    ($(($k:expr , $v:expr)), * $(,)?) => ({let mut m = Map::default();$( m.insert($k, $v); )*m});
}
