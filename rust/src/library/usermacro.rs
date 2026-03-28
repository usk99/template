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
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

/// 最大値更新
#[macro_export]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}
