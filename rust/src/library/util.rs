// 便利

pub const ABC: &[char; 52] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub const ABC_U: &[char] = ABC.split_at(26).0;
pub const ABC_L: &[char] = ABC.split_at(26).1;

// Hash関数早い版
pub type Set<V> = rustc_hash::FxHashSet<V>;
pub type Map<K, V> = rustc_hash::FxHashMap<K, V>;

pub trait FxNew: Sized {
    fn new() -> Self;
    fn with_capacity(n: usize) -> Self;
}
impl<V> FxNew for Set<V> {
    fn new() -> Self {
        Self::default()
    }
    fn with_capacity(n: usize) -> Self {
        Self::with_capacity_and_hasher(n, Default::default())
    }
}
impl<K, V> FxNew for Map<K, V> {
    fn new() -> Self {
        Self::default()
    }
    fn with_capacity(n: usize) -> Self {
        Self::with_capacity_and_hasher(n, Default::default())
    }
}
