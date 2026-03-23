#![allow(non_snake_case)]
use {
    super::fmt::*,
    std::{env::*, sync::*, time::*},
};

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

/// 設定用
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

/// 乱数
pub struct XorShift {
    state: u64,
}
impl XorShift {
    /// 初期化
    pub const fn new(seed: u64) -> Self {
        let mut seed = seed;
        if seed == 0 {
            seed = 88172645463325252;
        }
        Self { state: seed }
    }

    /// 乱数生成
    #[inline(always)]
    const fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// ランダム選択[0, max)
    #[inline(always)]
    pub const fn gen_range(&mut self, max: usize) -> usize {
        ((self.next() as u128 * max as u128) >> 64) as usize
    }

    /// 範囲内[min, max)
    #[inline(always)]
    pub const fn next_range(&mut self, min: usize, max: usize) -> usize {
        min + self.gen_range(max - min)
    }

    /// 0.0<=x<1.0で乱数
    pub const fn next_f64(&mut self) -> f64 {
        const INV_2_53: f64 = 1.0 / (1u64 << 53) as f64;
        (self.next() >> 11) as f64 * INV_2_53
    }

    /// 指定確率でtrueを返す
    #[inline(always)]
    pub const fn next_bool(&mut self, prob: f64) -> bool {
        self.next() < (prob * 18446744073709551616.0) as u64 // 2^64
    }

    /// ランダム選択
    #[inline]
    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.gen_range(items.len())]
    }

    /// シャッフル
    #[inline]
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            items.swap(i, self.next_range(0, i + 1));
        }
    }
}

/// タイマー
pub struct Timer {
    start_time: Instant,
    time_limit: f64, // 制限時間(ms)
}
impl Timer {
    /// 計測開始
    #[inline]
    pub fn new(time_limit: f64) -> Self {
        Self {
            start_time: Instant::now(),
            time_limit,
        }
    }

    /// 時間取得
    #[inline]
    pub fn get_time(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64() * 1000.0
    }

    /// 時間取得(カンマ区切り文字列)
    #[inline]
    pub fn get_time_str(&self) -> String {
        format!("{}ms", (self.get_time() as usize).comma())
    }

    /// 進捗
    #[inline]
    pub fn progress(&self) -> f64 {
        let p = self.get_time() / self.time_limit;
        p.clamp(0.0, 1.0)
    }

    /// 残り時間(ms)
    #[inline]
    pub fn remaining(&self) -> f64 {
        (self.time_limit - self.get_time()).max(0.0)
    }

    /// 時間確認
    #[inline]
    pub fn is_timeout(&self) -> bool {
        self.get_time() >= self.time_limit
    }

    /// 再起動
    #[inline]
    pub fn reset(&mut self) {
        self.start_time = Instant::now()
    }

    /// 制限時間セット
    #[inline]
    pub const fn set_limit(&mut self, timelimit: f64) {
        self.time_limit = timelimit;
    }

    /// 温度管理(焼きなまし用)
    #[inline]
    pub fn get_temp(&self, start_temp: f64, end_temp: f64) -> f64 {
        (end_temp - start_temp).mul_add(self.progress(), start_temp)
    }

    /// 指数温度管理
    #[inline]
    pub fn get_temp_exp(&self, start_temp: f64, end_temp: f64) -> f64 {
        start_temp * (end_temp / start_temp).powf(self.progress())
    }
}
