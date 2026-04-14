#![allow(non_snake_case)]
use {
    super::fmt::*,
    std::{env::*, sync::*, time::*},
};

/// 実行時間
pub const TIME_LIMIT: f64 = 0000.0;
/// 乱数生成用のシード
pub const RNG_SEED: u64 = 123456789;
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
pub struct FastRng {
    state: u64,
}
impl FastRng {
    /// 初期化（シード0でも問題なし）
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// [raw] 64bit整数をそのまま生成
    #[inline(always)]
    pub const fn u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    /// [0, max) の範囲の整数を選択
    #[inline(always)]
    pub const fn range(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        ((self.u64() as u128 * max as u128) >> 64) as usize
    }

    /// [min, max) の範囲の整数を選択
    #[inline(always)]
    pub const fn range_in(&mut self, min: usize, max: usize) -> usize {
        min + self.range(max - min)
    }

    /// [0.0, 1.0) の浮動小数点を生成
    #[inline(always)]
    pub const fn f64(&mut self) -> f64 {
        const INV_2_53: f64 = 1.0 / (1u64 << 53) as f64;
        (self.u64() >> 11) as f64 * INV_2_53
    }

    /// 確率 prob (0.0~1.0) で true を返す
    #[inline(always)]
    pub const fn bool(&mut self, prob: f64) -> bool {
        self.u64() < (prob * 18446744073709551616.0) as u64
    }

    /// 配列からランダムに要素を参照
    #[inline]
    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.range(items.len())]
    }

    /// 配列をシャッフル
    #[inline]
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        let len = items.len();
        if len <= 1 {
            return;
        }
        for i in (1..len).rev() {
            let j = self.range(i + 1);
            items.swap(i, j);
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

/// シンプルなログ出力
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if $crate::library::config::is_local() {
            eprintln!($($arg)*);
        }
    };
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
