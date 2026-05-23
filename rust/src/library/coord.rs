// 座標系
use {
    super::io::*,
    std::{cmp::*, fmt::*, ops::*},
};

/// グリッド座標の内部表現型。wrapping演算で範囲外チェックを行うため符号なし整数を使う。
/// -  u8:<255
/// - u16:<65,535
/// - u32:<4,294,967,295
/// - 境界値ちょうどの座標はwrapping_add等で範囲外に出る可能性があるため注意
/// - その場合一つ上の型にすると安全
pub type CoordType = u32;

/// グリッド上の4方向（上下左右）を表す列挙型。
/// `Coord` に加算することで1マス移動できる。インデックス値は 0〜3。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    /// 上(Up)   : row - 1
    U = 0,
    /// 右(Right): col + 1
    R = 1,
    /// 下(Down) : row + 1
    D = 2,
    /// 左(Left) : col - 1
    L = 3,
}
crate::impl_elem!(Direction);
impl Direction {
    /// 全4方向の定数配列。`for &dir in Direction::ALL.iter()` で全方向ループできる。
    pub const ALL: [Self; 4] = [Self::U, Self::R, Self::D, Self::L];

    /// インデックス `i % 4` に対応する方向を返す（0=U, 1=R, 2=D, 3=L）。
    pub const fn new(i: usize) -> Self {
        match i % 4 {
            0 => Self::U,
            1 => Self::R,
            2 => Self::D,
            3 => Self::L,
            _ => unreachable!(),
        }
    }

    /// 全4方向のイテレータを返す。`neighbors4` の内部でも使用している。
    pub fn iter() -> std::slice::Iter<'static, Self> {
        Self::ALL.iter()
    }

    /// 方向に対応する座標差分 `(dr, dc)` を CoordType の wrapping 値で返す。
    /// U=(!0,0), R=(0,1), D=(1,0), L=(0,!0)。
    /// !0 は CoordType::MAX であり、wrapping_add すると -1 の移動になる。
    #[inline]
    pub const fn delta(self) -> (CoordType, CoordType) {
        match self {
            Self::U => (!0, 0),
            Self::R => (0, 1),
            Self::D => (1, 0),
            Self::L => (0, !0),
        }
    }

    /// isize の差分 `(dr, dc)` から対応する方向を返す。
    /// 斜め・ゼロなど4方向以外の差分は `None`。
    #[inline]
    pub const fn from_delta(delta: (isize, isize)) -> Option<Self> {
        match delta {
            (-1, 0) => Some(Self::U),
            (0, 1) => Some(Self::R),
            (1, 0) => Some(Self::D),
            (0, -1) => Some(Self::L),
            _ => None,
        }
    }

    /// 反対方向を返す（U↔D、R↔L）。
    pub const fn opposite(self) -> Self {
        match self {
            Self::U => Self::D,
            Self::R => Self::L,
            Self::D => Self::U,
            Self::L => Self::R,
        }
    }

    /// 時計回りに90度回転した方向を返す（U→R→D→L→U）。
    pub const fn clockwise(self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    /// 反時計回りに90度回転した方向を返す（U→L→D→R→U）。
    pub const fn counter_clockwise(self) -> Self {
        match self {
            Self::U => Self::L,
            Self::R => Self::U,
            Self::D => Self::R,
            Self::L => Self::D,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// グリッド上の2次元座標（行 r, 列 c）を表す構造体。
/// 加減算は wrapping 演算で行われるため、グリッド境界チェックは `within` で行う。
/// `Direction` との加算（`coord + dir`）で隣接マスへ移動できる。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub r: CoordType,
    pub c: CoordType,
}
crate::impl_elem!(Coord);
impl Coord {
    /// 原点 (0, 0)。
    pub const ORIGIN: Self = Self { r: 0, c: 0 };

    /// `(row, col)` から座標を生成する。
    /// row, col は `CoordType::MAX` 未満であること（MAX だと wrapping 判定が破綻する）。
    #[inline]
    pub fn new(row: usize, col: usize) -> Self {
        debug_assert!(
            row < CoordType::MAX as usize && col < CoordType::MAX as usize,
            "Coord({}, {}) overflows {} (MAX={}). Wrapping logic requires strict inequality.",
            row,
            col,
            std::any::type_name::<CoordType>(),
            CoordType::MAX
        );
        Self {
            r: row as CoordType,
            c: col as CoordType,
        }
    }

    /// 行インデックス（r フィールド）を usize で返す。
    #[inline(always)]
    pub const fn row(&self) -> usize {
        self.r as usize
    }

    /// 列インデックス（c フィールド）を usize で返す。
    #[inline(always)]
    pub const fn col(&self) -> usize {
        self.c as usize
    }

    /// `rows × cols` のグリッド内に座標が収まっているか判定する。
    /// wrapping 演算で範囲外に出た座標（大きな値になる）も正しく弾く。
    #[inline(always)]
    pub const fn within(&self, rows: usize, cols: usize) -> bool {
        // 負の値は来ないようになっている
        self.row() < rows && self.col() < cols
    }

    /// 2次元座標 `(r, c)` を1次元インデックス `r * cols + c` に変換する。
    /// `Vec` や配列の flat な表現へのアクセスに使う。
    #[inline(always)]
    pub const fn to_index(self, cols: usize) -> usize {
        self.row() * cols + self.col()
    }

    /// 1次元インデックスから2次元座標へ変換する（`to_index` の逆）。
    #[inline(always)]
    pub fn from_index(idx: usize, cols: usize) -> Self {
        Self::new(idx / cols, idx % cols)
    }

    /// `rows × cols` グリッドの全座標を行優先順に列挙するイテレータを返す。
    #[inline]
    pub fn all(rows: usize, cols: usize) -> impl Iterator<Item = Self> {
        (0..rows).flat_map(move |r| (0..cols).map(move |c| Self::new(r, c)))
    }

    /// 2座標間の行・列の絶対差分を `Coord` として返す（符号なし）。
    /// 距離計算の内部処理に使う。
    #[inline]
    pub const fn diff(&self, other: &Self) -> Self {
        Self {
            r: self.r.abs_diff(other.r),
            c: self.c.abs_diff(other.c),
        }
    }

    /// 2座標を対角とする長方形の面積（|dr| * |dc|）を返す。
    #[inline]
    pub const fn rect_area(&self, other: &Self) -> usize {
        let d = self.diff(other);
        (d.r as usize) * (d.c as usize)
    }

    /// ユークリッド距離の **2乗** を返す（`sqrt` なし）。
    /// 距離の大小比較には2乗のまま使えるため、浮動小数点誤差を避けられる。
    #[inline]
    pub const fn dist_euclidean2(&self, other: &Self) -> usize {
        let d = self.diff(other);
        let r = d.row();
        let c = d.col();
        r * r + c * c
    }

    /// マンハッタン距離（|dr| + |dc|）を返す。
    /// 上下左右のみ移動できるグリッドでの最短手数に相当する。
    #[inline]
    pub const fn dist_manhattan(&self, other: &Self) -> usize {
        let d = self.diff(other);
        d.row() + d.col()
    }

    /// チェビシェフ距離（max(|dr|, |dc|)）を返す。
    /// 8方向移動できるグリッドでの最短手数に相当する。
    #[inline]
    pub const fn dist_chebyshev(&self, other: &Self) -> usize {
        let d = self.diff(other);
        if d.row() > d.col() { d.row() } else { d.col() }
    }

    /// 上下左右4方向の隣接座標のうち、グリッド内に収まるものだけを返すイテレータ。
    /// BFS・DFS のグリッド探索で典型的に使う。
    #[inline]
    pub fn neighbors4(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> + '_ {
        Direction::iter().filter_map(move |&dir| {
            let next = *self + dir;
            if next.within(rows, cols) {
                Some(next)
            } else {
                None
            }
        })
    }

    /// 斜めを含む8方向の隣接座標のうち、グリッド内に収まるものだけを返すイテレータ。
    #[inline]
    pub fn neighbors8(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> + '_ {
        const NEIGHBOR_8: [(CoordType, CoordType); 8] = [
            (!0, 0),
            (0, 1),
            (1, 0),
            (0, !0),
            (!0, 1),
            (1, 1),
            (1, !0),
            (!0, !0),
        ];
        NEIGHBOR_8.iter().filter_map(move |&(dr, dc)| {
            let next = Self {
                r: self.r.wrapping_add(dr as CoordType),
                c: self.c.wrapping_add(dc as CoordType),
            };
            if next.within(rows, cols) {
                Some(next)
            } else {
                None
            }
        })
    }

    /// `self` から `other` への方向を返す。
    /// 隣接していない場合や斜め方向の場合は `None`。
    #[inline]
    pub const fn direction_to(&self, other: &Self) -> Option<Direction> {
        let row_diff = other.r as isize - self.r as isize;
        let col_diff = other.c as isize - self.c as isize;
        match (row_diff.signum(), col_diff.signum()) {
            (-1, 0) => Some(Direction::U),
            (1, 0) => Some(Direction::D),
            (0, -1) => Some(Direction::L),
            (0, 1) => Some(Direction::R),
            _ => None,
        }
    }
}

/// `coord + coord` で行・列をそれぞれ加算する（wrapping）。
impl Add for Coord {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.wrapping_add(rhs.r),
            c: self.c.wrapping_add(rhs.c),
        }
    }
}

