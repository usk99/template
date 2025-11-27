// 便利
#![allow(non_upper_case_globals)]
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    ops::{Deref, DerefMut},
};

pub const ABC_U: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ABC_L: &str = "abcdefghijklmnopqrstuvwxyz";

// カンマ区切り文字列
pub fn comma<T: ToString>(n: T) -> String {
    let s = n.to_string();
    let (sign, num_str) = if s.starts_with('-') {
        ("-", &s[1..])
    } else {
        ("", &s[..])
    };

    let mut res = String::with_capacity(s.len() + s.len() / 3);
    res.push_str(sign);

    let offset = num_str.len() % 3;
    if offset > 0 {
        res.push_str(&num_str[..offset]);
    }

    for i in (offset..num_str.len()).step_by(3) {
        if i != 0 || offset > 0 {
            res.push(',');
        }
        res.push_str(&num_str[i..i + 3]);
    }
    res
}

/// 優先度付きキュー(最大)
pub type MaxHeap<T> = BinaryHeap<T>;
/// 優先度付きキュー(最小)
#[derive(Clone, Debug)]
pub struct MinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
}
impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
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
}
impl<T> Deref for MinHeap<T> {
    type Target = BinaryHeap<Reverse<T>>;
    fn deref(&self) -> &Self::Target {
        &self.heap
    }
}

impl<T> DerefMut for MinHeap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap
    }
}

/// インデックスソート
pub fn index_sort<T: PartialOrd>(arr: &[T]) -> Vec<usize> {
    let mut indices = (0..arr.len()).collect::<Vec<_>>();
    indices.sort_by(|&a, &b| {
        arr[a]
            .partial_cmp(&arr[b])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    indices
}

/// 最大公約数
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// 最小公倍数
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

/// ランレングス圧縮
pub fn rle<T: PartialEq + Copy>(s: &[T]) -> Vec<(T, usize)> {
    if s.is_empty() {
        return vec![];
    }
    let mut res = vec![];
    let mut pre = s[0];
    let mut cnt = 1;
    for &c in &s[1..] {
        if c == pre {
            cnt += 1;
        } else {
            res.push((pre, cnt));
            pre = c;
            cnt = 1;
        }
    }
    res.push((pre, cnt));
    res
}
