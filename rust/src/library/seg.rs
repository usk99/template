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
        let log = usize::BITS as usize - n.saturating_sub(1).leading_zeros() as usize;
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
        assert!(l <= r && r <= self.n);
        if l == 0 && r == self.n {
            return self.d[1];
        }
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
        assert!(n > 0, "Seg::from(vec) には1要素以上必要です");
        let log = usize::BITS as usize - n.saturating_sub(1).leading_zeros() as usize;
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
        let log = usize::BITS as usize - n.saturating_sub(1).leading_zeros() as usize;
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
        assert!(n > 0, "LazySeg::from(vec) には1要素以上必要です");
        let log = usize::BITS as usize - n.saturating_sub(1).leading_zeros() as usize;
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

define_monoid!(
    // 基本演算 (usize)
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
    // 基本演算 (isize)
    MaxI,    SegMaxI,    isize, isize::MIN, |a, b| a.max(b);
    MinI,    SegMinI,    isize, isize::MAX, |a, b| a.min(b);
    SumI,    SegSumI,    isize,          0, |a, b| a + b;
    SumLenI, SegSumLenI, (isize, isize), (0, 0), |a, b| (a.0 + b.0, a.1 + b.1); // 区間加算区間和用
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
    // 区間加算 (Add) 系 (usize)
    // -------------------------------------------------------
    AddMax, LazySegAddMax, Max, usize, 0, |f, g| f + g, |f, x| x + f;
    AddMin, LazySegAddMin, Min, usize, 0, |f, g| f + g, |f, x| x + f;
    AddSum, LazySegAddSum, SumLen, usize, 0, |f, g| f + g, |f, x| (x.0 + f * x.1, x.1);
    // 区間加算 (Add) 系 (isize)
    AddMaxI, LazySegAddMaxI, MaxI, isize, 0, |f, g| f + g, |f, x| x + f;
    AddMinI, LazySegAddMinI, MinI, isize, 0, |f, g| f + g, |f, x| x + f;
    AddSumI, LazySegAddSumI, SumLenI, isize, 0, |f, g| f + g, |f, x| (x.0 + f * x.1, x.1);
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

    // 区間更新 (Update) 系 (isize)
    UpdateMaxI, LazySegUpdateMaxI, MaxI, Option<isize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { *v } else { *x };

    UpdateMinI, LazySegUpdateMinI, MinI, Option<isize>, None,
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

    // 区間更新・区間和 (isize)
    UpdateSumI, LazySegUpdateSumI, SumLenI, Option<isize>, None,
    |f, g| if f.is_some() { *f } else { *g },
    |f, x| if let Some(v) = f { (v * x.1, x.1) } else { *x };
);

// 長さ付きモノイド用の便利メソッド (usize)
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

// 長さ付きモノイド用の便利メソッド (isize)
macro_rules! impl_ivalue_methods {
    ($seg_type:ty) => {
        impl $seg_type {
            /// 値のみで初期化（長さは自動で1に設定）
            pub fn from_values(values: Vec<isize>) -> Self {
                Self::from(values.into_iter().map(|v| (v, 1)).collect::<Vec<_>>())
            }
            /// サイズだけ指定して0初期化
            pub fn new_with_size(n: usize) -> Self {
                Self::from_values(vec![0; n])
            }
            /// 値のみで set（長さは自動で1）
            pub fn set_value(&mut self, p: usize, val: isize) {
                self.set(p, (val, 1));
            }
            /// 値のみ取得
            pub fn get_value(&mut self, p: usize) -> isize {
                self.get(p).0
            }
        }
    };
}

impl_value_methods!(LazySegAddSum);
impl_value_methods!(LazySegUpdateSum);
impl_value_methods!(LazySegUpdateXor);
impl_ivalue_methods!(LazySegAddSumI);
impl_ivalue_methods!(LazySegUpdateSumI);
