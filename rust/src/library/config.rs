use std::{env::*, sync::*};

/// 実行時間
pub const TIME_LIMIT: f64 = 0000.0;
/// XORシフトのシード
pub const RND_SEED: u64 = 123456789;
/// アルゴリズム
pub const ENABLE_ALGORITHM: bool = true;
/// 貪欲
pub const ENABLE_GREEDY: bool = false;
/// 焼きなまし
pub const ENABLE_SA: bool = false;
/// ビームサーチ
pub const ENABLE_BEAM: bool = false;

pub struct Config {
    pub islocal: bool,
}

/// 初回アクセス時に自動的に new 内のクロージャが実行され、値が固定されます
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    // 1. "ATCODER" 環境変数がない
    let is_env_local = var("ATCODER").is_err();

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
