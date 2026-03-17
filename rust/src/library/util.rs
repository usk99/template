// 便利
#![allow(non_upper_case_globals)]
use std::{cmp::*, collections::*};

const fn make_abc(base: u8) -> [char; 26] {
    let mut arr = ['\0'; 26];
    let mut i = 0;
    while i < 26 {
        arr[i] = (base + i as u8) as char;
        i += 1;
    }
    arr
}
pub const ABC_U: [char; 26] = make_abc(b'A');
pub const ABC_L: [char; 26] = make_abc(b'a');

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
