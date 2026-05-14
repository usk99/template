use {
    itertools::Itertools,
    num::{Integer, ToPrimitive},
    std::{cmp::*, collections::*, fmt::Display, hash::*, iter::*, ops::*},
};

// ==========================================================
// 1. 基本的なスライス拡張
// ==========================================================
#[rustfmt::skip]
pub trait SliceExt<T> {
    /// 全要素の合計を返す。空なら `T::default()`（通常0）。
    fn sum_all(&self)       ->                 T where T: Sum;
    /// 全要素の積を返す。空なら `T::default()`（通常1）。
    fn product_all(&self)   ->                 T where T: Product;
    /// 全要素の平均を `f64` で返す。空なら `0.0`。
    fn average(&self)       ->               f64 where T: ToPrimitive;
    /// 累積和を返す。長さ `n` のスライスに対して長さ `n+1` の Vec を返す。`result[i]` = `self[0..i]` の和。
    fn prefix_sum(&self)    ->            Vec<T> where T: Default + Copy + AddAssign<T>;
    /// ランレングス圧縮。連続する同じ値をまとめて `(個数, 値)` のリストを返す。
    fn rle(&self)           ->   Vec<(usize, T)> where T: PartialEq;
    /// 各要素の出現回数を `HashMap` で返す。
    fn counter(&self)       -> HashMap<T, usize> where T: Hash + Eq;
    /// 回文かどうかを返す。空スライスは `true`。
    fn is_palindrome(&self) ->              bool where T: PartialEq;
    /// 要素を区切りなしで結合した文字列を返す。
    fn joine(&self)         ->            String where T: Display;
    /// 要素を改行区切りで結合した文字列を返す。
    fn joinl(&self)         ->            String where T: Display;
    /// 要素を空白区切りで結合した文字列を返す。
    fn joins(&self)         ->            String where T: Display;
}
impl<T: Clone> SliceExt<T> for [T] {
    fn sum_all(&self) -> T
    where
        T: Sum,
    {
        self.iter().cloned().sum()
    }
    fn product_all(&self) -> T
    where
        T: Product,
    {
        self.iter().cloned().product()
    }
    fn average(&self) -> f64
    where
        T: ToPrimitive,
    {
        if self.is_empty() {
            0.0
        } else {
            self.iter().map(|x| x.to_f64().unwrap_or(0.0)).sum::<f64>() / (self.len() as f64)
        }
    }
    fn prefix_sum(&self) -> Vec<T>
    where
        T: Default + Copy + AddAssign<T>,
    {
        let mut result = Vec::with_capacity(self.len() + 1);
        let mut acc = T::default();
        result.push(acc);
        for &x in self {
            acc += x;
            result.push(acc);
        }
        result
    }

    fn rle(&self) -> Vec<(usize, T)>
    where
        T: PartialEq,
    {
        self.iter().cloned().dedup_with_count().collect()
    }

    fn counter(&self) -> HashMap<T, usize>
    where
        T: Hash + Eq,
    {
        self.iter().cloned().counts()
    }

    fn is_palindrome(&self) -> bool
    where
        T: PartialEq,
    {
        self.iter()
            .zip(self.iter().rev())
            .take(self.len() / 2)
            .all(|(a, b)| a == b)
    }
    fn joine(&self) -> String
    where
        T: Display,
    {
        self.iter().join("")
    }
    fn joinl(&self) -> String
    where
        T: Display,
    {
        self.iter().join("\n")
    }
    fn joins(&self) -> String
    where
        T: Display,
    {
        self.iter().join(" ")
    }
}

// ==========================================================
// 2. 順序・比較が必要な拡張 (Ord)
// ==========================================================
pub trait SliceOrdExt<T> {
    /// 最小値を返す。空なら `None`。
    fn min_all(&self) -> Option<T>;
    /// 最大値を返す。空なら `None`。
    fn max_all(&self) -> Option<T>;
    /// ソートしたときの元のインデックス列を返す。時間: O(n log n)。
    fn index_sort(&self) -> Vec<usize>;
    /// 座標圧縮。`(重複除去済みのソート済み値列, 各要素の圧縮後インデックス列)` を返す。
    fn compress(&self) -> (Vec<T>, Vec<usize>);
    /// ソート済みスライスで `x` 以上の最初のインデックスを返す。時間: O(log n)。
    fn lower_bound(&self, x: &T) -> usize;
    /// ソート済みスライスで `x` より大きい最初のインデックスを返す。時間: O(log n)。
    fn upper_bound(&self, x: &T) -> usize;
    /// ソート済みスライスで `x` と等しい要素の範囲を `Range` で返す。
    fn equal_range(&self, x: &T) -> Range<usize>;
    /// 次の辞書順順列に並び替える。最後の順列なら `false` を返す。
    fn next_permutation(&mut self) -> bool;
    /// 前の辞書順順列に並び替える。最初の順列なら `false` を返す。
    fn prev_permutation(&mut self) -> bool;
}
impl<T: Ord + Clone> SliceOrdExt<T> for [T] {
    fn min_all(&self) -> Option<T> {
        self.iter().cloned().min()
    }
    fn max_all(&self) -> Option<T> {
        self.iter().cloned().max()
    }
    fn index_sort(&self) -> Vec<usize> {
        (0..self.len()).sorted_by_key(|&i| &self[i]).collect()
    }
    fn compress(&self) -> (Vec<T>, Vec<usize>) {
        let mut vals = self.to_vec();
        vals.sort_unstable();
        vals.dedup();
        let encoded = self
            .iter()
            .map(|x| vals.binary_search(x).unwrap())
            .collect();
        (vals, encoded)
    }
    fn lower_bound(&self, x: &T) -> usize {
        self.partition_point(|v| v < x)
    }
    fn upper_bound(&self, x: &T) -> usize {
        self.partition_point(|v| v <= x)
    }
    fn equal_range(&self, x: &T) -> Range<usize> {
        self.lower_bound(x)..self.upper_bound(x)
    }
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let i = match (0..self.len() - 1).rfind(|&i| self[i] < self[i + 1]) {
            Some(i) => i,
            None => return false,
        };
        let j = (0..self.len()).rfind(|&j| self[i] < self[j]).unwrap();
        self.swap(i, j);
        self[i + 1..].reverse();
        true
    }
    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let i = match (0..self.len() - 1).rfind(|&i| self[i] > self[i + 1]) {
            Some(i) => i,
            None => return false,
        };
        let j = (0..self.len()).rfind(|&j| self[i] > self[j]).unwrap();
        self.swap(i, j);
        self[i + 1..].reverse();
        true
    }
}

