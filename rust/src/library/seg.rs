use num::{Bounded, Integer, PrimInt};
use std::{marker::PhantomData, ops::*};

// =====================================================
// Seg (通常セグ木)
// =====================================================

/// モノイド
pub trait Monoid {
    type S: Clone + Copy + Default;
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
    pub fn new(v: &[M::S]) -> Seg<M> {
        let n = v.len();
        assert!(n > 0);
        let log = usize::BITS as usize - n.saturating_sub(1).leading_zeros() as usize;
        let size = 1 << log;
        let mut d = vec![M::identity(); 2 * size];
        d[size..][..n].clone_from_slice(v);
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
impl<M: Monoid> Index<usize> for Seg<M> {
    type Output = M::S;
    fn index(&self, index: usize) -> &Self::Output {
        &self.d[index + self.size]
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
    (SumLen, (T, T), (T::zero(), T::zero()), |a, b| (a.0 + b.0, a.1 + b.1)),
    (XorLen, (T, T), (T::zero(), T::zero()), |a, b| (a.0 ^ b.0, a.1 + b.1)),
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
        let log = usize::BITS as usize - n.saturating_sub(1).leading_zeros() as usize;
        let size = 1 << log;
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
impl<M: MapMonoid> Index<usize> for LazySeg<M> {
    type Output = <M::M as Monoid>::S;
    fn index(&self, index: usize) -> &Self::Output {
        &self.d[index + self.size]
    }
}

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
map_monoid_struct!(UpdateGcd, LazySegUpdateGcd);
map_monoid_struct!(UpdateSum, LazySegUpdateSum);
map_monoid_struct!(UpdateXor, LazySegUpdateXor);

impl<T: PrimInt + Default> MapMonoid for AddMax<T> {
    type M = Max<T>;
    type F = T;
    #[inline]
    fn identity_map() -> T {
        T::zero()
    }
    #[inline]
    fn composition(f: &T, g: &T) -> T {
        *f + *g
    }
    #[inline]
    fn mapping(f: &T, x: &T) -> T {
        *x + *f
    }
}
impl<T: PrimInt + Default> MapMonoid for AddMin<T> {
    type M = Min<T>;
    type F = T;
    #[inline]
    fn identity_map() -> T {
        T::zero()
    }
    #[inline]
    fn composition(f: &T, g: &T) -> T {
        *f + *g
    }
    #[inline]
    fn mapping(f: &T, x: &T) -> T {
        *x + *f
    }
}
impl<T: PrimInt + Default> MapMonoid for AddSum<T> {
    type M = SumLen<T>;
    type F = T;
    #[inline]
    fn identity_map() -> T {
        T::zero()
    }
    #[inline]
    fn composition(f: &T, g: &T) -> T {
        *f + *g
    }
    #[inline]
    fn mapping(f: &T, x: &(T, T)) -> (T, T) {
        (x.0 + *f * x.1, x.1)
    }
}
impl<T: PrimInt + Default> MapMonoid for UpdateMax<T> {
    type M = Max<T>;
    type F = Option<T>;
    #[inline]
    fn identity_map() -> Option<T> {
        None
    }
    #[inline]
    fn composition(f: &Option<T>, g: &Option<T>) -> Option<T> {
        if f.is_some() { *f } else { *g }
    }
    #[inline]
    fn mapping(f: &Option<T>, x: &T) -> T {
        if let Some(v) = f { *v } else { *x }
    }
}
impl<T: PrimInt + Default> MapMonoid for UpdateMin<T> {
    type M = Min<T>;
    type F = Option<T>;
    #[inline]
    fn identity_map() -> Option<T> {
        None
    }
    #[inline]
    fn composition(f: &Option<T>, g: &Option<T>) -> Option<T> {
        if f.is_some() { *f } else { *g }
    }
    #[inline]
    fn mapping(f: &Option<T>, x: &T) -> T {
        if let Some(v) = f { *v } else { *x }
    }
}
impl<T: PrimInt + Default> MapMonoid for UpdateBitOr<T> {
    type M = BitOr<T>;
    type F = Option<T>;
    #[inline]
    fn identity_map() -> Option<T> {
        None
    }
    #[inline]
    fn composition(f: &Option<T>, g: &Option<T>) -> Option<T> {
        if f.is_some() { *f } else { *g }
    }
    #[inline]
    fn mapping(f: &Option<T>, x: &T) -> T {
        if let Some(v) = f { *v } else { *x }
    }
}
impl<T: PrimInt + Default> MapMonoid for UpdateBitAnd<T> {
    type M = BitAnd<T>;
    type F = Option<T>;
    #[inline]
    fn identity_map() -> Option<T> {
        None
    }
    #[inline]
    fn composition(f: &Option<T>, g: &Option<T>) -> Option<T> {
        if f.is_some() { *f } else { *g }
    }
    #[inline]
    fn mapping(f: &Option<T>, x: &T) -> T {
        if let Some(v) = f { *v } else { *x }
    }
}
impl<T: Integer + PrimInt + Default> MapMonoid for UpdateGcd<T> {
    type M = Gcd<T>;
    type F = Option<T>;
    #[inline]
    fn identity_map() -> Option<T> {
        None
    }
    #[inline]
    fn composition(f: &Option<T>, g: &Option<T>) -> Option<T> {
        if f.is_some() { *f } else { *g }
    }
    #[inline]
    fn mapping(f: &Option<T>, x: &T) -> T {
        if let Some(v) = f { *v } else { *x }
    }
}
impl<T: PrimInt + Default> MapMonoid for UpdateSum<T> {
    type M = SumLen<T>;
    type F = Option<T>;
    #[inline]
    fn identity_map() -> Option<T> {
        None
    }
    #[inline]
    fn composition(f: &Option<T>, g: &Option<T>) -> Option<T> {
        if f.is_some() { *f } else { *g }
    }
    #[inline]
    fn mapping(f: &Option<T>, x: &(T, T)) -> (T, T) {
        if let Some(v) = f { (*v * x.1, x.1) } else { *x }
    }
}
impl<T: PrimInt + Default> MapMonoid for UpdateXor<T> {
    type M = XorLen<T>;
    type F = Option<T>;
    #[inline]
    fn identity_map() -> Option<T> {
        None
    }
    #[inline]
    fn composition(f: &Option<T>, g: &Option<T>) -> Option<T> {
        if f.is_some() { *f } else { *g }
    }
    #[inline]
    fn mapping(f: &Option<T>, x: &(T, T)) -> (T, T) {
        // 値をvに変えると、XOR和は長さが奇数ならv、偶数なら0
        if let Some(v) = f {
            (
                if x.1 % T::from(2).unwrap() == T::zero() {
                    T::zero()
                } else {
                    *v
                },
                x.1,
            )
        } else {
            *x
        }
    }
}
