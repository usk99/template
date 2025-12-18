use {
    itertools::Itertools,
    num::*,
    std::{cmp::*, collections::*, hash::*, iter::*, ops::*},
};

// ==========================================================
// 1. 基本的なスライス拡張
// ==========================================================
#[rustfmt::skip]
pub trait SliceExt<T> {
    /// 合計値取得
    fn sum_all(&self)       ->                 T where T: Sum;
    /// 積取得
    fn product_all(&self)   ->                 T where T: Product;
    /// 平均値取得
    fn average(&self)       ->               f64 where T: ToPrimitive;
    /// 累積和取得
    fn cumsum(&self)        ->            Vec<T> where T: Default + Add<Output = T>;
    /// ランレングス圧縮
    fn rle(&self)           ->   Vec<(T, usize)> where T: PartialEq;
    /// 要素出現回数カウント
    fn counter(&self)       -> HashMap<T, usize> where T: Hash + Eq;
    /// 回文判定
    fn is_palindrome(&self) ->              bool where T: PartialEq;
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
    fn cumsum(&self) -> Vec<T>
    where
        T: Default + Add<Output = T>,
    {
        let mut acc = T::default();
        std::iter::once(acc.clone())
            .chain(self.iter().map(|x| {
                acc = acc.clone() + x.clone();
                acc.clone()
            }))
            .collect()
    }

    fn rle(&self) -> Vec<(T, usize)>
    where
        T: PartialEq,
    {
        self.iter()
            .cloned()
            .dedup_with_count()
            .map(|(c, v)| (v, c))
            .collect()
    }

    fn counter(&self) -> HashMap<T, usize>
    where
        T: Hash + Eq,
    {
        self.iter().cloned().fold(HashMap::new(), |mut map, v| {
            *map.entry(v).or_insert(0) += 1;
            map
        })
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
}

// ==========================================================
// 2. 順序・比較が必要な拡張 (Ord)
// ==========================================================
pub trait SliceOrdExt<T> {
    /// 最小値取得
    fn min_all(&self) -> Option<T>;
    /// 最大値取得
    fn max_all(&self) -> Option<T>;
    /// インデックスソート取得
    fn index_sort(&self) -> Vec<usize>;
    /// 座標圧縮
    fn compress(&self) -> (Vec<T>, Vec<usize>);
    /// lower_bound取得
    fn lower_bound(&self, x: &T) -> usize;
    /// upper_bound取得
    fn upper_bound(&self, x: &T) -> usize;
    /// xと等しい範囲
    fn equal_range(&self, x: &T) -> Range<usize>;
    /// 次の順列取得
    fn next_permutation(&mut self) -> bool;
    /// 前の順列取得
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
        (0..self.len())
            .sorted_by(|&a, &b| self[a].cmp(&self[b]).then(a.cmp(&b)))
            .collect()
    }
    fn compress(&self) -> (Vec<T>, Vec<usize>) {
        let vals = self.iter().cloned().sorted().dedup().collect::<Vec<_>>();
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
    /// 最大公約数
    fn gcd_all(&self) -> Option<T>;
    /// 最小公倍数
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
    fn counter(&self) -> HashMap<char, usize>;
}
impl StringExt for str {
    fn counter(&self) -> HashMap<char, usize> {
        self.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
    }
}

pub trait CharExt {
    fn to_num(self) -> usize;
}
impl CharExt for char {
    fn to_num(self) -> usize {
        self as usize - '0' as usize
    }
}

pub trait NumExt {
    /// 累乗ショートカット
    fn p(self, exp: Self) -> Self;
    /// 桁ベクタ
    fn digits(self) -> Vec<usize>;
    /// 各桁の和
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
// 必要な型に一括実装
impl_NumExt!(usize, u64, i64, u32, i32, u128, i128);
