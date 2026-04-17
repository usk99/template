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
pub type MaxHeap<T> = BinaryHeap<T>;
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
        for v in iter {
            self.push(v);
        }
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
