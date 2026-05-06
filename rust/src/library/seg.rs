use num::Integer;
use std::ops::*;

/// モノイド
pub trait Monoid {
    type S: Clone + Copy + Default;
    /// 単位元
    fn identity() -> Self::S;
    /// 演算
    fn binary_operation(a: Self::S, b: Self::S) -> Self::S;
}

/// 遅延セグ木用のモノイド
pub trait MapMonoid {
    type M: Monoid;
    type F: Clone;
    /// 操作モノイド
    fn identity_map() -> Self::F;
    /// 操作fを操作gの後に適用する
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    /// 操作fをデータxに適用する
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
}

/// セグ木
#[derive(Clone)]
pub struct Seg<M: Monoid> {
    /// 元サイズ
    n: usize,
    /// セグ木サイズ
    size: usize,
    /// 木の高さ
    log: usize,
    /// データ配列
    d: Vec<M::S>,
}
impl<M: Monoid> Seg<M> {
    /// 単位元初期化
    pub fn new(n: usize) -> Seg<M> {
        assert!(n > 0, "Seg::new(0) は許可されていません");
        let log = 64 - n.saturating_sub(1).leading_zeros() as usize;
        let size = 1 << log;
        Self {
            n,
            size,
            log,
            d: vec![M::identity(); 2 * size],
        }
    }

    /// 葉から更新
    #[inline]
    fn update(&mut self, k: usize) {
        self.d[k] = M::binary_operation(self.d[2 * k], self.d[2 * k + 1]);
    }

