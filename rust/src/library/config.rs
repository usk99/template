#![allow(non_snake_case)]

/// 実行時間
pub const TIME_LIMIT: f64 = 0000.0;
/// 乱数生成用のシード
pub const RNG_SEED: u64 = 123_456_789;
/// アルゴリズム
pub const ENABLE_ALGORITHM: bool = true;
/// 貪欲
pub const ENABLE_GREEDY: bool = false;
/// 焼きなまし
pub const ENABLE_SA: bool = false;
/// ビームサーチ
pub const ENABLE_BEAM: bool = false;

/// 設定用
pub struct Config {
    pub islocal: bool,
}
/// 初回アクセス時に自動的に new 内のクロージャが実行され、値が固定されます
pub static CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| {
    // 1. "ATCODER" 環境変数がない
    let is_env_local = std::env::var("ATCODER").is_err();

    // 2. 実行パスに "target" が含まれる (cargo run で実行している)
    let is_debug_build = cfg!(debug_assertions);

    // リリースビルドなら、ローカルじゃない判定
    let islocal = is_env_local && is_debug_build;

    // // リリースでもローカル判定
    // let islocal = is_env_local || is_debug_build;

    Config { islocal }
});
/// ローカル環境かどうか
#[inline(always)]
pub fn is_local() -> bool {
    CONFIG.islocal
}

/// 実行場所付きの出力（トレース）
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        if $crate::library::config::is_local() {
            eprint!("[{}:{:4}:{:3}] ", file!(), line!(), column!());
            eprintln!($($arg)*);
        }
    };
}

/// - ブロックの実行時間を計測する (Local Only)
/// - 使用例: measure!("Sorting", { list.sort(); });
#[macro_export]
macro_rules! measure {
    ($label:expr, $block:block) => {
        if $crate::library::config::is_local() {
            let start = std::time::Instant::now();
            $block
            eprintln!("[MEASURE] {}: {}ms", $label, start.elapsed().as_millis());
        } else {
            $block
        }
    };
}