// ==========================================================
// 3. 数学的な拡張 (Integer)
// ==========================================================
pub trait SliceIntExt<T> {
    /// 全要素の GCD を返す。空なら `None`。
    fn gcd_all(&self) -> Option<T>;
    /// 全要素の LCM を返す。空なら `None`。
    fn lcm_all(&self) -> Option<T>;
}
impl<T: Integer + Clone> SliceIntExt<T> for [T] {
    fn gcd_all(&self) -> Option<T> {
        self.iter().cloned().reduce(|a, b| a.gcd(&b))
    }
    fn lcm_all(&self) -> Option<T> {
        self.iter().cloned().reduce(|a, b| a.lcm(&b))
    }
}

// ==========================================================
// 4. 文字列拡張 (シンプル化)
// ==========================================================
pub trait StringExt {
    /// 各文字の出現回数を `HashMap` で返す。
    fn counter(&self) -> HashMap<char, usize>;
}
impl StringExt for str {
    fn counter(&self) -> HashMap<char, usize> {
        self.chars().counts()
    }
}

pub trait CharExt {
    /// `'0'`～`'9'` を数値 `0`～`9` に変換する。
    fn to_num(self) -> usize;
    /// `'a'`～`'z'` または `'A'`～`'Z'` を `0`～`25` に変換する。大文字小文字どちらも0始まり。
    fn to_alpha(self) -> usize;
    /// `0`～`25` を対応するアルファベット文字に変換する。`upper=true` で大文字。
    fn from_alpha(i: usize, upper: bool) -> char;
}
impl CharExt for char {
    fn to_num(self) -> usize {
        self as usize - '0' as usize
    }
    fn to_alpha(self) -> usize {
        match self {
            'a'..='z' => self as usize - 'a' as usize,
            'A'..='Z' => self as usize - 'A' as usize,
            _ => panic!(
                "to_alpha: not an ASCII alphabetic char: {:?} (U+{:04X})",
                self, self as u32
            ),
        }
    }
    fn from_alpha(i: usize, upper: bool) -> char {
        (if upper { b'A' } else { b'a' } + i as u8) as char
    }
}

pub trait NumExt {
    /// `self` の `exp` 乗を返す。
    fn p(self, exp: Self) -> Self;
    /// 10進数の各桁を上位桁から順に `Vec<usize>` で返す。`0` なら `[0]`。
    fn digits(self) -> Vec<usize>;
    /// 10進数の各桁の和を返す。
    fn digit_sum(self) -> usize;
}
macro_rules! impl_NumExt {
    ($($t:ty),*) => {
        $(
            impl NumExt for $t {
                fn p(self, exp: $t) -> $t {
                    self.pow(exp as u32)
                }

                fn digits(self) -> Vec<usize> {
                    if self == 0 { return vec![0]; }
                    let mut n = self;
                    let mut res = Vec::new();
                    while n > 0 {
                        res.push((n % 10) as usize);
                        n /= 10;
                    }
                    res.reverse();
                    res
                }

                fn digit_sum(self) -> usize {
                    let mut n = self;
                    let mut sum = 0;
                    while n > 0 {
                        sum += (n % 10) as usize;
                        n /= 10;
                    }
                    sum
                }
            }
        )*
    };
}
// 符号なし型に一括実装
impl_NumExt!(u8, u16, u32, u64, u128, usize);

// 符号付き型用（unsigned_abs で絶対値を取る）
macro_rules! impl_NumExt_signed {
    ($($t:ty),*) => {
        $(
            impl NumExt for $t {
                fn p(self, exp: $t) -> $t {
                    self.pow(exp as u32)
                }

                fn digits(self) -> Vec<usize> {
                    if self == 0 { return vec![0]; }
                    let mut n = self.unsigned_abs();
                    let mut res = Vec::new();
                    while n > 0 {
                        res.push((n % 10) as usize);
                        n /= 10;
                    }
                    res.reverse();
                    res
                }

                fn digit_sum(self) -> usize {
                    let mut n = self.unsigned_abs();
                    let mut sum = 0;
                    while n > 0 {
                        sum += (n % 10) as usize;
                        n /= 10;
                    }
                    sum
                }
            }
        )*
    };
}
impl_NumExt_signed!(i8, i16, i32, i64, i128, isize);
