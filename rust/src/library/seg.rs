//! セグメント木（Segment Tree）実装
//!
//! ## 2つの実装
//! - `LazySegTree<M>`: 遅延評価あり。Sumモノイドでadd_rangeがO(log n)で高速だが、
//!   Min/Maxでは正しく動作しない
//! - `SegTree<M>`: 遅延評価なし。すべてのモノイドでadd_rangeが正しく動作するが、
//!   add_range/update_rangeはO(n)
//!
//! ## 使い分け
//! - **高速な区間加算が必要（Sum）**: `LazySegTree`を使用
//! - **Min/Maxで区間加算が必要**: `SegTree`を使用
//! - **区間加算が不要**: どちらでも可（`LazySegTree`を推奨）

use num_integer::*;
use std::fmt::*;

#[derive(Clone)]
struct RangeOperationParams<S> {
    left: usize,
    right: usize,
    x: S,
    k: usize,
    l: usize,
    r: usize,
    is_add: bool,
}

pub trait Monoid {
    type S: Clone;

    /// 単位元を返す
    fn identity() -> Self::S;

    /// 二項演算（結合則を満たす必要がある）
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;

    /// 遅延評価で値を区間長に応じて適用（update_range用）
    fn apply_lazy_with_length(value: Self::S, length: usize) -> Self::S;

    /// 加算値を区間長に応じて適用（add_range用）
    fn apply_add_with_length(current: Self::S, add_value: Self::S, length: usize) -> Self::S;

    /// 加算の単位元を返す
    fn add_identity() -> Self::S;

    /// `LazySegTree`でadd_rangeが正しく動作するかを示す
    /// （Sum以外はfalseを返し、警告が表示される）
    fn supports_add_range() -> bool {
        false
    }
}

/// 遅延評価セグメント木（Lazy Segment Tree）
///
/// 区間更新と区間取得をO(log n)で行えるデータ構造。
/// **注意**: `add_range`はSumモノイドでのみ正しく動作します。
///          Min/Maxで区間加算が必要な場合は`SegTree`を使用。
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LazySegTree<M: Monoid> {
    n: usize,                // 元配列のサイズ
    size: usize,             // 内部配列のサイズ（2の累乗）
    log: usize,              // log2(size)
    data: Vec<M::S>,         // セグメント木のデータ
    lazy: Vec<Option<M::S>>, // 遅延更新用（update_range）
    lazy_add: Vec<M::S>,     // 遅延加算用（add_range）
    len: Vec<usize>,         // 各ノードが担当する区間の長さ
}