    /// 葉の更新
    pub fn set(&mut self, mut p: usize, x: M::S) {
        assert!(p < self.n);
        p += self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// 区間クエリ（半開区間 `[l, r)` を取得）
    ///
    /// # 注意
    /// **右端 r は含まれません**（`l..r` と同じ挙動）。
    /// 右端を含めたい場合は `prod(l..=r)` を使用してください。
    /// rangeで区間クエリ
    pub fn get_range(&self, mut l: usize, mut r: usize) -> M::S {
        if l == 0 && r == self.n {
            return self.d[1];
        }
        assert!(l <= r && r <= self.n);
        let mut sml = M::identity();
        let mut smr = M::identity();
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(sml, self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(self.d[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::binary_operation(sml, smr)
    }

    /// l から右に向かって、条件 f を満たす最大の r を返す。
    /// つまり、区間 [l, r) の総積が f で true になるギリギリの右端。
    ///
    /// # 制約
    /// f(単位元) は true であること。
    pub fn max_right<F: Fn(&M::S) -> bool>(&self, mut l: usize, f: F) -> usize {
        assert!(l <= self.n);
        assert!(f(&M::identity()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::identity();
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::binary_operation(sm, self.d[l])) {
                while l < self.size {
                    l *= 2;
                    let res = M::binary_operation(sm, self.d[l]);
                    if f(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::binary_operation(sm, self.d[l]);
            l += 1;
            if l.is_power_of_two() {
                break;
            }
        }
        self.n
    }

    /// r を右端として、左に向かって条件 f を満たす最小の l を返す。
    /// つまり、区間 [l, r) の総積が f で true になるギリギリの左端。
    ///
    /// # 制約
    /// f(単位元) は true であること。
    pub fn min_left<F: Fn(&M::S) -> bool>(&self, mut r: usize, f: F) -> usize {
        assert!(r <= self.n);
        assert!(f(&M::identity()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::identity();
        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::binary_operation(self.d[r], sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(self.d[r], sm);
                    if f(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::binary_operation(self.d[r], sm);
            if r.is_power_of_two() {
                break;
            }
        }
        0
    }
}
impl<M: Monoid> From<Vec<M::S>> for Seg<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let log = 64 - n.saturating_sub(1).leading_zeros() as usize;
        let size = 1 << log;
        let mut d = vec![M::identity(); 2 * size];
        d[size..][..n].clone_from_slice(&v);
        let mut ret = Seg { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<M: Monoid> FromIterator<M::S> for Seg<M> {
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        v.into()
    }
}
impl<M: Monoid> Index<usize> for Seg<M> {
    type Output = M::S;
    fn index(&self, index: usize) -> &Self::Output {
        &self.d[index + self.size]
    }
}

/// 遅延セグ木
#[derive(Clone)]
pub struct LazySeg<F: MapMonoid> {
    /// 元サイズ
    n: usize,
    /// セグ木サイズ
    size: usize,
    /// 木の高さ
    log: usize,
    /// データ配列
    d: Vec<<F::M as Monoid>::S>,
    /// 遅延配列
    lz: Vec<F::F>,
}
impl<F: MapMonoid> LazySeg<F> {
    /// 単位元初期化
    ///
    /// # 注意
    /// SumLen (区間加算区間和) のように「長さ」が必要なモノイドの場合、
    /// このメソッドで初期化すると長さが 0 になり、加算が反映されません。
    /// 代わりに `new_with_size` や `from_values` を使用してください。
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "LazySeg::new(0) は許可されていません");
        let log = 64 - n.saturating_sub(1).leading_zeros() as usize;
        let size = 1 << log;
        Self {
            n,
            size,
            log,
            d: vec![F::M::identity(); 2 * size],
            lz: vec![F::identity_map(); size],
        }
    }

    /// 葉の更新
    pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// 葉の取得
    pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::S {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p]
    }

    /// 区間クエリ（半開区間 `[l, r)` を取得）
    ///
    /// # 注意
    /// **右端 r は含まれません**（`l..r` と同じ挙動）。
    /// 右端を含めたい場合は `prod(l..=r)` を使用してください。
    /// rangeで区間クエリ
    pub fn get_range(&mut self, mut l: usize, mut r: usize) -> <F::M as Monoid>::S {
        if l == 0 && r == self.n {
            return self.d[1];
        }
        assert!(l <= r && r <= self.n);
        if l == r {
            return F::M::identity();
        }
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }
        let mut sml = F::M::identity();
        let mut smr = F::M::identity();
        while l < r {
            if l & 1 != 0 {
                sml = F::M::binary_operation(sml, self.d[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = F::M::binary_operation(self.d[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        F::M::binary_operation(sml, smr)
    }

    /// rangeで操作適用
    pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: F::F) {
        assert!(l <= r && r <= self.n);
        if l == r {
            return;
        }
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let l2 = l;
        let r2 = r;
        while l < r {
            if l & 1 != 0 {
                self.all_apply(l, f.clone());
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.all_apply(r, f.clone());
            }
            l >>= 1;
            r >>= 1;
        }
        l = l2;
        r = r2;
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    /// l から右に向かって、条件 f を満たす最大の r を返す。
    /// つまり、区間 [l, r) の総積が f で true になるギリギリの右端。
    ///
    /// # 制約
    /// f(単位元) は true であること。
    pub fn max_right<G: Fn(<F::M as Monoid>::S) -> bool>(&mut self, mut l: usize, g: G) -> usize {
        assert!(l <= self.n);
        assert!(g(F::M::identity()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }
        let mut sm = F::M::identity();
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g(F::M::binary_operation(sm, self.d[l])) {
                while l < self.size {
                    self.push(l);
                    l *= 2;
                    let res = F::M::binary_operation(sm, self.d[l]);
                    if g(res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = F::M::binary_operation(sm, self.d[l]);
            l += 1;
            if l.is_power_of_two() {
                break;
            }
        }
        self.n
    }

    /// r を右端として、左に向かって条件 f を満たす最小の l を返す。
    /// つまり、区間 [l, r) の総積が f で true になるギリギリの左端。
    ///
    /// # 制約
    /// f(単位元) は true であること。
    pub fn min_left<G: Fn(<F::M as Monoid>::S) -> bool>(&mut self, mut r: usize, g: G) -> usize {
        assert!(r <= self.n);
        assert!(g(F::M::identity()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut sm = F::M::identity();
        loop {
            r -= 1;
            while r > 1 && r % 2 != 0 {
                r >>= 1;
            }
            if !g(F::M::binary_operation(self.d[r], sm)) {
                while r < self.size {
                    self.push(r);
                    r = 2 * r + 1;
                    let res = F::M::binary_operation(self.d[r], sm);
                    if g(res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = F::M::binary_operation(self.d[r], sm);
            if r.is_power_of_two() {
                break;
            }
        }
        0
    }

    #[inline]
    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lz[k].clone());
        self.all_apply(2 * k + 1, self.lz[k].clone());
        self.lz[k] = F::identity_map();
    }
    #[inline]
    fn update(&mut self, k: usize) {
        self.d[k] = F::M::binary_operation(self.d[2 * k], self.d[2 * k + 1]);
    }
    #[inline]
    fn all_apply(&mut self, k: usize, f: F::F) {
        self.d[k] = F::mapping(&f, &self.d[k]);
        if k < self.size {
            self.lz[k] = F::composition(&f, &self.lz[k]);
        }
    }
}
impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for LazySeg<F> {
    fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
        let n = v.len();
        let log = 64 - n.saturating_sub(1).leading_zeros() as usize;
        let size = 1 << log;
        let mut d = vec![F::M::identity(); 2 * size];
        let lz = vec![F::identity_map(); size];
        d[size..(size + n)].clone_from_slice(&v);
        let mut ret = LazySeg {
            n,
            size,
            log,
            d,
            lz,
        };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}

// =====================================================
// 具象モノイド定義
// =====================================================
macro_rules! define_monoid {
    ($(
            $name:ident,       // 構造体名
            $alias:ident,      // エイリアス名
            $type:ty,          // 扱う型
            $id:expr,          // 単位元
            |$a:ident, $b:ident| $op:expr // 二項演算の処理
        );* $(;)?) => {
    $(
        #[derive(Clone)]
        pub struct $name;
        impl Monoid for $name {
            type S = $type;
            #[inline] // セグ木では頻繁に呼ばれるのでインライン化推奨
            fn identity() -> Self::S {
                $id
            }
            #[inline]
            fn binary_operation($a: Self::S, $b: Self::S) -> Self::S {
                $op
            }
        }
        pub type $alias = Seg<$name>;
    )*};
}

// 型を変えたい場合は usize を i64 等に書き換える（単位元も合わせて変更すること）
define_monoid!(
    // 基本演算
    Max,    SegMax,    usize,          0, |a, b| a.max(b);
    Min,    SegMin,    usize, usize::MAX, |a, b| a.min(b);
    Sum,    SegSum,    usize,          0, |a, b| a + b;
    SumLen, SegSumLen, (usize, usize), (0, 0), |a, b| (a.0 + b.0, a.1 + b.1); // 区間加算区間和用
    XorLen, SegXorLen, (usize, usize), (0, 0), |a, b| (a.0 ^ b.0, a.1 + b.1); // 区間更新区間XOR用
    // ビット演算
    Xor,    SegXor,    usize,          0, |a, b| a ^ b;
    BitOr,  SegBitOr,  usize,          0, |a, b| a | b;
    BitAnd, SegBitAnd, usize, usize::MAX, |a, b| a & b;
    // 数論
    Gcd,    SegGcd,    usize,          0, |a, b| a.gcd(&b);
    Lcm,    SegLcm,    usize,          1, |a, b| a.lcm(&b);
);

macro_rules! define_map_monoid {
    (
        $(
            $name:ident,        // 構造体名
            $alias:ident,       // エイリアス名
            $monoid:ty,         // 継承するMonoid
            $op_type:ty,        // 操作の型
            $id_map:expr,       // 操作の単位元
            |$f1:ident, $g1:ident| $composition:expr, // composition(new, old)
            |$f2:ident, $x2:ident| $mapping:expr      // mapping(op, data)
        );* $(;)?
    ) => {
        $(
            #[derive(Clone)]
            pub struct $name;
            impl MapMonoid for $name {
                type M = $monoid;
                type F = $op_type;
                #[inline]
                fn identity_map() -> Self::F { $id_map }
                #[inline]
                fn composition($f1: &Self::F, $g1: &Self::F) -> Self::F { $composition }
                #[inline]
                fn mapping($f2: &Self::F, $x2: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S { $mapping }
            }
            pub type $alias = LazySeg<$name>;
        )*
    };
}
define_map_monoid!(
    // -------------------------------------------------------
    // 区間加算 (Add) 系
    // -------------------------------------------------------
    AddMax, LazySegAddMax, Max, usize, 0, |f, g| f + g, |f, x| x + f;
    AddMin, LazySegAddMin, Min, usize, 0, |f, g| f + g, |f, x| x + f;
    AddSum, LazySegAddSum, SumLen, usize, 0, |f, g| f + g, |f, x| (x.0 + f * x.1, x.1);
);
define_map_monoid!(
    // -------------------------------------------------------
    // 区間更新 (Update) 系
    // -------------------------------------------------------
    // 合成: 新しい値(f)がSomeなら上書き、Noneなら古い値(g)を維持
    // 作用: 操作(f)がSomeならその値にする、Noneなら元の値(x)を維持

    // 区間更新・区間最大値
    UpdateMax, LazySegUpdateMax, Max, Option<usize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { *v } else { *x };

    // 区間更新・区間最小値
    UpdateMin, LazySegUpdateMin, Min, Option<usize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { *v } else { *x };

    // 区間更新・区間BitOR (v | v = v なので長さ不要)
    UpdateBitOr, LazySegUpdateBitOr, BitOr, Option<usize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { *v } else { *x };

    // 区間更新・区間BitAND (v & v = v なので長さ不要)
    UpdateBitAnd, LazySegUpdateBitAnd, BitAnd, Option<usize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { *v } else { *x };

    // 区間更新・区間GCD (gcd(v, v) = v なので長さ不要)
    UpdateGcd, LazySegUpdateGcd, Gcd, Option<usize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { *v } else { *x };

    // -------------------------------------------------------
    // 特殊形: 長さが計算に絡むもの (Sum, Xor)
    // -------------------------------------------------------

    // 区間更新・区間和
    // 値を v に変えると、和は v * len になる
    UpdateSum, LazySegUpdateSum, SumLen, Option<usize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { (v * x.1, x.1) } else { *x };

    // 区間更新・区間XOR
    // 値を v に変えると、XOR和は「長さが偶数なら0、奇数ならv」になる
    UpdateXor, LazySegUpdateXor, XorLen, Option<usize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { (if x.1 % 2 == 0 { 0 } else { *v }, x.1) } else { *x };
);

// 長さ付きモノイド用の便利メソッド
macro_rules! impl_value_methods {
    ($seg_type:ty) => {
        impl $seg_type {
            /// 値のみで初期化（長さは自動で1に設定）
            pub fn from_values(values: Vec<usize>) -> Self {
                Self::from(values.into_iter().map(|v| (v, 1)).collect::<Vec<_>>())
            }

            /// サイズだけ指定して0初期化
            pub fn new_with_size(n: usize) -> Self {
                Self::from_values(vec![0; n])
            }

            /// 値のみで set（長さは自動で1）
            pub fn set_value(&mut self, p: usize, val: usize) {
                self.set(p, (val, 1));
            }

            /// 値のみ取得
            pub fn get_value(&mut self, p: usize) -> usize {
                self.get(p).0
            }
        }
    };
}

impl_value_methods!(LazySegAddSum);
impl_value_methods!(LazySegUpdateSum);
impl_value_methods!(LazySegUpdateXor);

#[cfg(test)]
mod tests {
    use crate::library::seg::*;
    // -------------------------------------------------------------------------
    // 1. 各演算の個別テスト
    // -------------------------------------------------------------------------
    #[test]
    fn test_sum() {
        // Add: 足し算 (単位元 0)
        let n = 5;
        let mut seg = SegSum::new(n);
        // [1, 2, 3, 4, 5]
        for i in 0..n {
            seg.set(i, i + 1);
        }
        assert_eq!(seg.get_range(0, 5), 15); // 1+2+3+4+5
        assert_eq!(seg.get_range(1, 3), 5); // 2+3
        assert_eq!(seg.get_range(0, 0), 0); // 空区間は単位元
    }
    #[test]
    fn test_min_max() {
        // Min: 最小値 (単位元 usize::MAX)
        // Max: 最大値 (単位元 0)
        let n = 5;
        let mut seg_min = SegMin::new(n);
        let mut seg_max = SegMax::new(n);
        // [10, 20, 5, 30, 15]
        let data = [10, 20, 5, 30, 15];
        for (i, &v) in data.iter().enumerate() {
            seg_min.set(i, v);
            seg_max.set(i, v);
        }
        // Min Check
        assert_eq!(seg_min.get_range(0, 5), 5); // 全体の最小
        assert_eq!(seg_min.get_range(0, 2), 10); // [10, 20] -> 10
        assert_eq!(seg_min.get_range(3, 5), 15); // [30, 15] -> 15
        assert_eq!(seg_min.get_range(0, 0), usize::MAX); // 単位元
        // Max Check
        assert_eq!(seg_max.get_range(0, 5), 30); // 全体の最大
        assert_eq!(seg_max.get_range(0, 3), 20); // [10, 20, 5] -> 20
        assert_eq!(seg_max.get_range(0, 0), 0); // 単位元
    }
    #[test]
    fn test_bitwise() {
        // Xor: 排他的論理和 (単位元 0)
        // BitOr: 論理和 (単位元 0)
        // BitAnd: 論理積 (単位元 usize::MAX)
        let n = 4;
        let mut seg_xor = SegXor::new(n);
        let mut seg_or = SegBitOr::new(n);
        let mut seg_and = SegBitAnd::new(n);
        // Data: [1(001), 2(010), 4(100), 7(111)]
        let data = [1, 2, 4, 7];
        for (i, &v) in data.iter().enumerate() {
            seg_xor.set(i, v);
            seg_or.set(i, v);
            seg_and.set(i, v);
        }
        // Xor Check
        // 1^2^4^7 = 001 ^ 010 ^ 100 ^ 111 = 111 ^ 111 = 0
        assert_eq!(seg_xor.get_range(0, 4), 0);
        assert_eq!(seg_xor.get_range(0, 2), 3); // 1^2 = 3
        // BitOr Check
        // 1|2|4|7 = 7
        assert_eq!(seg_or.get_range(0, 4), 7);
        // BitAnd Check
        // 1&2&4&7 = 0
        assert_eq!(seg_and.get_range(0, 4), 0);
        // 4&7 = 100 & 111 = 100(4)
        assert_eq!(seg_and.get_range(2, 4), 4);
        // 単位元チェック
        assert_eq!(seg_and.get_range(0, 0), usize::MAX);
    }
    #[test]
    fn test_gcd_lcm() {
        // Gcd: 最大公約数 (単位元 0)
        // Lcm: 最小公倍数 (単位元 1)
        let n = 3;
        let mut seg_gcd = SegGcd::new(n);
        let mut seg_lcm = SegLcm::new(n);
        // Data: [12, 18, 6]
        let data = [12, 18, 6];
        for (i, &v) in data.iter().enumerate() {
            seg_gcd.set(i, v);
            seg_lcm.set(i, v);
        }
        // GCD Check
        assert_eq!(seg_gcd.get_range(0, 2), 6); // gcd(12, 18) = 6
        assert_eq!(seg_gcd.get_range(0, 3), 6); // gcd(12, 18, 6) = 6
        // LCM Check
        assert_eq!(seg_lcm.get_range(0, 2), 36); // lcm(12, 18) = 36
        assert_eq!(seg_lcm.get_range(1, 3), 18); // lcm(18, 6) = 18
        // 単位元チェック
        assert_eq!(seg_gcd.get_range(0, 0), 0);
        assert_eq!(seg_lcm.get_range(0, 0), 1);
    }

    // -------------------------------------------------------------------------
    // 2. 二分探索 (max_right / min_left) のテスト
    // -------------------------------------------------------------------------
    #[test]
    fn test_binary_search() {
        // [1, 1, 1, 1, 1] でAddを使用
        let n = 5;
        let mut seg = SegSum::new(n);
        for i in 0..n {
            seg.set(i, 1);
        }

        // max_right: 和が3未満である限界の右端
        assert_eq!(seg.max_right(0, |&s| s < 3), 2); // index 0,1,2 (sum=3) なので3で止まる
        assert_eq!(seg.max_right(2, |&s| s < 2), 3); // index 2,3 (sum=2) なので4で止まる (2+2=4)

        // min_left: 和が2以下である限界の左端
        assert_eq!(seg.min_left(5, |&s| s <= 2), 3); // index 3,4 (sum=2) OK. index 2,3,4(sum=3) NG.
    }

    // -------------------------------------------------------------------------
    // 3. ランダムストレステスト (愚直解との比較)
    // -------------------------------------------------------------------------
    struct Rng(u64);
    impl Rng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let len = range.end - range.start;
            range.start + (self.next() as usize % len)
        }
    }

    #[test]
    fn stress_test() {
        let n = 50;
        let q = 1000;
        let mut rng = Rng::new(123456);

        // addで構造の正しさを検証するのが一番確実
        let mut seg = SegSum::new(n);
        let mut vec = vec![0; n];

        for _ in 0..q {
            let ty = rng.gen_range(0..2);
            if ty == 0 {
                // Update
                let i = rng.gen_range(0..n);
                let x = rng.gen_range(0..100);
                seg.set(i, x);
                vec[i] = x;
            } else {
                // Query
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r);
                let vec_res: usize = vec[l..r].iter().sum();

                assert_eq!(seg_res, vec_res, "Mismatch at ({}..{})", l, r);
            }
        }
    }

    // -------------------------------------------------------------------------
    // 遅延セグ木
    // 1. 基本動作テスト (Add系: 区間加算)
    // -------------------------------------------------------------------------
    #[test]
    fn test_lazy_add_sum() {
        let n = 10;
        // SumLen系は初期化時に (値, 1) を入れないと長さが0になってしまうので注意
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegAddSum::from(init_data);

        // [0, 5) に 10 を加算
        // 区間長5 * 10 = +50
        seg.apply_range(0, 5, 10);
        assert_eq!(seg.get_range(0, 10).0, 50);

        // [2, 6) に 5 を加算
        // 重なり部分: [2, 5) は 10+5=15
        // 新規部分: [5, 6) は 0+5=5
        // 全体: [0, 2)=10, [2, 5)=15, [5, 6)=5, [6, 10)=0
        seg.apply_range(2, 6, 5);

        // 検算:
        // [0, 2) (長さ2) * 10 = 20
        // [2, 5) (長さ3) * 15 = 45
        // [5, 6) (長さ1) * 5  = 5
        // 合計: 70
        assert_eq!(seg.get_range(0, 10).0, 70);

        // 部分取得
        assert_eq!(seg.get_range(2, 5).0, 45);
    }
    #[test]
    fn test_lazy_add_max() {
        let n = 5;
        let mut seg = LazySegAddMax::new(n);

        // [0, 0, 0, 0, 0]
        seg.apply_range(0, 3, 10); // [10, 10, 10, 0, 0]
        seg.apply_range(2, 5, 20); // [10, 10, 30, 20, 20]

        assert_eq!(seg.get_range(0, 5), 30); // 全体の最大値
        assert_eq!(seg.get_range(0, 2), 10);
        assert_eq!(seg.get_range(3, 5), 20);

        // 点更新との組み合わせ
        seg.set(0, 50); // [50, 10, 30, 20, 20]
        assert_eq!(seg.get_range(0, 5), 50);
    }

    // -------------------------------------------------------------------------
    // 2. 基本動作テスト (Update系: 区間更新)
    // -------------------------------------------------------------------------
    #[test]
    fn test_lazy_update_min() {
        let n = 5;
        // 初期値は Min の単位元 (usize::MAX)
        let mut seg = LazySegUpdateMin::new(n);

        // 全体を 100 に更新
        seg.apply_range(0, 5, Some(100));
        // [100, 100, 100, 100, 100]
        assert_eq!(seg.get_range(0, 5), 100);

        // [1, 4) を 10 に更新
        seg.apply_range(1, 4, Some(10));
        // [100, 10, 10, 10, 100]
        assert_eq!(seg.get_range(0, 5), 10);
        assert_eq!(seg.get_range(0, 1), 100);

        // None (更新なし) を投げても値が変わらないこと
        seg.apply_range(0, 5, None);
        assert_eq!(seg.get_range(0, 5), 10);
    }
    #[test]
    fn test_lazy_update_sum() {
        let n = 5;
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegUpdateSum::from(init_data);

        // [0, 5) を 10 に更新 -> 合計 50
        seg.apply_range(0, 5, Some(10));
        assert_eq!(seg.get_range(0, 5).0, 50);

        // [0, 3) を 5 に更新 -> [5, 5, 5, 10, 10]
        seg.apply_range(0, 3, Some(5));

        // 検算: 5*3 + 10*2 = 15 + 20 = 35
        assert_eq!(seg.get_range(0, 5).0, 35);
    }
    #[test]
    fn test_lazy_update_xor() {
        // UpdateXor は「長さが偶数なら0、奇数なら値」になるはず
        let n = 5;
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegUpdateXor::from(init_data);

        // [0, 5) を 3 (011) に更新。長さ5(奇数) -> 結果 3
        seg.apply_range(0, 5, Some(3));
        assert_eq!(seg.get_range(0, 5).0, 3);

        // [0, 4) をチェック。長さ4(偶数) -> 結果 0
        assert_eq!(seg.get_range(0, 4).0, 0);
    }

    // -------------------------------------------------------------------------
    // 3. 遅延セグ木上の二分探索
    // -------------------------------------------------------------------------
    #[test]
    fn test_lazy_binary_search() {
        let n = 10;
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegAddSum::from(init_data);

        // 全てに 1 を加算 -> [1, 1, ..., 1]
        seg.apply_range(0, n, 1);

        // max_right: 左端0から和が 5 未満になる右端は？
        // 1+1+1+1+1 = 5 (NG) -> index 5の手前まで -> 4
        // ※ すみません、条件の境界値によります。
        // 「s < 5」なら sum([0,4))=4 (OK), sum([0,5))=5 (NG) -> 答え 4
        assert_eq!(seg.max_right(0, |s| s.0 < 5), 4);

        // min_left: 右端10から和が 3 以下になる左端は？
        // sum([7, 10)) = 3 (OK)
        // sum([6, 10)) = 4 (NG) -> 答え 7
        assert_eq!(seg.min_left(10, |s| s.0 <= 3), 7);
    }

    // -------------------------------------------------------------------------
    // 4. ランダムストレステスト (愚直解との比較)
    // -------------------------------------------------------------------------
    #[test]
    fn stress_test_add_sum() {
        let n = 30; // サイズ
        let q = 1000; // クエリ数
        let mut rng = Rng::new(2025);

        // 遅延セグ木 (値, 1) で初期化
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegAddSum::from(init_data);

        // 愚直配列 (値のみ)
        let mut vec = vec![0; n];

        for _ in 0..q {
            let ty = rng.gen_range(0..3);
            if ty == 0 {
                // Range Add Apply
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);
                let val = rng.gen_range(0..100);

                seg.apply_range(l, r, val);
                for item in vec.iter_mut().take(r).skip(l) {
                    *item += val;
                }
            } else if ty == 1 {
                // Point Set
                let i = rng.gen_range(0..n);
                let val = rng.gen_range(0..100);

                seg.set(i, (val, 1)); // setのときは長さを維持する必要あり
                vec[i] = val;
            } else {
                // Range Query
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r).0;
                let vec_res: usize = vec[l..r].iter().sum();

                assert_eq!(seg_res, vec_res, "AddSum Mismatch at query {},{}", l, r);
            }
        }
    }
    #[test]
    fn stress_test_update_min() {
        let n = 30;
        let q = 1000;
        let mut rng = Rng::new(9999);

        // 初期値は適当な値で埋める
        let mut vec = vec![0; n];
        for item in vec.iter_mut() {
            *item = rng.gen_range(0..1000);
        }

        let mut seg = LazySegUpdateMin::new(n);
        for (i, item) in vec.iter().enumerate() {
            seg.set(i, *item);
        }

        for _ in 0..q {
            let ty = rng.gen_range(0..3);
            if ty == 0 {
                // Range Update Apply
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);
                let val = rng.gen_range(0..1000);

                seg.apply_range(l, r, Some(val));
                for item in vec.iter_mut().take(r).skip(l) {
                    *item = val;
                }
            } else if ty == 1 {
                // Point Set
                let i = rng.gen_range(0..n);
                let val = rng.gen_range(0..1000);

                seg.set(i, val);
                vec[i] = val;
            } else {
                // Range Query
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r);
                let vec_res = if l == r {
                    usize::MAX // 単位元
                } else {
                    *vec[l..r].iter().min().unwrap()
                };

                assert_eq!(seg_res, vec_res, "UpdateMin Mismatch at query {}..{}", l, r);
            }
        }
    }
    #[test]
    fn test_edge_cases() {
        // サイズ1
        let mut seg = SegSum::new(1);
        seg.set(0, 42);
        assert_eq!(seg.get_range(0, 1), 42);

        // 空区間
        assert_eq!(seg.get_range(0, 0), 0);
        assert_eq!(seg.get_range(1, 1), 0);

        // 境界値
        let mut seg = SegMin::new(3);
        seg.set(0, usize::MAX);
        assert_eq!(seg.get_range(0, 1), usize::MAX);
    }
    #[test]
    fn test_value_methods() {
        // ========================================
        // LazySegAddSum（区間加算・区間和）
        // ========================================

        // from_values でシンプルに初期化
        let mut seg = LazySegAddSum::from_values(vec![1, 2, 3, 4, 5]);
        assert_eq!(seg.get_range(0, 5).0, 15); // 1+2+3+4+5

        // set_value で値のみ設定
        seg.set_value(0, 10);
        assert_eq!(seg.get_range(0, 5).0, 24); // 10+2+3+4+5

        // get_value で値のみ取得
        assert_eq!(seg.get_value(0), 10);
        assert_eq!(seg.get_value(1), 2);

        // 区間加算
        seg.apply_range(1, 4, 5); // index 1,2,3 に +5
        assert_eq!(seg.get_range(0, 5).0, 39); // 10+(2+5)+(3+5)+(4+5)+5

        // new_with_size で0初期化
        let mut seg2 = LazySegAddSum::new_with_size(3);
        assert_eq!(seg2.get_range(0, 3).0, 0);
        seg2.apply_range(0, 3, 10);
        assert_eq!(seg2.get_range(0, 3).0, 30);

        // ========================================
        // LazySegUpdateSum（区間更新・区間和）
        // ========================================

        let mut seg = LazySegUpdateSum::from_values(vec![1, 2, 3, 4, 5]);
        assert_eq!(seg.get_range(0, 5).0, 15);

        // 区間更新
        seg.apply_range(1, 4, Some(10)); // index 1,2,3 を 10 に
        assert_eq!(seg.get_range(0, 5).0, 36); // 1+10+10+10+5

        seg.set_value(0, 100);
        assert_eq!(seg.get_value(0), 100);
        assert_eq!(seg.get_range(0, 5).0, 135); // 100+10+10+10+5

        // ========================================
        // LazySegUpdateXor（区間更新・区間XOR）
        // ========================================

        let mut seg = LazySegUpdateXor::from_values(vec![1, 2, 4, 8, 16]);
        // 1^2^4^8^16 = 31
        assert_eq!(seg.get_range(0, 5).0, 31);

        // 全て 7 に更新
        seg.apply_range(0, 5, Some(7));
        // 7^7^7^7^7 = 7 (奇数個なので7)
        assert_eq!(seg.get_range(0, 5).0, 7);

        // 偶数個の範囲
        assert_eq!(seg.get_range(0, 4).0, 0); // 7^7^7^7 = 0

        seg.set_value(0, 3);
        assert_eq!(seg.get_value(0), 3);
        // 3^7^7^7^7 = 3^0 = 3
        assert_eq!(seg.get_range(0, 5).0, 3);
    }
    #[test]
    fn test_value_methods_stress() {
        // LazySegAddSum のストレステスト（便利メソッド版）
        let n = 30;
        let q = 1000;
        let mut rng = Rng::new(54321);

        let mut seg = LazySegAddSum::new_with_size(n);
        let mut vec = vec![0; n];

        for _ in 0..q {
            let ty = rng.gen_range(0..3);
            if ty == 0 {
                // Range Add
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);
                let val = rng.gen_range(0..100);

                seg.apply_range(l, r, val);
                for item in vec.iter_mut().take(r).skip(l) {
                    *item += val;
                }
            } else if ty == 1 {
                // Point Set (便利メソッド)
                let i = rng.gen_range(0..n);
                let val = rng.gen_range(0..100);

                seg.set_value(i, val);
                vec[i] = val;
            } else {
                // Range Query (便利メソッド)
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r);
                let vec_res = vec[l..r].iter().sum::<usize>();

                assert_eq!(seg_res.0, vec_res, "ValueMethod Mismatch at {}..{}", l, r);
            }
        }
    }
    #[test]
    fn test_value_methods_comparison() {
        // 従来の方法と便利メソッドで結果が一致することを確認
        let n = 10;
        let init_values = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

        // 従来の方法
        let init_data: Vec<_> = init_values.iter().map(|&v| (v, 1)).collect();
        let mut seg_old = LazySegAddSum::from(init_data);

        // 便利メソッド
        let mut seg_new = LazySegAddSum::from_values(init_values.clone());

        // 初期状態の一致確認
        for i in 0..n {
            assert_eq!(seg_old.get(i).0, seg_new.get_value(i));
        }
        assert_eq!(seg_old.get_range(0, n).0, seg_new.get_range(0, n).0);

        // 区間加算後の一致確認
        seg_old.apply_range(2, 7, 10);
        seg_new.apply_range(2, 7, 10);

        for i in 0..n {
            assert_eq!(
                seg_old.get(i).0,
                seg_new.get_value(i),
                "Mismatch at index {}",
                i
            );
        }
        assert_eq!(seg_old.get_range(0, n).0, seg_new.get_range(0, n).0);

        // 点更新後の一致確認
        seg_old.set(5, (100, 1));
        seg_new.set_value(5, 100);

        assert_eq!(seg_old.get(5).0, seg_new.get_value(5));
        assert_eq!(seg_old.get_range(0, n).0, seg_new.get_range(0, n).0);
    }
    #[test]
    fn test_value_methods_edge_cases() {
        // サイズ1のケース
        let mut seg = LazySegAddSum::from_values(vec![42]);
        assert_eq!(seg.get_value(0), 42);
        assert_eq!(seg.get_range(0, 1).0, 42);

        seg.set_value(0, 100);
        assert_eq!(seg.get_value(0), 100);

        seg.apply_range(0, 1, 50);
        assert_eq!(seg.get_range(0, 1).0, 150);

        // 空初期化
        let mut seg = LazySegAddSum::new_with_size(5);
        assert_eq!(seg.get_range(0, 5).0, 0);

        for i in 0..5 {
            assert_eq!(seg.get_value(i), 0);
        }

        // 部分区間のテスト
        let mut seg = LazySegUpdateSum::from_values(vec![10, 20, 30, 40, 50]);

        seg.apply_range(1, 3, Some(100)); // [10, 100, 100, 40, 50]
        assert_eq!(seg.get_range(0, 5).0, 300);
        assert_eq!(seg.get_range(1, 3).0, 200);
        assert_eq!(seg.get_range(0, 1).0, 10);
        assert_eq!(seg.get_range(4, 5).0, 50);
    }
    #[test]
    fn test_value_methods_xor_logic() {
        // XOR の長さ依存ロジックをテスト
        let mut seg = LazySegUpdateXor::new_with_size(6);

        // 全て 5 に設定
        seg.apply_range(0, 6, Some(5));

        // 偶数個: 0
        assert_eq!(seg.get_range(0, 2).0, 0); // 5^5
        assert_eq!(seg.get_range(0, 4).0, 0); // 5^5^5^5
        assert_eq!(seg.get_range(0, 6).0, 0); // 5^5^5^5^5^5

        // 奇数個: 5
        assert_eq!(seg.get_range(0, 1).0, 5); // 5
        assert_eq!(seg.get_range(0, 3).0, 5); // 5^5^5
        assert_eq!(seg.get_range(0, 5).0, 5); // 5^5^5^5^5

        // 複雑なパターン
        let mut seg = LazySegUpdateXor::from_values(vec![1, 2, 3, 4]);
        seg.apply_range(0, 2, Some(7)); // [7, 7, 3, 4]
        seg.apply_range(2, 4, Some(15)); // [7, 7, 15, 15]

        assert_eq!(seg.get_range(0, 2).0, 0); // 7^7
        assert_eq!(seg.get_range(2, 4).0, 0); // 15^15
        assert_eq!(seg.get_range(0, 4).0, 0); // 7^7^15^15
        assert_eq!(seg.get_range(0, 3).0, 15); // 7^7^15 = 15
        assert_eq!(seg.get_range(1, 4).0, 7); // 7^15^15 = 7
    }
}
