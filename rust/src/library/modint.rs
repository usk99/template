use std::{
    fmt,
    iter::{Product, Sum},
    ops::*,
    sync::OnceLock,
};

// ===== MOD 切替 (どちらか一方を有効にする) =====
pub const MOD: usize = 998_244_353;
// pub const MOD: usize = 1_000_000_007;

/// MOD を法とする整数。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Mint(usize);

impl Mint {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

    /// 値をMODで丸めて構築
    pub const fn new(x: usize) -> Self {
        Self(x % MOD)
    }

    /// 内部値を取り出す
    pub const fn val(self) -> usize {
        self.0
    }

    /// 繰り返し二乗法による a^n (O(log n))
    pub fn pow(self, mut n: u64) -> Self {
        let mut base = self;
        let mut result = Self::ONE;
        while n > 0 {
            if n & 1 == 1 {
                result *= base;
            }
            base *= base;
            n >>= 1;
        }
        result
    }

    /// 逆元 (フェルマーの小定理。MOD が素数のとき有効。0 に対しては未定義)
    pub fn inv(self) -> Self {
        debug_assert!(self.0 != 0, "Mint::inv() on zero");
        self.pow((MOD - 2) as u64)
    }
}

impl fmt::Display for Mint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for Mint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! impl_from_signed {($($t:ty),*) => { $(impl From<$t> for Mint {fn from(x: $t) -> Mint {Mint((x as i64).rem_euclid(MOD as i64) as usize)}})* };}
impl_from_signed!(i8, i16, i32, i64, isize);
macro_rules! impl_from_unsigned {($($t:ty),*) => { $(impl From<$t> for Mint {fn from(x: $t) -> Mint {Mint(x as usize % MOD)}})*};}
impl_from_unsigned!(u8, u16, u32, u64, usize);

macro_rules! impl_op {
    ($Op:ident, $f:ident, $OpA:ident, $fa:ident, |$a:ident, $b:ident| $body:expr) => {
        #[allow(clippy::suspicious_arithmetic_impl, clippy::suspicious_op_assign_impl)]
        impl $Op for Mint {
            type Output = Mint;
            fn $f(self, r: Mint) -> Mint {
                let $a = self;
                let $b = r;
                $body
            }
        }
        #[allow(clippy::suspicious_arithmetic_impl, clippy::suspicious_op_assign_impl)]
        impl $OpA for Mint {
            fn $fa(&mut self, r: Mint) {
                *self = $Op::$f(*self, r);
            }
        }
    };
}
impl_op!(Add, add, AddAssign, add_assign, |a, b| Mint(
    if a.0 + b.0 >= MOD {
        a.0 + b.0 - MOD
    } else {
        a.0 + b.0
    }
));
impl_op!(Sub, sub, SubAssign, sub_assign, |a, b| Mint(
    if a.0 >= b.0 {
        a.0 - b.0
    } else {
        a.0 + MOD - b.0
    }
));
impl_op!(Mul, mul, MulAssign, mul_assign, |a, b| Mint(
    a.0 * b.0 % MOD
));
impl_op!(Div, div, DivAssign, div_assign, |a, b| a * b.inv());

impl Neg for Mint {
    type Output = Self;
    fn neg(self) -> Self {
        if self.0 == 0 {
            Self(0)
        } else {
            Self(MOD - self.0)
        }
    }
}

macro_rules! impl_scalar_ops {
    ($($t:ty),*) => { $(
        #[allow(clippy::suspicious_arithmetic_impl, clippy::suspicious_op_assign_impl)]
        const _: () = {
            impl Add<$t> for Mint { type Output = Mint; fn add(self, r: $t) -> Mint { self + Mint::from(r) } }
            impl Sub<$t> for Mint { type Output = Mint; fn sub(self, r: $t) -> Mint { self - Mint::from(r) } }
            impl Mul<$t> for Mint { type Output = Mint; fn mul(self, r: $t) -> Mint { self * Mint::from(r) } }
            impl Div<$t> for Mint { type Output = Mint; fn div(self, r: $t) -> Mint { self / Mint::from(r) } }
            impl Add<Mint> for $t { type Output = Mint; fn add(self, r: Mint) -> Mint { Mint::from(self) + r } }
            impl Sub<Mint> for $t { type Output = Mint; fn sub(self, r: Mint) -> Mint { Mint::from(self) - r } }
            impl Mul<Mint> for $t { type Output = Mint; fn mul(self, r: Mint) -> Mint { Mint::from(self) * r } }
            impl Div<Mint> for $t { type Output = Mint; fn div(self, r: Mint) -> Mint { Mint::from(self) / r } }
            impl AddAssign<$t> for Mint { fn add_assign(&mut self, r: $t) { *self += Mint::from(r); } }
            impl SubAssign<$t> for Mint { fn sub_assign(&mut self, r: $t) { *self -= Mint::from(r); } }
            impl MulAssign<$t> for Mint { fn mul_assign(&mut self, r: $t) { *self *= Mint::from(r); } }
            impl DivAssign<$t> for Mint { fn div_assign(&mut self, r: $t) { *self /= Mint::from(r); } }
        };
    )* };
}
impl_scalar_ops!(i32, i64, isize, u32, u64, usize);

impl Sum for Mint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}
impl<'a> Sum<&'a Self> for Mint {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
impl Product for Mint {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}
impl<'a> Product<&'a Self> for Mint {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

// ===== 二項係数 (Combination) =====
// 2_000_001 まで対応。初回呼び出し時に前計算 (O(n))。
const COMB_MAX: usize = 2_000_001;
pub static FACT: OnceLock<Vec<Mint>> = OnceLock::new();
pub static FACT_INV: OnceLock<Vec<Mint>> = OnceLock::new();

fn fact() -> &'static Vec<Mint> {
    FACT.get_or_init(|| {
        let mut f = vec![Mint::ONE; COMB_MAX];
        for i in 1..COMB_MAX {
            f[i] = f[i - 1] * i;
        }
        f
    })
}
fn fact_inv() -> &'static Vec<Mint> {
    FACT_INV.get_or_init(|| {
        let f = fact();
        let mut fi = vec![Mint::ONE; COMB_MAX];
        fi[COMB_MAX - 1] = f[COMB_MAX - 1].inv();
        for i in (1..COMB_MAX).rev() {
            fi[i - 1] = fi[i] * i;
        }
        fi
    })
}

/// nCr (n < r のときは 0)
pub fn ncr(n: usize, r: usize) -> Mint {
    if n < r {
        return Mint::ZERO;
    }
    fact()[n] * fact_inv()[r] * fact_inv()[n - r]
}
/// 巨大 n 用の nCr (r が小さいケース、計算量 O(r))。
/// 例: n = 10^18, r ≤ 10^4。r は COMB_MAX 未満であること。
pub fn ncr_large(n: u64, r: usize) -> Mint {
    if (n as usize) < r {
        return Mint::ZERO;
    }
    let mut num = Mint::ONE;
    for i in 0..r {
        num *= n - i as u64;
    }
    num * fact_inv()[r]
}
/// nPr (n < r のときは 0)
pub fn npr(n: usize, r: usize) -> Mint {
    if n < r {
        return Mint::ZERO;
    }
    fact()[n] * fact_inv()[n - r]
}
/// 重複組合せ H(n, r) = C(n + r - 1, r)
pub fn nhr(n: usize, r: usize) -> Mint {
    if n == 0 {
        return if r == 0 { Mint::ONE } else { Mint::ZERO };
    }
    ncr(n + r - 1, r)
}
