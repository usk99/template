// マクロ

/// 複数の値の最小値
#[macro_export]
macro_rules! min {
    ($a:expr $(,)*) => {{ $a }};
    ($a:expr,$b:expr $(,)*) => {{ std::cmp::min($a, $b) }};
    ($a:expr,$($rest:expr),+ $(,)*) => {{ std::cmp::min($a, min!($($rest),+)) }};
}

/// 複数の値の最大値
#[macro_export]
macro_rules! max {
    ($a:expr $(,)*) => {{ $a }};
    ($a:expr,$b:expr $(,)*) => {{ std::cmp::max($a, $b) }};
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

/// シンプルなログ出力
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if is_local() {
            eprintln!($($arg)*);
        }
    };
}

/// 実行場所付きの出力（トレース）
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        if is_local() {
            eprint!("[{}:{:4}:{:3}] ", file!(), line!(), column!());
            eprintln!($($arg)*);
        }
    };
}

/// 2次元配列 (Vec<Vec<T>>) を作成するマクロ
/// 使用例: let grid = grid!\[0; h; w\];
#[macro_export]
macro_rules! grid {
    ($val:expr; $r:expr; $c:expr) => {
        vec![vec![$val; $c]; $r]
    };
}

/// ブロックの実行時間を計測する (Local Only)
/// 使用例: measure!("Sorting", { list.sort(); });
#[macro_export]
macro_rules! measure {
    ($label:expr, $block:block) => {
        if is_local() {
            let start = std::time::Instant::now();
            $block
            let elapsed = start.elapsed();
            eprintln!("[MEASURE] {}: {}ms", $label, elapsed.as_millis());
        } else {
            $block
        }
    };
}
