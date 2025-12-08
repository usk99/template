use std::{env::*, sync::*};

/// 実行時間
pub const TIME_LIMIT: f64 = 0000.0;
/// 実行タイプ
pub const EXE_TYPE: u8 = 0b00000000;
/// XORシフトのシード
pub const RND_SEED: u64 = 123456789;

pub struct Config {
    islocal: bool,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
impl Config {
    pub fn init() {
        // local判定・・・pahcerや提出時はfalse
        let islocal = args().map(|x| x.contains("target")).next().unwrap()
            & var("ATCODER").and(Ok(false)).unwrap_or(true);
        CONFIG.get_or_init(|| Self { islocal });
    }
}
pub fn is_local() -> bool {
    CONFIG.get().unwrap().islocal
}
