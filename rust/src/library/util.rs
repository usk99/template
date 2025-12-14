// 便利
#![allow(non_upper_case_globals)]
use std::{cmp::*, collections::*, iter::*, ops::*};

pub const ABC_U: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ABC_L: &str = "abcdefghijklmnopqrstuvwxyz";

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

pub fn bisect<F: FnMut(i64) -> bool>(mut ok: i64, mut ng: i64, mut check: F) -> i64 {
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if check(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
