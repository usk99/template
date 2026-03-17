use super::seg::*;
use num::*;

/// モノイド
pub trait Monoid {
    type S: Clone + Copy + Default;
    /// 単位元
    fn identity() -> Self::S;
    /// 演算
    fn binary_operation(a: Self::S, b: Self::S) -> Self::S;
}

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
    MaxIdx, SegMaxIdx, (usize, usize), (0, usize::MAX),
    |a, b| if a.0 > b.0 || (a.0 == b.0 && a.1 <= b.1) { a } else { b };
    MinIdx, SegMinIdx, (usize, usize), (usize::MAX, usize::MAX),
    |a, b| if a.0 < b.0 || (a.0 == b.0 && a.1 <= b.1) { a } else { b };
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

macro_rules! impl_idx_methods {
    ($seg_type:ty) => {
        impl $seg_type {
            pub fn from_values(values: Vec<usize>) -> Self {
                Self::from(
                    values
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| (v, i))
                        .collect::<Vec<_>>(),
                )
            }
            pub fn set_value(&mut self, p: usize, val: usize) {
                self.set(p, (val, p));
            }
            pub fn get_value(&self, p: usize) -> usize {
                self[p].0
            }
        }
    };
}

impl_idx_methods!(SegMaxIdx);
impl_idx_methods!(SegMinIdx);
