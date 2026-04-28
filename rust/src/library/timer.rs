use std::time::Instant;

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
    #[inline(always)]
    pub fn get_time(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64() * 1000.0
    }

    /// 時間取得(カンマ区切り文字列)
    #[inline]
    pub fn get_time_str(&self) -> String {
        format!("{}ms", comma(self.get_time() as usize))
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
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        self.get_time() >= self.time_limit
    }

    /// 再起動
    #[inline]
    pub fn reset(&mut self) {
        self.start_time = Instant::now()
    }

    /// 制限時間セット
    #[inline(always)]
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

/// 数値を3桁区切り文字列に変換 (timer内部用)
fn comma(mut n: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut buf = [0u8; 50];
    let mut p = 50;
    let mut c = 0;
    while n > 0 {
        if c > 0 && c % 3 == 0 {
            p -= 1;
            buf[p] = b',';
        }
        p -= 1;
        buf[p] = b'0' + (n % 10) as u8;
        n /= 10;
        c += 1;
    }
    unsafe { String::from_utf8_unchecked(buf[p..].to_vec()) }
}
