// マクロ

/// 複数の値の最小値
#[macro_export]
macro_rules! min {
    ($a:expr $(,)*) => {{ $a }};
    ($a:expr,$($rest:expr),+ $(,)*) => {{ std::cmp::min($a, min!($($rest),+)) }};
}

/// 複数の値の最大値
#[macro_export]
macro_rules! max {
    ($a:expr $(,)*) => {{ $a }};
    ($a:expr,$($rest:expr),+ $(,)*) => {{ std::cmp::max($a, max!($($rest),+)) }};
}

/// 最小値更新
#[macro_export]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        ($base > cmp_min).then(|| $base = cmp_min).is_some()
    }};
}

/// 最大値更新
#[macro_export]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        ($base < cmp_max).then(|| $base = cmp_max).is_some()
    }};
}

/// 二次元vec宣言
macro_rules! vv {
    ($e:expr; $n:expr, $m:expr) => {
        vec![vec![$e; $m]; $n]
    };
}