/// `coord += coord` の複合代入。
impl AddAssign for Coord {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// `coord - coord` で行・列をそれぞれ減算する（wrapping）。
impl Sub for Coord {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.wrapping_sub(rhs.r),
            c: self.c.wrapping_sub(rhs.c),
        }
    }
}

/// `coord -= coord` の複合代入。
impl SubAssign for Coord {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<(usize, usize)> for Coord {
    #[inline]
    fn from((row, col): (usize, usize)) -> Self {
        Self::new(row, col)
    }
}

/// `Output(coord)` で `"r c"` の形式（空白区切り）で出力される。
impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.r, self.c)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Coord({}, {})", self.r, self.c)
    }
}

/// `coord + direction` で方向に1マス移動した座標を返す（wrapping）。
/// グリッド外に出た場合は `within` で弾く。
impl Add<Direction> for Coord {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Direction) -> Self::Output {
        let (dr, dc) = rhs.delta();
        Self {
            r: self.r.wrapping_add(dr as CoordType),
            c: self.c.wrapping_add(dc as CoordType),
        }
    }
}

/// `coord += direction` の複合代入。
impl AddAssign<Direction> for Coord {
    #[inline]
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

/// `direction * n` で方向に n マス分の差分ベクトル（Coord）を返す。
impl Mul<CoordType> for Direction {
    type Output = Coord;
    #[inline]
    fn mul(self, rhs: CoordType) -> Coord {
        let (dr, dc) = self.delta();
        Coord {
            r: dr.wrapping_mul(rhs),
            c: dc.wrapping_mul(rhs),
        }
    }
}

/// `coord * n` で行・列をそれぞれ n 倍にスケールする（wrapping）。
impl Mul<CoordType> for Coord {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: CoordType) -> Self {
        Self {
            r: self.r.wrapping_mul(rhs),
            c: self.c.wrapping_mul(rhs),
        }
    }
}
