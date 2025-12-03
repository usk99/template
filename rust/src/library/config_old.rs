use std::{env::*, sync::*};
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
