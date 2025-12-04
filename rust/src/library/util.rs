// 便利
#![allow(non_upper_case_globals)]
use itertools::Itertools;
use num_integer::*;
use num_traits::*;
use std::{cmp::*, collections::*, hash::*, iter::*, ops::*};

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
    let (sign, num_str) = s
        .strip_prefix('-')
        .map_or_else(|| ("", &s[..]), |stripped| ("-", stripped));

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

#[rustfmt::skip]
pub trait SliceExt<T> {
    /// 合計値取得
    fn sum_all(&self) -> T where T: Sum;
    /// 積取得
    fn product_all(&self) -> T where T: Product;
    /// 平均値取得
    fn average(&self) -> f64 where T: ToPrimitive;
    /// 累積和取得
    fn cumsum(&self) -> Vec<T> where T: Default + Add<Output = T> + Copy;

    /// 最大公約数取得
    fn gcd_all(&self) -> Option<T> where T: Integer;
    /// 最小公倍数取得
    fn lcm_all(&self) -> Option<T> where T: Integer;
    /// 最小値取得
    fn min_all(&self) -> Option<T> where T: Ord;
    /// 最大値取得
    fn max_all(&self) -> Option<T> where T: Ord;

    /// インデックスソート取得
    fn index_sort(&self) -> Vec<usize> where T: PartialOrd;
    /// ランレングス圧縮取得
    fn rle(&self) -> Vec<(T, usize)> where T: PartialEq + Clone;
    /// 出現数カウント
    fn counter(&self) -> HashMap<T, usize> where T: Hash + Eq + Clone;

    /// lower_bound取得
    fn lower_bound(&self, x: &T) -> usize where T: Ord;
    /// upper_bound取得
    fn upper_bound(&self, x: &T) -> usize where T: Ord;

    /// xと等しい範囲を返す
    fn equal_range(&self, x: &T) -> Range<usize> where T: Ord;
    /// 次の順列
    fn next_permutation(&mut self) -> bool where T: Ord;
    /// 前の順列
    fn prev_permutation(&mut self) -> bool where T: Ord;

    /// 回文判定
    fn is_palindrome(&self) -> bool where T: PartialEq;
}

macro_rules! impl_reduce {
    ($name:ident, $bound:path, $op:expr) => {
        fn $name(&self) -> Option<T>
        where
            T: $bound,
        {
            self.iter().copied().reduce($op)
        }
    };
}
impl<T> SliceExt<T> for [T]
where
    T: Copy,
{
    fn sum_all(&self) -> T
    where
        T: Sum,
    {
        self.iter().copied().sum()
    }

    fn product_all(&self) -> T
    where
        T: Product,
    {
        self.iter().copied().product()
    }

    fn average(&self) -> f64
    where
        T: ToPrimitive,
    {
        if self.is_empty() {
            return 0.0;
        }
        self.iter().map(|&x| x.to_f64().unwrap_or(0.0)).sum::<f64>() / (self.len() as f64)
    }

    fn cumsum(&self) -> Vec<T>
    where
        T: Default + Add<Output = T> + Copy,
    {
        let mut acc = T::default();
        let mut res = Vec::with_capacity(self.len() + 1);
        res.push(acc);
        for &value in self.iter() {
            acc = acc + value;
            res.push(acc);
        }
        res
    }

    // メソッド名,トレイト,処理
    impl_reduce!(gcd_all, Integer, |a, b| a.gcd(&b));
    impl_reduce!(lcm_all, Integer, |a, b| a.lcm(&b));
    impl_reduce!(min_all, Ord, |a, b| a.min(b));
    impl_reduce!(max_all, Ord, |a, b| a.max(b));

    fn index_sort(&self) -> Vec<usize>
    where
        T: PartialOrd,
    {
        let mut indices = (0..self.len()).collect::<Vec<_>>();
        indices.sort_by(|&a, &b| {
            self[a]
                .partial_cmp(&self[b])
                .unwrap_or(Ordering::Equal)
                .then_with(|| a.cmp(&b))
        });
        indices
    }

    fn rle(&self) -> Vec<(T, usize)>
    where
        T: PartialEq + Clone,
    {
        self.iter()
            .cloned()
            .dedup_with_count()
            .map(|(c, v)| (v, c))
            .collect()
    }

    fn counter(&self) -> HashMap<T, usize>
    where
        T: Hash + Eq + Clone,
    {
        let mut map = HashMap::new();
        for v in self.iter().cloned() {
            *map.entry(v).or_insert(0) += 1;
        }
        map
    }

    fn lower_bound(&self, x: &T) -> usize
    where
        T: Ord,
    {
        self.partition_point(|v| v < x)
    }

    fn upper_bound(&self, x: &T) -> usize
    where
        T: Ord,
    {
        self.partition_point(|v| v <= x)
    }

    fn equal_range(&self, x: &T) -> Range<usize>
    where
        T: Ord,
    {
        self.lower_bound(x)..self.upper_bound(x)
    }

    fn next_permutation(&mut self) -> bool
    where
        T: Ord,
    {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while self[i - 1] >= self[j] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        true
    }

    fn prev_permutation(&mut self) -> bool
    where
        T: Ord,
    {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while self[j] >= self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        true
    }

    fn is_palindrome(&self) -> bool
    where
        T: PartialEq,
    {
        if self.len() < 2 {
            return true;
        }
        let len = self.len();
        self.iter()
            .zip(self.iter().rev())
            .take(len / 2)
            .all(|(a, b)| a == b)
    }
}

/// nビットの全パターン取得
pub fn bit_pattern(n: usize) -> Vec<Vec<bool>> {
    let mut bool = Vec::with_capacity(Pow::pow(2, n) as usize);
    for i in 0..(1 << n) {
        let mut b = Vec::with_capacity(n);
        for j in 0..n {
            b.push((i >> j) & 1 == 1);
        }
        bool.push(b);
    }
    bool
}
