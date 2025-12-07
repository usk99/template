// 便利
#![allow(non_upper_case_globals)]
use itertools::Itertools;
use num_integer::*;
use num_traits::*;
use std::{cmp::*, collections::*, fmt::Display, hash::*, iter::*, ops::*};

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
    let bytes = s.as_bytes();

    // 負の数の処理
    let (sign_len, num_slice) = if bytes.first() == Some(&b'-') {
        (1, &bytes[1..])
    } else {
        (0, bytes)
    };

    let len = num_slice.len();
    if len <= 3 {
        return s;
    }

    // 必要な容量を一度に確保（アロケーション回数削減）
    let comma_cnt = (len - 1) / 3;
    let mut res = Vec::with_capacity(s.len() + comma_cnt);

    // 符号があれば追加
    if sign_len > 0 {
        res.push(b'-');
    }

    // 先頭の端数部分（1〜3文字）を計算して追加
    // 例: 12345 (len=5) -> head=2 ("12")
    let head = (len - 1) % 3 + 1;
    res.extend_from_slice(&num_slice[..head]);

    // 残りを3文字ごとの塊（チャンク）にしてコンマと共に追加
    for chunk in num_slice[head..].chunks(3) {
        res.push(b',');
        res.extend_from_slice(chunk);
    }

    // SAFETY: 元が数字と記号のみの文字列なのでUTF-8チェックをスキップして高速化
    unsafe { String::from_utf8_unchecked(res) }
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
    /// 文字列化
    fn to_string(&self,sep:&str)->String where T:Display;

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
    fn to_string(&self, sep: &str) -> String
    where
        T: Display,
    {
        self.iter().join(sep)
    }

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

pub trait StringExt {
    fn counter(&self) -> HashMap<char, usize>;
}
impl StringExt for &str {
    fn counter(&self) -> HashMap<char, usize> {
        let chars = self.chars().collect::<Vec<_>>();
        chars.counter()
    }
}

pub trait CharExt {
    fn to_num(self) -> usize;
    fn is_lower(&self) -> bool;
    fn is_upper(&self) -> bool;
}
impl CharExt for char {
    fn to_num(self) -> usize {
        (self as u8 - b'0') as usize
    }
    fn is_lower(&self) -> bool {
        self.is_lowercase()
    }
    fn is_upper(&self) -> bool {
        self.is_uppercase()
    }
}

pub trait GridExt<T> {
    /// 90度右回転（時計回り）
    fn rotate_right(&self) -> Vec<Vec<T>>;
    /// 90度左回転（反時計回り）
    fn rotate_left(&self) -> Vec<Vec<T>>;
    /// 180度回転
    fn rotate_180(&self) -> Vec<Vec<T>>;
    /// 転置（行と列の入れ替え）
    fn transpose(&self) -> Vec<Vec<T>>;
}

impl<T: Clone> GridExt<T> for [Vec<T>] {
    fn rotate_right(&self) -> Vec<Vec<T>> {
        if self.is_empty() {
            return vec![];
        }
        let (h, w) = (self.len(), self[0].len());
        (0..w)
            .map(|j| (0..h).rev().map(|i| self[i][j].clone()).collect())
            .collect()
    }

    fn rotate_left(&self) -> Vec<Vec<T>> {
        if self.is_empty() {
            return vec![];
        }
        let (h, w) = (self.len(), self[0].len());
        (0..w)
            .rev()
            .map(|j| (0..h).map(|i| self[i][j].clone()).collect())
            .collect()
    }

    fn rotate_180(&self) -> Vec<Vec<T>> {
        self.iter()
            .rev()
            .map(|v| v.iter().rev().cloned().collect())
            .collect()
    }

    fn transpose(&self) -> Vec<Vec<T>> {
        if self.is_empty() {
            return vec![];
        }
        let (h, w) = (self.len(), self[0].len());
        (0..w)
            .map(|j| (0..h).map(|i| self[i][j].clone()).collect())
            .collect()
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

pub trait ToComma {
    fn comma(&self) -> String;
}
impl ToComma for str {
    fn comma(&self) -> String {
        let s = self.as_bytes();
        if s.is_empty() {
            return "".into();
        }
        let p = s.iter().position(|&c| c == b'.').unwrap_or(s.len());
        let (l, r) = s.split_at(p);
        let (sg, nm) = if l.first() == Some(&b'-') {
            (&l[..1], &l[1..])
        } else {
            (&l[..0], l)
        };
        if nm.len() < 4 {
            return self.into();
        }
        let mut v = Vec::with_capacity(self.len() + (nm.len() - 1) / 3);
        v.extend_from_slice(sg);
        let h = (nm.len() - 1) % 3 + 1;
        v.extend_from_slice(&nm[..h]);
        for c in nm[h..].chunks(3) {
            v.push(b',');
            v.extend_from_slice(c)
        }
        v.extend_from_slice(r);
        unsafe { String::from_utf8_unchecked(v) }
    }
}
impl ToComma for String {
    fn comma(&self) -> String {
        (**self).comma()
    }
}
macro_rules! C{
    (@ $n:expr,$neg:expr)=>{{let(mut n,mut b,mut p)=($n,[0u8;50],50);if n==0{return "0".into()}let mut c=0;while n>0{if c>0&&c%3==0{p-=1;b[p]=b','}p-=1;b[p]=b'0'+(n%10)as u8;n/=10;c+=1}if $neg{p-=1;b[p]=b'-'}unsafe{String::from_utf8_unchecked(b[p..].to_vec())}}};
    (S $($t:ty),*)=>{$(impl ToComma for$t{fn comma(&self)->String{C!(@ self.unsigned_abs()as u128,*self<0)}})*};
    (U $($t:ty),*)=>{$(impl ToComma for$t{fn comma(&self)->String{C!(@ *self as u128,false)}})*};
}
C!(S i8,i16,i32,i64,i128,isize);
C!(U u8,u16,u32,u64,u128,usize);
