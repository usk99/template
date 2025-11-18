#![allow(non_snake_case)]
// タイマー
use super::util::*;
use std::time::Instant;
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
        format!("{}ms", comma(self.get_time() as usize))
    }

    /// 時間超過確認
    #[inline]
    pub fn is_time_over(&self) -> bool {
        self.get_time() >= self.time_limit
    }

    /// 進捗
    #[inline]
    pub fn progress(&self) -> f64 {
        let p = self.get_time() / self.time_limit;
        p.min(1.0)
    }

    ///
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
    pub fn set_limit(&mut self, timelimit: f64) {
        self.time_limit = timelimit;
    }

    /// 温度管理(焼きなまし用)
    #[inline]
    pub fn get_temp(&self, start_temp: f64, end_temp: f64) -> f64 {
        start_temp + (end_temp - start_temp) * self.progress()
    }

    /// 指数温度管理
    #[inline]
    pub fn get_temp_exp(&self, start_temp: f64, end_temp: f64) -> f64 {
        start_temp * (end_temp / start_temp).powf(self.progress())
    }
}
