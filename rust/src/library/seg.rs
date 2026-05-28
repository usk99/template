use num::{Integer, PrimInt};
use std::{marker::PhantomData, ops::*};

// =====================================================
// Seg (通常セグ木)
// =====================================================

/// モノイド
pub trait Monoid {
    type S: Copy;
    /// 単位元
    fn identity() -> Self::S;
    /// 演算
    fn binary_operation(a: Self::S, b: Self::S) -> Self::S;
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
    pub fn new(v: &[M::S]) -> Self {
        let n = v.len();
        assert!(n > 0);
        let size = n.next_power_of_two();
        let log = size.trailing_zeros() as usize;
        let mut d = vec![M::identity(); 2 * size];
        d[size..size + n].clone_from_slice(v);
        let mut ret = Self { n, size, log, d };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }

    /// 葉から更新
    #[inline]
    fn update(&mut self, k: usize) {
        self.d[k] = M::binary_operation(self.d[2 * k], self.d[2 * k + 1]);
    }
    #[inline]
    fn update_from(&mut self, p: usize) {
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// 葉の更新
    pub fn set(&mut self, mut p: usize, x: M::S) {
        assert!(p < self.n);
        p += self.size;
        self.d[p] = x;
        self.update_from(p);
    }

    /// 葉に関数を適用して更新（`seg[i] += 1` 相当を `seg.apply(i, |x| x + 1)` で書ける）
    pub fn apply<G: FnOnce(M::S) -> M::S>(&mut self, mut p: usize, f: G) {
        assert!(p < self.n);
        p += self.size;
        self.d[p] = f(self.d[p]);
        self.update_from(p);
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
impl<M: Monoid> Index<usize> for Seg<M> {
    type Output = M::S;
    fn index(&self, index: usize) -> &Self::Output {
        &self.d[index + self.size]
    }
}
impl<T: PrimInt + Default> From<&[T]> for Seg<MinMax<T>> {
    fn from(v: &[T]) -> Self {
        v.iter().copied().collect()
    }
}
impl<T: PrimInt + Default> FromIterator<T> for Seg<MinMax<T>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mapped: Vec<(T, T)> = iter.into_iter().map(|x| (x, x)).collect();
        Self::new(&mapped)
    }
}

// =====================================================
// 具象モノイド (ジェネリック)
// =====================================================

macro_rules! impl_monoids {
    (@impl $bound:tt, [$( ($name:ident, $s:ty, $id:expr, |$a:ident, $b:ident| $op:expr) ),* $(,)?]) => {
        $( impl_monoids!(@one $bound, $name, $s, $id, |$a, $b| $op); )*
    };
    (@one [$($b:tt)*], $name:ident, $s:ty, $id:expr, |$a:ident, $bv:ident| $op:expr) => {
        impl<T: $($b)*> Monoid for $name<T> {
            type S = $s;
            #[inline] fn identity() -> Self::S { $id }
            #[inline] fn binary_operation($a: Self::S, $bv: Self::S) -> Self::S { $op }
        }
    };
}
impl_monoids!(@impl [PrimInt + Default], [
    (Max,    T,      T::min_value(),         |a, b| a.max(b)),
    (Min,    T,      T::max_value(),         |a, b| a.min(b)),
    (Sum,    T,      T::zero(),              |a, b| a + b),
    (Xor,    T,      T::zero(),              |a, b| a ^ b),
    (BitOr,  T,      T::zero(),              |a, b| a | b),
    (BitAnd, T,      T::max_value(),         |a, b| a & b),
    (SumLen, (T, T), (T::zero(), T::zero()),              |a, b| (a.0 + b.0, a.1 + b.1)),
    (XorLen, (T, T), (T::zero(), T::zero()),              |a, b| (a.0 ^ b.0, a.1 + b.1)),
    (MinMax, (T, T), (T::max_value(), T::min_value()),    |a, b| (a.0.min(b.0), a.1.max(b.1))),
]);
impl_monoids!(@impl [Integer + PrimInt + Default], [
    (Gcd, T, T::zero(), |a, b| a.gcd(&b)),
    (Lcm, T, T::one(),  |a, b| a.lcm(&b)),
]);

macro_rules! monoid_struct {
    ($name:ident, $alias:ident) => {
        pub struct $name<T>(PhantomData<T>);
        pub type $alias<T> = Seg<$name<T>>;
    };
    ($name:ident) => {
        pub struct $name<T>(PhantomData<T>);
    };
}
monoid_struct!(Max, SegMax);
monoid_struct!(Min, SegMin);
monoid_struct!(Sum, SegSum);
monoid_struct!(Xor, SegXor);
monoid_struct!(BitOr, SegBitOr);
monoid_struct!(BitAnd, SegBitAnd);
monoid_struct!(SumLen);
monoid_struct!(XorLen);
monoid_struct!(MinMax, SegMinMax);
monoid_struct!(Gcd, SegGcd);
monoid_struct!(Lcm, SegLcm);

// =====================================================
// LazySeg (遅延セグ木)
// =====================================================

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
    pub fn new(v: &[<F::M as Monoid>::S]) -> Self {
        let n = v.len();
        assert!(n > 0);
        let size = n.next_power_of_two();
        let log = size.trailing_zeros() as usize;
        let mut d = vec![F::M::identity(); 2 * size];
        let lz = vec![F::identity_map(); size];
        d[size..(size + n)].clone_from_slice(v);
        let mut ret = Self {
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

    /// 葉の更新
    pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
        assert!(p < self.n);
        p += self.size;
        self.push_to(p);
        self.d[p] = x;
        self.update_from(p);
    }

    /// 葉の取得
    pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::S {
        assert!(p < self.n);
        p += self.size;
        self.push_to(p);
        self.d[p]
    }

    /// 葉に関数を適用して更新（`seg[i] += 1` 相当を `seg.apply(i, |x| x + 1)` で書ける）
    pub fn apply<G: FnOnce(<F::M as Monoid>::S) -> <F::M as Monoid>::S>(
        &mut self,
        mut p: usize,
        f: G,
    ) {
        assert!(p < self.n);
        p += self.size;
        self.push_to(p);
        self.d[p] = f(self.d[p]);
        self.update_from(p);
    }

    /// 区間クエリ（半開区間 `[l, r)` を取得）
    ///
    /// # 注意
    /// **右端 r は含まれません**（`l..r` と同じ挙動）。
    /// 右端を含めたい場合は `prod(l..=r)` を使用してください。
    /// rangeで区間クエリ
    pub fn get_range(&mut self, mut l: usize, mut r: usize) -> <F::M as Monoid>::S {
        assert!(l <= r && r <= self.n);
        if l == 0 && r == self.n {
            return self.d[1];
        }
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
        let (l0, r0) = (l, r);
        while l < r {
            if l & 1 != 0 {
                self.all_apply(l, &f);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.all_apply(r, &f);
            }
            l >>= 1;
            r >>= 1;
        }
        for i in 1..=self.log {
            if ((l0 >> i) << i) != l0 {
                self.update(l0 >> i);
            }
            if ((r0 >> i) << i) != r0 {
                self.update((r0 - 1) >> i);
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
        self.push_to(l);
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
            while r > 1 && r % 2 == 1 {
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
    fn push_to(&mut self, p: usize) {
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
    }
    #[inline]
    fn update_from(&mut self, p: usize) {
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }
    #[inline]
    fn push(&mut self, k: usize) {
        let f = std::mem::replace(&mut self.lz[k], F::identity_map());
        self.all_apply(2 * k, &f);
        self.all_apply(2 * k + 1, &f);
    }
    #[inline]
    fn update(&mut self, k: usize) {
        self.d[k] = F::M::binary_operation(self.d[2 * k], self.d[2 * k + 1]);
    }
    #[inline]
    fn all_apply(&mut self, k: usize, f: &F::F) {
        self.d[k] = F::mapping(f, &self.d[k]);
        if k < self.size {
            self.lz[k] = F::composition(f, &self.lz[k]);
        }
    }
}
macro_rules! impl_lazy_seg_from_prim {
    ($($name:ident),* $(,)?) => {$(
        impl<T: PrimInt + Default> From<&[T]> for LazySeg<$name<T>> {
            fn from(v: &[T]) -> Self {
                v.iter().copied().collect()
            }
        }
        impl<T: PrimInt + Default> FromIterator<T> for LazySeg<$name<T>> {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                Self::new(&iter.into_iter().map(|x| (x, T::one())).collect::<Vec<_>>())
            }
        }
    )*};
}
impl_lazy_seg_from_prim!(AddSum, UpdateSum, UpdateXor);

// =====================================================
// 具象 MapMonoid (ジェネリック)
// =====================================================

macro_rules! map_monoid_struct {
    ($name:ident, $alias:ident) => {
        pub struct $name<T>(PhantomData<T>);
        pub type $alias<T> = LazySeg<$name<T>>;
    };
}
map_monoid_struct!(AddMax, LazySegAddMax);
map_monoid_struct!(AddMin, LazySegAddMin);
map_monoid_struct!(AddSum, LazySegAddSum);
map_monoid_struct!(UpdateMax, LazySegUpdateMax);
map_monoid_struct!(UpdateMin, LazySegUpdateMin);
map_monoid_struct!(UpdateBitOr, LazySegUpdateBitOr);
map_monoid_struct!(UpdateBitAnd, LazySegUpdateBitAnd);
map_monoid_struct!(UpdateSum, LazySegUpdateSum);
map_monoid_struct!(UpdateXor, LazySegUpdateXor);
map_monoid_struct!(UpdateGcd, LazySegUpdateGcd);
map_monoid_struct!(UpdateLcm, LazySegUpdateLcm);

macro_rules! impl_map_monoid {
    (@impl $bound:tt, [$( ($name:ident, $m:ty, $f:ty, $id:expr, |$fc:ident, $gc:ident| $comp:expr, |$fm:ident, $xm:ident| $map:expr) ),* $(,)?]) => {
        $( impl_map_monoid!(@one $bound, $name, $m, $f, $id, |$fc, $gc| $comp, |$fm, $xm| $map); )*
    };
    (@one [$($b:tt)*], $name:ident, $m:ty, $f:ty, $id:expr, |$fc:ident, $gc:ident| $comp:expr, |$fm:ident, $xm:ident| $map:expr) => {
        impl<T: $($b)*> MapMonoid for $name<T> {
            type M = $m;
            type F = $f;
            #[inline] fn identity_map() -> Self::F { $id }
            #[inline] fn composition($fc: &Self::F, $gc: &Self::F) -> Self::F { $comp }
            #[inline] fn mapping($fm: &Self::F, $xm: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S { $map }
        }
    };
}
impl_map_monoid!(@impl [PrimInt + Default], [
    (AddMax,       Max<T>,    T,   T::zero(), |f, g| *f + *g,                          |f,x|*x+ *f),
    (AddMin,       Min<T>,    T,   T::zero(), |f, g| *f + *g,                          |f,x|*x+ *f),
    (AddSum,       SumLen<T>, T,   T::zero(), |f, g| *f + *g,                          |f,x|(x.0+ *f * x.1,x.1)),
    (UpdateMax,    Max<T>,    Option<T>, None,|f, g| (*f).or(*g),|f,x|(*f).unwrap_or(*x)),
    (UpdateMin,    Min<T>,    Option<T>, None,|f, g| (*f).or(*g),|f,x|(*f).unwrap_or(*x)),
    (UpdateBitOr,  BitOr<T>,  Option<T>, None,|f, g| (*f).or(*g),|f,x|(*f).unwrap_or(*x)),
    (UpdateBitAnd, BitAnd<T>, Option<T>, None,|f, g| (*f).or(*g),|f,x|(*f).unwrap_or(*x)),
    (UpdateSum,    SumLen<T>, Option<T>, None,|f, g| (*f).or(*g),|f,x|(*f).map_or(*x,|v|(v*x.1,x.1))),
    // 値をvに変えると、XOR和は長さが奇数ならv、偶数なら0
    (UpdateXor,    XorLen<T>, Option<T>, None,|f, g| (*f).or(*g),|f,x|(*f).map_or(*x,|v|(if x.1&T::one()==T::zero(){T::zero()}else{v},x.1))),
]);
impl_map_monoid!(@impl [Integer + PrimInt + Default], [
    (UpdateGcd,Gcd<T>,Option<T>,None,|f,g|(*f).or(*g),|f,x|(*f).unwrap_or(*x)),
    (UpdateLcm,Lcm<T>,Option<T>,None,|f,g|(*f).or(*g),|f,x|(*f).unwrap_or(*x)),
]);
