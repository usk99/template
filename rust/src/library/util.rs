// 便利
#![allow(non_upper_case_globals)]
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    ops::{Deref, DerefMut},
};

pub const ABC_U: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ABC_U_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub const ABC_L: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ABC_L_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

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

/// ランレングス圧縮 (Run Length Encoding)
pub fn rle<T: PartialEq + Clone>(s: &[T]) -> Vec<(T, usize)> {
    s.iter()
        .cloned()
        .dedup_with_count()
        .map(|(count, value)| (value, count))
        .collect()
}

pub trait NumVecExt<T> {
    fn sum_all(&self) -> T
    where
        T: std::iter::Sum;

    fn gcd_all(&self) -> Option<T>
    where
        T: num::Integer;

    fn lcm_all(&self) -> Option<T>
    where
        T: num::Integer;
}

impl<T> NumVecExt<T> for Vec<T>
where
    T: Copy,
{
    fn sum_all(&self) -> T
    where
        T: std::iter::Sum,
    {
        self.iter().copied().sum()
    }

    fn gcd_all(&self) -> Option<T>
    where
        T: num::Integer,
    {
        self.iter().copied().reduce(|acc, x| acc.gcd(&x))
    }

    fn lcm_all(&self) -> Option<T>
    where
        T: num::Integer,
    {
        self.iter().copied().reduce(|acc, x| acc.lcm(&x))
    }
}