impl<M: Monoid> LazySegTree<M> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let mut len = vec![0; size * 2];
        len[size..size * 2].fill(1);
        for i in (1..size).rev() {
            len[i] = len[i * 2] + len[i * 2 + 1];
        }
        Self {
            n,
            size,
            log: size.trailing_zeros() as usize,
            data: vec![M::identity(); size * 2],
            lazy: vec![None; size * 2],
            lazy_add: vec![M::add_identity(); size * 2],
            len,
        }
    }

    /// ベクタからセグメント木を作成
    pub fn from_vec(v: &[M::S]) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let log = size.trailing_zeros() as usize;
        let mut data = vec![M::identity(); size * 2];
        data[size..size + n].clone_from_slice(v);
        for i in (1..size).rev() {
            data[i] = M::binary_operation(&data[i * 2], &data[i * 2 + 1]);
        }
        let mut len = vec![0; size * 2];
        len[size..size * 2].fill(1);
        for i in (1..size).rev() {
            len[i] = len[i * 2] + len[i * 2 + 1];
        }
        Self {
            n,
            size,
            log,
            data,
            lazy: vec![None; size * 2],
            lazy_add: vec![M::add_identity(); size * 2],
            len,
        }
    }

    /// 元配列のサイズを取得
    pub const fn len(&self) -> usize {
        self.n
    }

    /// 空かどうかを判定
    pub const fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// 遅延評価を伝搬させる（内部処理用）
    ///
    /// ノードkの遅延情報を子ノードに伝搬し、自ノードのデータを更新する。
    /// 更新（lazy）と加算（lazy_add）の両方を処理し、更新が優先される。
    fn propagate(&mut self, k: usize) {
        // 更新が優先（加算をクリア）
        if let Some(x) = self.lazy[k].take() {
            if k < self.size {
                self.lazy[k * 2] = Some(x.clone());
                self.lazy[k * 2 + 1] = Some(x.clone());
                // 更新時は子の加算をクリア
                self.lazy_add[k * 2] = M::add_identity();
                self.lazy_add[k * 2 + 1] = M::add_identity();
            }
            self.data[k] = M::apply_lazy_with_length(x, self.len[k]);
            // 更新時は自身の加算もクリア
            self.lazy_add[k] = M::add_identity();
            return;
        }

        // 加算の処理（常に適用、add_identityなら影響なし）
        let add_val = self.lazy_add[k].clone();
        if k < self.size {
            self.lazy_add[k * 2] = M::binary_operation(&self.lazy_add[k * 2], &add_val);
            self.lazy_add[k * 2 + 1] = M::binary_operation(&self.lazy_add[k * 2 + 1], &add_val);
        }
        self.data[k] = M::apply_add_with_length(self.data[k].clone(), add_val, self.len[k]);
        self.lazy_add[k] = M::add_identity();
    }

    /// インデックスkの要素をxに更新
    pub fn update(&mut self, k: usize, x: M::S) {
        self.update_range(k, k + 1, x);
    }

    /// 区間[left, right)の全要素をxに更新
    pub fn update_range(&mut self, left: usize, right: usize, x: M::S) {
        self.range_operation_recursive(RangeOperationParams {
            left,
            right,
            x,
            k: 1,
            l: 0,
            r: self.size,
            is_add: false,
        });
    }

    fn range_operation_recursive(&mut self, params: RangeOperationParams<M::S>) {
        self.propagate(params.k);
        if params.right <= params.l || params.r <= params.left {
            return;
        }
        if params.left <= params.l && params.r <= params.right {
            self.lazy[params.k] = Some(params.x.clone());
            self.propagate(params.k);
            return;
        }
        let mid = (params.l + params.r) / 2;
        self.range_operation_recursive(RangeOperationParams {
            left: params.left,
            right: params.right,
            x: params.x.clone(),
            k: params.k * 2,
            l: params.l,
            r: mid,
            is_add: params.is_add,
        });
        self.range_operation_recursive(RangeOperationParams {
            left: params.left,
            right: params.right,
            x: params.x.clone(),
            k: params.k * 2 + 1,
            l: mid,
            r: params.r,
            is_add: params.is_add,
        });
        self.update_operation(params.k);
    }

    fn update_operation(&mut self, k: usize) {
        self.data[k] = M::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
    }

    /// 区間加算
    pub fn add_range(&mut self, left: usize, right: usize, x: M::S) {
        if !M::supports_add_range() {
            eprintln!(
                "警告: このモノイドではadd_rangeは正しく動作しません。Sumモノイドのみ対応しています。"
            );
        }
        self.add_range_recursive(left, right, x, 1, 0, self.size);
    }

    fn add_range_recursive(
        &mut self,
        left: usize,
        right: usize,
        x: M::S,
        k: usize,
        l: usize,
        r: usize,
    ) {
        self.propagate(k);
        if right <= l || r <= left {
            return;
        }
        if left <= l && r <= right {
            self.lazy_add[k] = M::binary_operation(&self.lazy_add[k], &x);
            self.propagate(k);
            return;
        }
        let mid = (l + r) / 2;
        self.add_range_recursive(left, right, x.clone(), k * 2, l, mid);
        self.add_range_recursive(left, right, x, k * 2 + 1, mid, r);
        self.update_operation(k);
    }

    /// 単一加算
    pub fn add(&mut self, k: usize, x: M::S) {
        self.add_range(k, k + 1, x);
    }

    /// 区間取得
    pub fn get_range(&mut self, left: usize, right: usize) -> M::S {
        self.get_range_recursive(left, right, 1, 0, self.size)
    }
    fn get_range_recursive(
        &mut self,
        left: usize,
        right: usize,
        k: usize,
        l: usize,
        r: usize,
    ) -> M::S {
        self.propagate(k);
        if right <= l || r <= left {
            return M::identity();
        }
        if left <= l && r <= right {
            return self.data[k].clone();
        }
        let mid = (l + r) / 2;
        let vl = self.get_range_recursive(left, right, k * 2, l, mid);
        let vr = self.get_range_recursive(left, right, k * 2 + 1, mid, r);
        M::binary_operation(&vl, &vr)
    }
    /// 単一取得
    pub fn get(&mut self, k: usize) -> M::S {
        self.get_range(k, k + 1)
    }
    /// 全要素取得
    pub fn to_vec(&mut self) -> Vec<M::S> {
        (0..self.n).map(|i| self.get(i)).collect()
    }
    /// 全体結果取得
    pub fn get_all(&mut self) -> M::S {
        self.get_range(0, self.n)
    }
    /// リセット
    pub fn clear(&mut self) {
        self.data.fill(M::identity());
        self.lazy.fill(None);
    }

    /// デバッグ用：セグメント木の内部状態を表示
    ///
    /// 木構造を視覚的に表示し、各ノードのデータ、遅延情報、区間長を出力します。
    pub fn debug_print(&mut self)
    where
        M::S: Debug,
    {
        println!("=== LazySegTree Debug Info ===");
        println!("n={}, size={}, log={}", self.n, self.size, self.log);
        println!("\n[葉ノードの値]");
        for i in 0..self.n {
            print!("{:?} ", self.get(i));
        }
        println!("\n\n[木構造]");

        // 最大インデックスの桁数を計算
        let max_index_width = (self.size * 2 - 1).to_string().len();

        let mut level = 0;
        let mut start = 1;
        while start < self.size * 2 {
            let end = (start * 2).min(self.size * 2);
            println!("レベル {level:max_index_width$}: ノード[{start}..{end})");
            for i in start..end {
                if i >= self.size * 2 {
                    break;
                }
                self.propagate(i);
                print!("  [{i:max_index_width$}] data={:?}", self.data[i]);
                if let Some(ref lazy_val) = self.lazy[i] {
                    print!(", lazy={:?}", lazy_val);
                }
                print!(", len={}", self.len[i]);
                println!();
            }
            println!();
            start = end;
            level += 1;
        }
        println!("==============================\n");
    }
}
// 和の実装
#[derive(Clone, Copy)]
pub struct Sum;
impl Monoid for Sum {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
    }
    fn apply_lazy_with_length(value: Self::S, length: usize) -> Self::S {
        value * length
    }
    fn apply_add_with_length(current: Self::S, add_value: Self::S, length: usize) -> Self::S {
        current + add_value * length
    }
    fn add_identity() -> Self::S {
        0
    }
    fn supports_add_range() -> bool {
        true
    }
}
// 最小値の実装
#[derive(Clone, Copy)]
pub struct Min;
impl Monoid for Min {
    type S = usize;
    fn identity() -> Self::S {
        usize::MAX
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
    fn apply_lazy_with_length(value: Self::S, _length: usize) -> Self::S {
        value
    }
    fn apply_add_with_length(current: Self::S, add_value: Self::S, _length: usize) -> Self::S {
        current.saturating_add(add_value)
    }
    fn add_identity() -> Self::S {
        0
    }
}
// 最大値の実装
#[derive(Clone, Copy)]
pub struct Max;
impl Monoid for Max {
    type S = usize;
    fn identity() -> Self::S {
        usize::MIN
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
    fn apply_lazy_with_length(value: Self::S, _length: usize) -> Self::S {
        value
    }
    fn apply_add_with_length(current: Self::S, add_value: Self::S, _length: usize) -> Self::S {
        current.saturating_add(add_value)
    }
    fn add_identity() -> Self::S {
        0
    }
}
// XORの実装
#[derive(Clone, Copy)]
pub struct Xor;
impl Monoid for Xor {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
    fn apply_lazy_with_length(value: Self::S, _length: usize) -> Self::S {
        value
    }
    fn apply_add_with_length(current: Self::S, _add_value: Self::S, _length: usize) -> Self::S {
        current // XORは加算非対応
    }
    fn add_identity() -> Self::S {
        0
    }
}
// gcdの実装
#[derive(Clone, Copy)]
pub struct Gcd;
impl Monoid for Gcd {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a.gcd(b)
    }
    fn apply_lazy_with_length(value: Self::S, _length: usize) -> Self::S {
        value
    }
    fn apply_add_with_length(current: Self::S, _add_value: Self::S, _length: usize) -> Self::S {
        current // GCDは加算非対応
    }
    fn add_identity() -> Self::S {
        0
    }
}
// lcmの実装
#[derive(Clone, Copy)]
pub struct Lcm;
impl Monoid for Lcm {
    type S = usize;
    fn identity() -> Self::S {
        1
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a.lcm(b)
    }
    fn apply_lazy_with_length(value: Self::S, _length: usize) -> Self::S {
        value
    }
    fn apply_add_with_length(current: Self::S, _add_value: Self::S, _length: usize) -> Self::S {
        current // LCMは加算非対応
    }
    fn add_identity() -> Self::S {
        0
    }
}

impl LazySegTree<Sum> {
    pub fn new_sum(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_sum(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl LazySegTree<Min> {
    pub fn new_min(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_min(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl LazySegTree<Max> {
    pub fn new_max(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_max(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl LazySegTree<Xor> {
    pub fn new_xor(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_xor(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl LazySegTree<Gcd> {
    pub fn new_gcd(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_gcd(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl LazySegTree<Lcm> {
    pub fn new_lcm(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_lcm(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}

/// 通常版セグメント木（遅延評価なし）
///
/// 遅延評価を行わないシンプルなセグメント木実装。
/// **すべてのモノイド（Sum, Min, Max等）で`add_range`が正しく動作します**。
///
/// 遅延評価がないため、区間更新・区間加算はO(n)ですが、
/// Min/Maxで区間加算が必要な場合はこちらを使用してください。
/// - `get_range`: O(log n)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SegTree<M: Monoid> {
    n: usize,        // 元配列のサイズ
    size: usize,     // 内部配列のサイズ（2の累乗）
    data: Vec<M::S>, // セグメント木のデータ
}

impl<M: Monoid> SegTree<M> {
    /// 新しいセグメント木を作成（全要素が単位元で初期化）
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        Self {
            n,
            size,
            data: vec![M::identity(); size * 2],
        }
    }

    /// ベクタからセグメント木を作成
    pub fn from_vec(v: &[usize]) -> Self
    where
        M::S: From<usize>,
    {
        let n = v.len();
        let size = n.next_power_of_two();
        let mut data = vec![M::identity(); size * 2];
        for i in 0..n {
            data[size + i] = M::S::from(v[i]);
        }
        for i in (1..size).rev() {
            data[i] = M::binary_operation(&data[i * 2], &data[i * 2 + 1]);
        }
        Self { n, size, data }
    }

    /// インデックスindexの要素をvalueに設定
    pub fn set(&mut self, mut index: usize, value: M::S) {
        index += self.size;
        self.data[index] = value;
        while index > 1 {
            index /= 2;
            self.data[index] =
                M::binary_operation(&self.data[index * 2], &self.data[index * 2 + 1]);
        }
    }

    /// インデックスindexの要素を取得
    pub fn get(&self, index: usize) -> M::S {
        self.data[self.size + index].clone()
    }

    /// 区間[left, right)の全要素をvalueに更新
    pub fn update_range(&mut self, left: usize, right: usize, value: M::S) {
        for i in left..right.min(self.n) {
            self.set(i, value.clone());
        }
    }

    /// 区間[left, right)の全要素にadd_valueを加算
    pub fn add_range(&mut self, left: usize, right: usize, add_value: M::S) {
        for i in left..right.min(self.n) {
            let current = self.get(i);
            let new_value = M::apply_add_with_length(current, add_value.clone(), 1);
            self.set(i, new_value);
        }
    }

    /// 区間[left, right)の集約値を取得
    pub fn get_range(&self, left: usize, right: usize) -> M::S {
        let mut result_left = M::identity();
        let mut result_right = M::identity();
        let mut l = left + self.size;
        let mut r = right + self.size;

        while l < r {
            if l % 2 == 1 {
                result_left = M::binary_operation(&result_left, &self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                result_right = M::binary_operation(&self.data[r], &result_right);
            }
            l /= 2;
            r /= 2;
        }
        M::binary_operation(&result_left, &result_right)
    }

    /// 全要素をベクタとして取得
    pub fn to_vec(&self) -> Vec<M::S> {
        self.data[self.size..self.size + self.n].to_vec()
    }

    /// デバッグ用：セグメント木の内部状態を表示
    ///
    /// 木構造を視覚的に表示し、各ノードのデータを出力します。
    pub fn debug_print(&self)
    where
        M::S: Debug,
    {
        println!("=== SegTree Debug Info ===");
        println!("n={}, size={}", self.n, self.size);
        println!("\n[葉ノードの値]");
        for i in 0..self.n {
            print!("{:?} ", self.get(i));
        }
        println!("\n\n[木構造]");

        // 最大インデックスの桁数を計算
        let max_index_width = (self.size * 2 - 1).to_string().len();

        let mut level = 0;
        let mut start = 1;
        while start < self.size * 2 {
            let end = (start * 2).min(self.size * 2);
            println!("レベル {level:max_index_width$}: ノード[{start}..{end})");
            for i in start..end {
                if i >= self.size * 2 {
                    break;
                }
                print!("  [{i:max_index_width$}] data={:?}", self.data[i]);
                println!();
            }
            println!();
            start = end;
            level += 1;
        }
        println!("==========================\n");
    }
}

// ===== コンストラクタヘルパー =====
// 各モノイド用の便利なコンストラクタ

impl SegTree<Sum> {
    pub fn new_sum(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_sum(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl SegTree<Min> {
    pub fn new_min(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_min(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl SegTree<Max> {
    pub fn new_max(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_max(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl SegTree<Xor> {
    pub fn new_xor(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_xor(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl SegTree<Gcd> {
    pub fn new_gcd(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_gcd(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}
impl SegTree<Lcm> {
    pub fn new_lcm(n: usize) -> Self {
        Self::new(n)
    }
    pub fn from_vec_lcm(v: &[usize]) -> Self {
        Self::from_vec(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let mut seg = LazySegTree::<Min>::from_vec_min(&[10, 20, 30]);
        seg.add_range(0, 2, 100);
        println!("{:?}", seg.to_vec());
        println!()
    }

    #[test]
    fn test_sum_operations() {
        println!("=== 和（Sum）の操作テスト ===");

        // 配列の長さは10
        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        println!("初期配列: {:?}", seg.to_vec());

        // 初期状態の確認
        println!("初期状態の全体の和を確認");
        let initial_sum = seg.get_range(0, 10);
        println!("  結果: {}", initial_sum);
        assert_eq!(initial_sum, 55, "初期状態の和は55のはず");

        // 単一更新のテスト
        println!("インデックス5を100に更新");
        seg.update(5, 100);
        println!("  更新後の配列: {:?}", seg.to_vec());
        let sum_after_single_update = seg.get_range(0, 10);
        println!("  更新後の和: {}", sum_after_single_update);
        assert_eq!(
            sum_after_single_update, 149,
            "1+2+3+4+5+100+7+8+9+10=149のはず"
        );

        // 区間更新のテスト
        println!("区間[2, 7)を20に更新");
        seg.update_range(2, 7, 20);
        let array_after_range_update = seg.to_vec();
        println!("  更新後の配列: {:?}", array_after_range_update);
        assert_eq!(
            array_after_range_update,
            vec![1, 2, 20, 20, 20, 20, 20, 8, 9, 10],
            "区間更新の結果が正しいこと"
        );

        let sum_after_range_update = seg.get_range(0, 10);
        println!("  更新後の和: {}", sum_after_range_update);
        assert_eq!(sum_after_range_update, 130, "1+2+20*5+8+9+10=130のはず");

        // 部分区間の取得テスト
        println!("区間[3, 8)の和を取得");
        let partial_sum = seg.get_range(3, 8);
        println!("  結果: {}", partial_sum);
        assert_eq!(partial_sum, 88, "20*4+8=88のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_min_operations() {
        println!("=== 最小値（Min）の操作テスト ===");

        let mut seg = LazySegTree::<Min>::from_vec_min(&[50, 30, 70, 20, 90, 10, 40, 60, 80, 25]);

        println!("初期状態の最小値を確認");
        let initial_min = seg.get_range(0, 10);
        println!("  結果: {}", initial_min);
        assert_eq!(initial_min, 10, "初期状態の最小値は10のはず");

        println!("インデックス5を5に更新");
        seg.update(5, 5);
        let min_after_update = seg.get_range(0, 10);
        println!("  更新後の最小値: {}", min_after_update);
        assert_eq!(min_after_update, 5, "最小値は5になるはず");

        println!("区間[0, 5)を100に更新");
        seg.update_range(0, 5, 100);
        let min_after_range_update = seg.get_range(0, 10);
        println!("  更新後の最小値: {}", min_after_range_update);
        assert_eq!(min_after_range_update, 5, "最小値は5のまま");

        let partial_min = seg.get_range(0, 4);
        println!("区間[0, 4)の最小値: {}", partial_min);
        assert_eq!(partial_min, 100, "区間[0, 4)は全て100なので最小値は100");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_lazy_min_add_range_warning() {
        println!("\n=== 遅延版でMinのadd_range警告テスト ===");

        let mut seg = LazySegTree::<Min>::from_vec_min(&[5, 3, 7, 1, 6, 4, 8, 2, 9, 0]);
        println!("初期配列: {:?}", seg.to_vec());

        println!("\n区間[2, 7)に10を加算（警告が表示されるはず）");
        seg.add_range(2, 7, 10);

        println!("=== テスト完了（警告表示を確認してください） ===\n");
    }

    #[test]
    fn test_max_operations() {
        println!("=== 最大値（Max）の操作テスト ===");

        let mut seg = LazySegTree::<Max>::from_vec_max(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

        println!("初期状態の最大値を確認");
        let initial_max = seg.get_range(0, 10);
        println!("  結果: {}", initial_max);
        assert_eq!(initial_max, 100, "初期状態の最大値は100のはず");

        println!("区間[7, 10)を5に更新");
        seg.update_range(7, 10, 5);
        let max_after_update = seg.get_range(0, 10);
        println!("  更新後の最大値: {}", max_after_update);
        assert_eq!(max_after_update, 70, "最大値は70になるはず");

        let array_result = seg.to_vec();
        println!("  更新後の配列: {:?}", array_result);
        assert_eq!(
            array_result,
            vec![10, 20, 30, 40, 50, 60, 70, 5, 5, 5],
            "配列の内容が正しいこと"
        );

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_complex_scenario() {
        println!("=== 複雑なシナリオのテスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);

        println!("初期状態（全て5）の和");
        let initial = seg.get_range(0, 10);
        println!("  結果: {}", initial);
        assert_eq!(initial, 50, "5×10=50のはず");

        println!("区間[0, 5)を10に更新");
        seg.update_range(0, 5, 10);
        let after_first_update = seg.get_range(0, 10);
        println!("  更新後の和: {}", after_first_update);
        assert_eq!(after_first_update, 75, "10×5+5×5=75のはず");

        println!("区間[3, 8)を20に更新");
        seg.update_range(3, 8, 20);
        let after_second_update = seg.get_range(0, 10);
        println!("  更新後の和: {}", after_second_update);
        assert_eq!(after_second_update, 140, "10×3+20×5+5×2=140のはず");

        let final_array = seg.to_vec();
        println!("  最終的な配列: {:?}", final_array);
        assert_eq!(
            final_array,
            vec![10, 10, 10, 20, 20, 20, 20, 20, 5, 5],
            "配列の内容が正しいこと"
        );

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_overlapping_range_updates() {
        println!("=== 重複する区間更新のテスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        println!("初期配列: {:?}", seg.to_vec());

        println!("区間[0, 6)を10に更新");
        seg.update_range(0, 6, 10);
        let array_1 = seg.to_vec();
        println!("  配列: {:?}", array_1);
        let sum_1 = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_1);
        assert_eq!(sum_1, 64, "10×6+1×4=64のはず");

        println!("区間[4, 10)を5に更新");
        seg.update_range(4, 10, 5);
        let array_2 = seg.to_vec();
        println!("  配列: {:?}", array_2);
        let sum_2 = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_2);
        assert_eq!(sum_2, 70, "10×4+5×6=70のはず");

        println!("区間[2, 8)を3に更新");
        seg.update_range(2, 8, 3);
        let array_3 = seg.to_vec();
        println!("  配列: {:?}", array_3);
        let sum_3 = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_3);
        assert_eq!(sum_3, 48, "10×2+3×6+5×2=48のはず");

        println!("複数の部分区間を取得");
        let range_0_3 = seg.get_range(0, 3);
        println!("  区間[0, 3)の和: {}", range_0_3);
        assert_eq!(range_0_3, 23, "10+10+3=23のはず");

        let range_5_9 = seg.get_range(5, 9);
        println!("  区間[5, 9)の和: {}", range_5_9);
        assert_eq!(range_5_9, 14, "3×3+5=14のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_single_element_operations() {
        println!("=== 単一要素の連続更新テスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
        println!("初期配列: {:?}", seg.to_vec());

        println!("インデックス3を連続で更新");
        seg.update(3, 100);
        let value_1 = seg.get(3);
        println!("  1回目更新後の値: {}", value_1);
        assert_eq!(value_1, 100, "100になるはず");

        seg.update(3, 200);
        let value_2 = seg.get(3);
        println!("  2回目更新後の値: {}", value_2);
        assert_eq!(value_2, 200, "200になるはず");

        seg.update(3, 5);
        let value_3 = seg.get(3);
        println!("  3回目更新後の値: {}", value_3);
        assert_eq!(value_3, 5, "5になるはず");

        let total = seg.get_range(0, 10);
        println!("  全体の和: {}", total);
        assert_eq!(total, 515, "10+20+30+5+50+60+70+80+90+100=515のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_mixed_monoid_operations() {
        println!("=== 異なるモノイドの複合テスト ===");

        // Sum（和）
        println!("[Sum] 初期配列: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]");
        let mut seg_sum = LazySegTree::<Sum>::from_vec_sum(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
        seg_sum.update_range(3, 7, 5);
        let sum_result = seg_sum.get_range(0, 10);
        println!("  区間[3, 7)を5に更新後の全体の和: {}", sum_result);
        assert_eq!(sum_result, 86, "2+4+6+5×4+14+16+18+20=86のはず");

        // Min（最小値）
        println!("[Min] 初期配列: [50, 30, 70, 20, 90, 10, 40, 60, 80, 25]");
        let mut seg_min =
            LazySegTree::<Min>::from_vec_min(&[50, 30, 70, 20, 90, 10, 40, 60, 80, 25]);
        seg_min.update_range(0, 5, 100);
        let min_result_1 = seg_min.get_range(0, 10);
        println!("  区間[0, 5)を100に更新後の最小値: {}", min_result_1);
        assert_eq!(min_result_1, 10, "最小値は10のまま");

        seg_min.update_range(5, 10, 200);
        let min_result_2 = seg_min.get_range(0, 10);
        println!("  区間[5, 10)を200に更新後の最小値: {}", min_result_2);
        assert_eq!(min_result_2, 100, "最小値は100になる");

        // Max（最大値）
        println!("[Max] 初期配列: [5, 15, 25, 35, 45, 55, 65, 75, 85, 95]");
        let mut seg_max =
            LazySegTree::<Max>::from_vec_max(&[5, 15, 25, 35, 45, 55, 65, 75, 85, 95]);
        seg_max.update_range(7, 10, 10);
        let max_result = seg_max.get_range(0, 10);
        println!("  区間[7, 10)を10に更新後の最大値: {}", max_result);
        assert_eq!(max_result, 65, "最大値は65になる");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_edge_cases() {
        println!("=== エッジケースのテスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        println!("単一要素の区間取得");
        let single = seg.get_range(5, 6);
        println!("  区間[5, 6)の和: {}", single);
        assert_eq!(single, 6, "インデックス5の値は6");

        println!("最初の要素のみ更新");
        seg.update_range(0, 1, 100);
        let first = seg.get(0);
        println!("  インデックス0の値: {}", first);
        assert_eq!(first, 100, "100になるはず");

        println!("最後の要素のみ更新");
        seg.update_range(9, 10, 200);
        let last = seg.get(9);
        println!("  インデックス9の値: {}", last);
        assert_eq!(last, 200, "200になるはず");

        println!("全体を同じ値に更新");
        seg.update_range(0, 10, 7);
        let all_same = seg.to_vec();
        println!("  全体を7に更新: {:?}", all_same);
        assert_eq!(all_same, vec![7; 10], "全て7になるはず");
        let sum_all = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_all);
        assert_eq!(sum_all, 70, "7×10=70のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_nested_range_queries() {
        println!("=== ネストした区間クエリのテスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
        println!("初期配列: {:?}", seg.to_vec());

        println!("大きな区間[1, 9)を5に更新");
        seg.update_range(1, 9, 5);
        let array_after = seg.to_vec();
        println!("  配列: {:?}", array_after);

        println!("様々な部分区間の和を取得");
        let sum_1_3 = seg.get_range(1, 3);
        println!("  区間[1, 3)の和: {}", sum_1_3);
        assert_eq!(sum_1_3, 10, "5×2=10のはず");

        let sum_3_6 = seg.get_range(3, 6);
        println!("  区間[3, 6)の和: {}", sum_3_6);
        assert_eq!(sum_3_6, 15, "5×3=15のはず");

        let sum_6_9 = seg.get_range(6, 9);
        println!("  区間[6, 9)の和: {}", sum_6_9);
        assert_eq!(sum_6_9, 15, "5×3=15のはず");

        let sum_2_8 = seg.get_range(2, 8);
        println!("  区間[2, 8)の和: {}", sum_2_8);
        assert_eq!(sum_2_8, 30, "5×6=30のはず");

        println!("小さな区間[4, 6)を100に更新");
        seg.update_range(4, 6, 100);
        let array_final = seg.to_vec();
        println!("  配列: {:?}", array_final);

        let sum_3_7 = seg.get_range(3, 7);
        println!("  区間[3, 7)の和: {}", sum_3_7);
        assert_eq!(sum_3_7, 210, "5+100+100+5=210のはず");

        let sum_all = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_all);
        assert_eq!(sum_all, 340, "10+5×3+100×2+5×3+100=340のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_range_add_basic() {
        println!("=== 区間加算の基本テスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        println!("初期配列: {:?}", seg.to_vec());

        println!("区間[2, 7)に10を加算");
        seg.add_range(2, 7, 10);
        let array_after_add = seg.to_vec();
        println!("  配列: {:?}", array_after_add);
        assert_eq!(
            array_after_add,
            vec![1, 2, 13, 14, 15, 16, 17, 8, 9, 10],
            "各要素に10が加算されるはず"
        );

        let sum_after_add = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_after_add);
        assert_eq!(sum_after_add, 105, "55+10×5=105のはず");

        println!("区間[0, 5)に3を加算");
        seg.add_range(0, 5, 3);
        let array_after_add2 = seg.to_vec();
        println!("  配列: {:?}", array_after_add2);
        assert_eq!(
            array_after_add2,
            vec![4, 5, 16, 17, 18, 16, 17, 8, 9, 10],
            "さらに加算されるはず"
        );

        let sum_after_add2 = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_after_add2);
        assert_eq!(sum_after_add2, 120, "105+3×5=120のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_range_add_and_update_mixed() {
        println!("=== 区間加算と区間更新の混合テスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
        println!("初期配列（全て5）: {:?}", seg.to_vec());

        println!("区間[2, 8)に10を加算");
        seg.add_range(2, 8, 10);
        let array_1 = seg.to_vec();
        println!("  配列: {:?}", array_1);
        assert_eq!(array_1, vec![5, 5, 15, 15, 15, 15, 15, 15, 5, 5], "加算後");

        println!("区間[4, 7)を100に更新");
        seg.update_range(4, 7, 100);
        let array_2 = seg.to_vec();
        println!("  配列: {:?}", array_2);
        assert_eq!(
            array_2,
            vec![5, 5, 15, 15, 100, 100, 100, 15, 5, 5],
            "更新後（加算は消える）"
        );

        let sum_2 = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_2);
        assert_eq!(sum_2, 365, "5×4+15×3+100×3=365のはず");

        println!("区間[3, 9)に5を加算");
        seg.add_range(3, 9, 5);
        let array_3 = seg.to_vec();
        println!("  配列: {:?}", array_3);
        assert_eq!(
            array_3,
            vec![5, 5, 15, 20, 105, 105, 105, 20, 10, 5],
            "さらに加算"
        );

        let sum_3 = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_3);
        assert_eq!(sum_3, 395, "365+5×6=395のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_single_add_operations() {
        println!("=== 単一要素の加算テスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
        println!("初期配列: {:?}", seg.to_vec());

        println!("インデックス5に15を加算");
        seg.add(5, 15);
        let value_1 = seg.get(5);
        println!("  インデックス5の値: {}", value_1);
        assert_eq!(value_1, 75, "60+15=75のはず");

        println!("インデックス5にさらに25を加算");
        seg.add(5, 25);
        let value_2 = seg.get(5);
        println!("  インデックス5の値: {}", value_2);
        assert_eq!(value_2, 100, "75+25=100のはず");

        let total = seg.get_range(0, 10);
        println!("  全体の和: {}", total);
        assert_eq!(total, 590, "初期550+40=590のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_range_add_overlapping() {
        println!("=== 重複する区間加算のテスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        println!("初期配列（全て0）");

        println!("区間[0, 5)に1を加算");
        seg.add_range(0, 5, 1);
        let array_1 = seg.to_vec();
        println!("  配列: {:?}", array_1);

        println!("区間[3, 8)に2を加算");
        seg.add_range(3, 8, 2);
        let array_2 = seg.to_vec();
        println!("  配列: {:?}", array_2);
        assert_eq!(
            array_2,
            vec![1, 1, 1, 3, 3, 2, 2, 2, 0, 0],
            "重複部分は3になる"
        );

        println!("区間[2, 6)に4を加算");
        seg.add_range(2, 6, 4);
        let array_3 = seg.to_vec();
        println!("  配列: {:?}", array_3);
        assert_eq!(
            array_3,
            vec![1, 1, 5, 7, 7, 6, 2, 2, 0, 0],
            "さらに重複して加算"
        );

        let sum = seg.get_range(0, 10);
        println!("  全体の和: {}", sum);
        assert_eq!(sum, 31, "1+1+5+7+7+6+2+2=31のはず");

        println!("=== テスト成功 ===\n");
    }

    #[test]
    fn test_range_add_comprehensive() {
        println!("=== 総合的な区間加算テスト（Sum専用） ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        println!("初期配列: {:?}", seg.to_vec());

        println!("複数の操作を組み合わせる");
        seg.add_range(0, 5, 10); // [11, 12, 13, 14, 15, 6, 7, 8, 9, 10]
        seg.update_range(7, 10, 100); // [11, 12, 13, 14, 15, 6, 7, 100, 100, 100]
        seg.add_range(5, 10, 5); // [11, 12, 13, 14, 15, 11, 12, 105, 105, 105]

        let final_array = seg.to_vec();
        println!("  最終配列: {:?}", final_array);
        assert_eq!(
            final_array,
            vec![11, 12, 13, 14, 15, 11, 12, 105, 100, 100],
            "複雑な操作の結果"
        );

        let total = seg.get_range(0, 10);
        println!("  全体の和: {}", total);
        assert_eq!(total, 403, "合計は403のはず");

        println!("=== テスト成功 ===\n");
    }

    // 通常版のテスト群
    #[test]
    fn test_segtree_min_add_range() {
        println!("\n=== 通常版セグメント木: 最小値（Min）のadd_rangeテスト ===");

        let v = vec![5, 3, 7, 1, 6, 4, 8, 2, 9, 0];
        let mut seg = SegTree::<Min>::from_vec_min(&v);
        println!("初期配列: {:?}", seg.to_vec());

        let min_all = seg.get_range(0, 10);
        println!("全体の最小値: {}", min_all);
        assert_eq!(min_all, 0, "初期の最小値は0");

        println!("\n区間[2, 7)に10を加算");
        seg.add_range(2, 7, 10);
        let array = seg.to_vec();
        println!("  更新後の配列: {:?}", array);
        assert_eq!(array, vec![5, 3, 17, 11, 16, 14, 18, 2, 9, 0]);

        let min_range = seg.get_range(2, 7);
        println!("  区間[2, 7)の最小値: {}", min_range);
        assert_eq!(min_range, 11, "加算後の区間最小値は11");
        let min_all = seg.get_range(0, 10);
        println!("  全体の最小値: {}", min_all);
        assert_eq!(min_all, 0, "全体の最小値は0");

        println!("\n=== テスト成功 ===\n");
    }

    #[test]
    fn test_segtree_max_add_range() {
        println!("\n=== 通常版セグメント木: 最大値（Max）のadd_rangeテスト ===");

        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut seg = SegTree::<Max>::from_vec_max(&v);
        println!("初期配列: {:?}", seg.to_vec());

        let max_all = seg.get_range(0, 10);
        println!("全体の最大値: {}", max_all);
        assert_eq!(max_all, 10, "初期の最大値は10");

        println!("\n区間[3, 8)に5を加算");
        seg.add_range(3, 8, 5);
        let array = seg.to_vec();
        println!("  更新後の配列: {:?}", array);
        assert_eq!(array, vec![1, 2, 3, 9, 10, 11, 12, 13, 9, 10]);

        let max_range = seg.get_range(3, 8);
        println!("  区間[3, 8)の最大値: {}", max_range);
        assert_eq!(max_range, 13, "加算後の区間最大値は13");

        let max_all = seg.get_range(0, 10);
        println!("  全体の最大値: {}", max_all);
        assert_eq!(max_all, 13, "全体の最大値は13");

        println!("\n=== テスト成功 ===\n");
    }

    #[test]
    fn test_segtree_sum_add_range() {
        println!("\n=== 通常版セグメント木: 和（Sum）のadd_rangeテスト ===");

        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut seg = SegTree::<Sum>::from_vec_sum(&v);
        println!("初期配列: {:?}", seg.to_vec());

        let sum_all = seg.get_range(0, 10);
        println!("全体の和: {}", sum_all);
        assert_eq!(sum_all, 55, "初期の和は55");

        println!("\n区間[2, 7)に10を加算");
        seg.add_range(2, 7, 10);
        let array = seg.to_vec();
        println!("  更新後の配列: {:?}", array);
        assert_eq!(array, vec![1, 2, 13, 14, 15, 16, 17, 8, 9, 10]);

        let sum_range = seg.get_range(2, 7);
        println!("  区間[2, 7)の和: {}", sum_range);
        assert_eq!(sum_range, 75, "加算後の区間の和は75");

        let sum_all = seg.get_range(0, 10);
        println!("  全体の和: {}", sum_all);
        assert_eq!(sum_all, 105, "全体の和は105");

        println!("\n=== テスト成功 ===\n");
    }

    #[test]
    fn test_segtree_complex_operations() {
        println!("\n=== 通常版セグメント木: 複雑な操作テスト（Min） ===");

        let v = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut seg = SegTree::<Min>::from_vec_min(&v);
        println!("初期配列: {:?}", seg.to_vec());

        println!("\n区間[0, 5)に15を加算");
        seg.add_range(0, 5, 15);
        println!("  配列: {:?}", seg.to_vec());

        println!("\n区間[5, 10)に5を加算");
        seg.add_range(5, 10, 5);
        println!("  配列: {:?}", seg.to_vec());

        println!("\nインデックス2を100に更新");
        seg.set(2, 100);
        let array = seg.to_vec();
        println!("  最終配列: {:?}", array);
        assert_eq!(array, vec![25, 35, 100, 55, 65, 65, 75, 85, 95, 105]);

        let min_left = seg.get_range(0, 5);
        println!("  区間[0, 5)の最小値: {}", min_left);
        assert_eq!(min_left, 25, "左半分の最小値は25");

        let min_right = seg.get_range(5, 10);
        println!("  区間[5, 10)の最小値: {}", min_right);
        assert_eq!(min_right, 65, "右半分の最小値は65");

        println!("\n=== テスト成功 ===\n");
    }

    #[test]
    fn test_debug_print_lazy() {
        println!("\n=== LazySegTreeのデバッグ表示テスト ===");

        let mut seg = LazySegTree::<Sum>::from_vec_sum(&[1, 2, 3, 4, 5]);
        println!("初期状態:");
        seg.debug_print();

        println!("区間[1, 4)を10に更新後:");
        seg.update_range(1, 4, 10);
        seg.debug_print();

        println!("区間[0, 3)に5を加算後:");
        seg.add_range(0, 3, 5);
        seg.debug_print();

        println!("=== テスト完了 ===\n");
    }

    #[test]
    fn test_debug_print_normal() {
        println!("\n=== SegTreeのデバッグ表示テスト ===");

        let mut seg = SegTree::<Min>::from_vec_min(&[5, 3, 7, 1, 6]);
        println!("初期状態:");
        seg.debug_print();

        println!("インデックス2を0に更新後:");
        seg.set(2, 0);
        seg.debug_print();

        println!("区間[1, 4)に10を加算後:");
        seg.add_range(1, 4, 10);
        seg.debug_print();

        println!("=== テスト完了 ===\n");
    }
}
