use std::{cmp::Reverse, collections::BinaryHeap};
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
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    pub fn append(&mut self, other: &mut Self) {
        self.heap.append(&mut other.heap);
    }
    /// 順序保証なしで全要素を取り出す (元のヒープは空になる)
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.heap.drain()
    }
    /// 順序保証なしで全要素を参照
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter()
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
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    pub fn append(&mut self, other: &mut Self) {
        self.heap.append(&mut other.heap);
    }
    /// 順序保証なしで全要素を取り出す (元のヒープは空になる)
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.heap.drain().map(|Reverse(v)| v)
    }
    /// 順序保証なしで全要素を参照
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter().map(|Reverse(v)| v)
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
    type IntoIter =
        std::iter::Map<std::collections::binary_heap::IntoIter<Reverse<T>>, fn(Reverse<T>) -> T>;
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
