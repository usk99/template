// マクロ
#[macro_export]
macro_rules! min {
        ($a:expr $(,)*)=>{{$a}};
        ($a:expr,$b:expr $(,)*)=>{{std::cmp::min($a, $b)}};
        ($a:expr,$($rest:expr),+ $(,)*)=>{{std::cmp::min($a, min!($($rest),+))}};
    }
#[macro_export]
macro_rules! max {
        ($a:expr $(,)*)=>{{$a}};
        ($a:expr,$b:expr $(,)*)=>{{std::cmp::max($a, $b)}};
        ($a:expr,$($rest:expr),+ $(,)*)=>{{std::cmp::max($a, max!($($rest),+))}};
    }
#[macro_export]
macro_rules! chmin {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_min = min!($($cmps),+);
            if $base > cmp_min {$base = cmp_min;true}else{false}}};
    }
#[macro_export]
macro_rules! chmax {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_max = max!($($cmps),+);
            if $base < cmp_max {$base = cmp_max;true} else {false}}};
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
