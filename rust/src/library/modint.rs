// ===== MOD 切替 (どちらか一方を有効にする) =====
pub const MOD: usize = 998_244_353;
// pub const MOD: usize = 1_000_000_007;

/// MOD を法とする整数。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Mint(usize);
