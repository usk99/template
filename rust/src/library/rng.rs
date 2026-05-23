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
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
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
        self.f64() < prob
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
