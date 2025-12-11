// 乱数
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
    #[inline]
    const fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// ランダム選択[0, max)
    #[inline]
    pub const fn gen_range(&mut self, max: usize) -> usize {
        ((self.next() as u128 * max as u128) >> 64) as usize
    }

    /// 範囲内[min, max)
    #[inline]
    pub const fn next_range(&mut self, min: usize, max: usize) -> usize {
        min + self.gen_range(max - min)
    }

    /// 0.0<=x<1.0で乱数
    pub const fn next_f64(&mut self) -> f64 {
        const INVERSE_53: f64 = 1.0 / 9007199254740992.0;
        (self.next() >> 11) as f64 * INVERSE_53
    }

    /// 指定確率でtrueを返す
    #[inline]
    pub const fn next_bool(&mut self, prob: f64) -> bool {
        self.next() < (prob * 18446744073709551616.0) as u64
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
